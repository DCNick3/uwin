//
// Created by dcnick3 on 10/31/20.
//

#pragma once

#include "mem/tptr.h"

#include <cstdlib>

namespace uwin {
    namespace mem {
        namespace mgr {
            struct consts {
                static const std::size_t address_space_size = 1UL << 32UL;

                static const taddr::tvalue allocation_granularity = 0x10000;
                static const taddr::tvalue page_size = 0x1000;

                // we don't allocate first 64KiB to prevent null pointer dereferencing from succeeding
                // Maybe should reserve a bit more...
                static const taddr::tvalue initial_reserved_space = allocation_granularity;

                // On 32-bit windows only lower half (2 GiB) of the address space is usable by the usermode
                // The upper half is reserved for kernel. Will put handles there, I guess
                static const taddr::tvalue address_space_reserved_start = 1UL << 31UL;
            };
        }
    }
}