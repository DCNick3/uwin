//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "ht/kobj.h"
#include "mem/tptr.h"
#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "heap/block.h"
#include "heap/consts.h"
#include "heap/span.h"

#include <type_traits>
#include <set>

namespace uwin::heap {
    class heap : public ht::kobj {

        mem::mgr::target_mem_mgr& _mem_mgr;

        [[nodiscard]] static block* block_to_first_block(block* ptr) ;

    public:
        inline heap(mem::mgr::target_mem_mgr& mem_mgr, mem::taddr::tvalue initial_size, mem::taddr::tvalue maximum_size)
            : _mem_mgr(mem_mgr) {
            if (maximum_size != 0)
                throw util::not_implemented_error("Non-growable heap");
            assert(util::is_aligned(initial_size, mem::mgr::consts::page_size));
            assert(util::is_aligned(maximum_size, mem::mgr::consts::page_size));
        }

        // Quite a dumb algorithm with almost no research conducted. Probably performs quite poorly
        mem::taddr alloc(mem::taddr::tvalue size);

        mem::taddr realloc(mem::taddr ptr, mem::taddr::tvalue new_size);
    };
}