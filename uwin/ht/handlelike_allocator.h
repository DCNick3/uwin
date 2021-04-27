//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "ht/handle.h"
#include "win32/types/types.h"
#include "mem/tptr.h"cd
#include "mem/mgr/target_mem_mgr.h"
#include "util/nocopy.h"
#include "util/nomove.h"

namespace uwin::ht {
    class handlelike_allocator : util::nocopy, util::nomove {
        static_assert(std::is_same_v<handle_tvalue,
                std::remove_cv_t<decltype(mem::mgr::consts::address_space_reserved_start)>>,
                "decltype(mem::mgr::consts::address_space_reserved_start) should be handle_tvalue");

        static const handle_tvalue start = mem::mgr::consts::address_space_reserved_start;

        handle_tvalue _last_allocated = start;

    public:
        inline handle_tvalue allocate() {
            return ++_last_allocated;
        }
    };
}