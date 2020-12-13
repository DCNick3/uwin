//
// Created by dcnick3 on 12/13/20.
//

#pragma once

#include "ht/handle_holder.h"
#include "heap/heap.h"

namespace uwin::ctx {
    class process_heap : public ht::handle_holder<heap::heap> {
    public:
        process_heap(ht::handletable &handletable, mem::mgr::target_mem_mgr& mem_mgr)
            : handle_holder(handletable, handletable.emplace<heap::heap>(mem_mgr, 0, 0)) {
        }
    };
}