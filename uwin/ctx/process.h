//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include "mem/mgr/target_mem_mgr.h"
#include "mem/tptr.h"
#include "ht/handlelike_allocator.h"
#include "ht/handletable.h"
#include "ht/handle_holder.h"
#include "ctx/env.h"
#include "ctx/process_heap.h"
#include "heap/heap.h"

namespace uwin::ctx {
    class dll;
    class ldr;

    class process {
    public:
        mem::mgr::target_mem_mgr& _mem_mgr;
        dll& _dll;
        ldr& _ldr;
        ht::handlelike_allocator& _handlelike_allocator;
        ht::handletable& _handle_table;
        process_heap _process_heap;
        env& _env;
    };
}