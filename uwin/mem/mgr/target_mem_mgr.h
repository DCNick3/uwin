//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "mem/mem_mapper.h"
#include "mem/mem_region.h"
#include "pages_regions_container.h"
#include "mem/tptr.h"
#include "mem/mgr/query_results.h"
#include "win32/types/types.h"
#include "str/str.h"

#include <memory>
#include <set>

#include <cassert>

namespace uwin::mem::mgr {

    // Here we try to emulate windows virtual memory model
    // To access memory you need to do two things: RESERVE it and COMMIT (See VirtualAlloc docs)
    //
    // Reservation (and unreservation) is done in regions.
    // You cannot unreserve region partially, only full region.
    // (Regions can be listed with new win10 RS1 QueryVirtualMemoryInformation API; does not work on WoW64 for me)
    // Also note that reserved region address must be 64KiB-aligned, but size requires only 4KiB alignment
    // This implies that if, for example, you allocate a region with size=4KiB, you lose 60KiB of address space
    // due to internal fragmentation: no other reservation can use that space, as there are no 64KiB aligned
    // addresses there.
    // Windows 95 has a bit different semantics: it rounds up your reservation request to 64 KiB so you do not lose that memory
    // you can commit it. I do not (yet) implement this semantic and go with the NT one (cuz meh, does anything that is not system program rely on it?)
    // (can be studied with VirtualQuery)
    //
    // Commiting, on the other hand, is done with page (4KiB) granularity, you can commit and uncommit any 4 KiB
    // (or more) of the reserved region. Changing page protection can also be done at individual page basis
    //
    // Therefore the following approach is taken:
    // We store a sorted set of reserved regions. Inside those regions we store info about commited pages
    // This storage is implemented as pages_regions_container and pages_region (does not do any allocation, only bookkeeping)
    // Also looks like this is the approach taken in windows, as it does not allow either VirtualProtect or
    // VirtualAlloc with MEM_COMMIT to cross the reserved region boundary, even if they are adjacent

    // This api tries to save winapi semantics, but splitting various paths in VirtualAlloc and VirtualFree

    class target_mem_mgr {
        std::shared_ptr<base_mem_mapper> _mapper;
        hmem_region _host_region;
        pages_regions_container _regions_container;

        static hprot to_hprot(tprot prot);

        decltype(_regions_container)::iterator find_containing_win32(tmem_region region);

        decltype(_regions_container)::iterator find_starting_with_win32(taddr start_addr);

    public:
        ~target_mem_mgr();

        typedef std::variant<
                query_results::free,
                query_results::reserved,
                query_results::committed
        > query_result;

        explicit target_mem_mgr(std::shared_ptr<base_mem_mapper> mapper);

        tmem_region reserve_dynamic_aligned(taddr::tvalue size, taddr::tvalue alignment);

        inline tmem_region reserve_dynamic(taddr::tvalue size) { return reserve_dynamic_aligned(size, 1); }

        tmem_region reserve_fixed(tmem_region region);

        tmem_region unreserve(taddr start_addr);

        tmem_region commit(tmem_region region, tprot prot);

        tmem_region reprotect(tmem_region region, tprot prot);

        tmem_region uncommit(tmem_region region);

        tmem_region uncommit_whole_reserved_region(taddr start_addr);

        query_result query(taddr ptr) const;

        [[nodiscard]] std::string dump_memory_map() const;

        template<typename T>
        [[nodiscard]] inline auto ptr(tptr<T> addr) const {
            auto res = _host_region.begin() + addr.value();
            assert(res < _host_region.end());
            return reinterpret_cast<T *>(res);
        }

        template<typename T>
        [[nodiscard]] inline auto ptr(tcptr<T> addr) const {
            auto res = _host_region.begin() + addr.value();
            assert(res < _host_region.end());
            return reinterpret_cast<T const *>(res);
        }

        [[nodiscard]] [[maybe_unused]] void* ptr_raw(taddr::tvalue addr) const;

        [[nodiscard]] inline hmem_region ptr(tmem_region const &region) const {
            return {ptr<std::uint8_t>(region.begin()), region.size()};
        }

        template<typename T>
        inline auto tptr(T* ptr) const {
            auto bptr = reinterpret_cast<std::uint8_t*>(ptr);
            assert(_host_region.begin() <= bptr && _host_region.end() > bptr);
            return mem::tptr<T>(bptr - _host_region.begin());
        }

        template<typename T>
        inline auto tptr(T const* ptr) const {
            auto bptr = reinterpret_cast<std::uint8_t const*>(ptr);
            assert(_host_region.begin() <= bptr && _host_region.end() > bptr);
            return mem::tcptr<T>(bptr - _host_region.begin());
        }

        template<typename T>
        inline T &deref(mem::tptr<T, false> tptr) const {
            return *ptr<T>(tptr);
        }

        template<typename T>
        inline T const &deref(mem::tptr<T, true> tptr) const {
            return *ptr<T>(tptr);
        }

        template<bool C>
        inline str::narrow_view str(mem::tptr<char, C> tptr) const {
            auto ptr = &deref(tptr);
            auto size = strlen(ptr); // this can blow up. Add a length limit? Something else?
            return str::narrow_view(ptr, size);
        }

        [[nodiscard]] inline std::uint8_t *get_region_base() const { return _host_region.begin(); }
    };
}