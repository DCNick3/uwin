//
// Created by dcnick3 on 10/25/20.
//

#include "mem/mgr/target_mem_mgr.h"
#include "mem/mgr/query_results.h"
#include "mem/except.h"
#include "win32/error.h"
#include "log/log.h"
#include "util/align.h"
#include "util/except.h"
#include "util/visit.h"

#include <utility>
#include <memory>


namespace uwin::mem::mgr {

    target_mem_mgr::target_mem_mgr(std::shared_ptr<base_mem_mapper> mapper) :
            _mapper(std::move(mapper)),
            _host_region(_mapper->host_reserve(consts::address_space_size)) {
        if (_mapper->page_size() != consts::page_size)
            throw util::not_implemented_error("Supporting different host and target page size");
        log::debug("Creating target_mem_mgr...");
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

    tmem_region target_mem_mgr::reserve_dynamic_aligned(taddr::tvalue size, taddr::tvalue alignment) {
        log::debug("reserve_dynamic(0x{:x})", size);
        return *_regions_container.alloc(util::align_up(size, consts::page_size), alignment); // NOLINT(cppcoreguidelines-slicing)
        // object slicing is intended here
    }

    tmem_region target_mem_mgr::reserve_fixed(tmem_region region) {
        log::debug("reserve_fixed({})", region);
        return *_regions_container.insert(                  // NOLINT(cppcoreguidelines-slicing)
                region.align_greedy(consts::allocation_granularity,
                                                              consts::page_size));
    }

    tmem_region target_mem_mgr::unreserve(taddr start_addr) {
        log::debug("unreserve({})", start_addr);
        auto it = find_starting_with_win32(start_addr);

        tmem_region res = *it; // NOLINT(cppcoreguidelines-slicing)

        if (it->has_commited_pages(res))
            uncommit(res);
        _regions_container.erase(it);
        return res;
    }

    tmem_region target_mem_mgr::commit(tmem_region region, tprot prot) {
        log::debug("commit({}, {})", region, prot);
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->commit_pages(region, prot);
        _mapper->host_map_fixed(ptr(region), to_hprot(prot));
        return region;
    }

    tmem_region target_mem_mgr::reprotect(tmem_region region, tprot prot) {
        log::debug("reprotect({}, {})", region, prot);
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->reprotect_pages(region, prot);
        _mapper->host_reprotect(ptr(region), to_hprot(prot));
        return region;
    }

    tmem_region target_mem_mgr::uncommit(tmem_region region) {
        log::debug("uncommit({})", region);
        region = region.align_greedy(consts::page_size);
        auto it = find_containing_win32(region);
        it->uncommit_pages(region);
        _mapper->host_unmap(ptr(region));
        return region;
    }

    tmem_region target_mem_mgr::uncommit_whole_reserved_region(taddr start_addr) {
        log::debug("uncommit_whole_reserved_region({})", start_addr);
        auto it = find_starting_with_win32(start_addr);
        tmem_region region = *it; // NOLINT(cppcoreguidelines-slicing)
        it->uncommit_pages(region);
        _mapper->host_unmap(ptr(region));
        return region;
    }

    target_mem_mgr::query_result target_mem_mgr::query(taddr ptr) const {
        log::debug("query({})", ptr);
        ptr = util::align_down(ptr, consts::page_size);
        auto region_var = _regions_container.query(ptr);

        return util::visit(region_var,
                [&](pages_regions_container::query_results::free &free) -> query_result {
                    return query_results::free{{ptr, taddr::tvalue(free.region.end() - ptr)}};
                },
                [&](pages_regions_container::query_results::reserved &reserved) -> query_result {
                    auto r = reserved.region_it->query(ptr);
                    return util::visit(r,
                        [&](query_results::reserved &reserved) -> query_result {
                            return reserved;
                        },
                        [&](query_results::committed &committed) -> query_result {
                            return committed;
                        });
                }
        );
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
                _mapper->host_unmap(ptr(region));
        }
        _mapper->host_unreserve(_host_region);
    }

    std::string target_mem_mgr::dump_memory_map() const {
        std::string map;
        taddr p(0);
        while (true) {
            auto res_var = query(p);
            util::visit(res_var, [&](query_results::committed& committed) {
                auto& rg = committed.region;
                map += fmt::format("C {:#010x}-{:#010x} ({:#010x}) {}\n",
                                   rg.begin().value(), rg.end().value(), rg.size(), committed.protection);
                p = rg.end();
            }, [&](query_results::reserved& reserved) {
                auto& rg = reserved.region;
                map += fmt::format("R {:#010x}-{:#010x} ({:#010x})\n",
                                   rg.begin().value(), rg.end().value(), rg.size());
                p = rg.end();
            }, [&](query_results::free& free) {
                auto& rg = free.region;
                map += fmt::format("F {:#010x}-{:#010x} ({:#010x})\n", rg.begin().value(), rg.end().value(), rg.size());
                p = rg.end();
            });
            if (p.value() == 0)
                break;
        }
        if (!map.empty())
            map.erase(map.size() - 1);
        return map;
    }

    void *target_mem_mgr::ptr_raw(taddr::tvalue addr) const {
        return ptr(taddr(addr));
    }
}
