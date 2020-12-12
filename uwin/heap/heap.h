//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "ht/kobj.h"
#include "mem/tptr.h"
#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "heap/consts.h"
#include "heap/span_group.h"

#include <type_traits>
#include <set>

namespace uwin::heap {
    class heap : public ht::kobj {

        mem::mgr::target_mem_mgr& _mem_mgr;
        span_group _span_group; // TODO: create separate span groups for different sizes

        std::set<mem::taddr> _raw_allocations;

    public:
        inline heap(mem::mgr::target_mem_mgr& mem_mgr, mem::taddr::tvalue initial_size, mem::taddr::tvalue maximum_size)
            : _mem_mgr(mem_mgr), _span_group(_mem_mgr) {
            if (maximum_size != 0)
                throw util::not_implemented_error("Non-growable heap");
            assert(util::is_aligned(initial_size, mem::mgr::consts::page_size));
            assert(util::is_aligned(maximum_size, mem::mgr::consts::page_size));
            // TODO: do not ignore initial_size
        }

        mem::taddr alloc(mem::taddr::tvalue size);

        mem::taddr realloc(mem::taddr old_ptr, mem::taddr::tvalue new_size);

        inline mem::taddr::tvalue size(mem::taddr ptr) {
            return span_group::size(_mem_mgr, ptr);
        }

        void free(mem::taddr ptr);

        ~heap() override;
    };
}