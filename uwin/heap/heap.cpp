//
// Created by dcnick3 on 12/1/20.
//

#include "heap/heap.h"

namespace uwin::heap {

    block *heap::block_to_first_block(block *ptr) {
        return reinterpret_cast<block*>(util::align_down(reinterpret_cast<std::uintptr_t>(ptr), consts::span_size));
    }

    mem::taddr heap::alloc(mem::taddr::tvalue size) {

        return uwin::mem::taddr(0);
    }

    mem::taddr heap::realloc(mem::taddr ptr, mem::taddr::tvalue new_size) {
        return uwin::mem::taddr(0);
    }

}
