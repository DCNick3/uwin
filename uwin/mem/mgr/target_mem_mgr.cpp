//
// Created by dcnick3 on 10/25/20.
//

#include "target_mem_mgr.h"
#include "util/align.h"
#include "mem/except.h"
#include "win32/error.h"

#include <utility>
#include <memory>


namespace uwin::mem::mgr {

    target_mem_mgr::target_mem_mgr(std::shared_ptr<base_mem_mapper> mapper) :
            _mapper(std::move(mapper)),
            _host_region(_mapper->host_reserve(consts::address_space_size)) {
        assert(_mapper->page_size() == consts::page_size);
    }


    pages_regions_container::iterator target_mem_mgr::find_starting_with_win32(taddr start_addr) {
        auto exc = win32::error(win32::error_code::ERROR_INVALID_ADDRESS,
                                "specified pages region was not found.");
        if (!util::is_aligned(start_addr, consts::allocation_granularity))
            throw std::move(exc);
        auto it = _regions_container.find_starting_with(start_addr);
        if (it == _regions_container.end())
            throw std::move(exc);
        return it;
    }

    pages_regions_container::iterator target_mem_mgr::find_containing_win32(tmem_region region) {
        auto it = _regions_container.find_containing(region);
        if (it == _regions_container.end())
            throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS,
                               "The specified memory region does not belong to any of the reserved regions.");
        return it;
    }

    tmem_region target_mem_mgr::reserve_dynamic(taddr::tvalue size) {
        return *_regions_container.alloc(util::align_up(size, consts::page_size)); // NOLINT(cppcoreguidelines-slicing)
        // object slicing is intended here
    }

    tmem_region target_mem_mgr::reserve_fixed(tmem_region region) {
        return *_regions_container.insert(region.align_greedy(consts::allocation_granularity,
                                                              consts::page_size)); // NOLINT(cppcoreguidelines-slicing)
    }

    tmem_region target_mem_mgr::unreserve(taddr start_addr) {
        auto it = find_starting_with_win32(start_addr);

        tmem_region res = *it; // NOLINT(cppcoreguidelines-slicing)

        if (it->has_commited_pages(res))
            uncommit(res);
        _regions_container.erase(it);
        return res;
    }

    tmem_region target_mem_mgr::commit(tmem_region region, tprot prot) {
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->commit_pages(region, prot);
        _mapper->host_map_fixed(guest_to_host(region), to_hprot(prot));
        return region;
    }

    tmem_region target_mem_mgr::reprotect(tmem_region region, tprot prot) {
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->reprotect_pages(region, prot);
        _mapper->host_reprotect(guest_to_host(region), to_hprot(prot));
        return region;
    }

    tmem_region target_mem_mgr::uncommit(tmem_region region) {
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->uncommit_pages(region);
        _mapper->host_unmap(guest_to_host(region));
        return region;
    }

    tmem_region target_mem_mgr::uncommit_whole_reserved_region(taddr start_addr) {
        auto it = find_starting_with_win32(start_addr);
        tmem_region region = *it; // NOLINT(cppcoreguidelines-slicing)
        it->uncommit_pages(region);
        _mapper->host_unmap(guest_to_host(region));
        return region;
    }

    hprot target_mem_mgr::to_hprot(tprot prot) {
        switch (prot) {
            case tprot::none:
            case tprot::x:
                return hprot::none;

            case tprot::r:
            case tprot::rx:
                return hprot::r;

            case tprot::w:
            case tprot::rw:
            case tprot::wx:
            case tprot::rwx:
                return hprot::rw;
            default:
                std::terminate();
        }
    }

    target_mem_mgr::~target_mem_mgr() {
        for (auto const &region : _regions_container) {
            if (region.has_commited_pages(region)) // NOLINT(cppcoreguidelines-slicing)
                _mapper->host_unmap(guest_to_host(region));
        }
    }
}
