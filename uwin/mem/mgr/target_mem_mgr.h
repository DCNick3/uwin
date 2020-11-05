//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "mem/mem_mapper.h"
#include "mem/mem_region.h"
#include "pages_regions_container.h"
#include "mem/tptr.h"

#include <memory>
#include <set>

#include <cassert>

namespace uwin {
    namespace mem {
        namespace mgr {

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
                explicit target_mem_mgr(std::shared_ptr<base_mem_mapper> mapper);

                tmem_region reserve_dynamic(taddr::tvalue size);
                tmem_region reserve_fixed(tmem_region region);
                tmem_region unreserve(taddr start_addr);

                tmem_region commit(tmem_region region, tprot prot);
                tmem_region reprotect(tmem_region region, tprot prot);
                tmem_region uncommit(tmem_region region);
                tmem_region uncommit_whole_reserved_region(taddr start_addr);

                template<typename T>
                inline T *guest_to_host(taddr addr) {
                    auto res = _host_region.begin() + addr.value();
                    assert(res < _host_region.end());
                    return reinterpret_cast<T *>(res);
                }

                inline hmem_region guest_to_host(tmem_region const& region) {
                    return {guest_to_host<std::uint8_t>(region.begin()), region.size()};
                }

                template<typename T>
                inline T &deref(tptr<T> ptr) {
                    return *guest_to_host<T>(ptr.as_taddr());
                }

                inline std::uint8_t* get_region_base() const { return _host_region.begin(); }
            };
        }
    }
}