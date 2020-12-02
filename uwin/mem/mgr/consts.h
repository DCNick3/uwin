//
// Created by dcnick3 on 10/31/20.
//

#pragma once

#include "mem/tptr.h"

#include <cstdlib>

namespace uwin::mem::mgr {
    struct consts {
        static constexpr std::size_t address_space_size = 1UL << 32UL;

        static constexpr taddr::tvalue allocation_granularity = 0x10000;
        static constexpr taddr::tvalue page_size = 0x1000;

        // we don't allocate first 64KiB to prevent null pointer dereferencing from succeeding
        // Maybe should reserve a bit more...
        static constexpr taddr::tvalue initial_reserved_space = allocation_granularity;

        // On 32-bit windows only lower half (2 GiB) of the address space is usable by the usermode
        // The upper half is reserved for kernel. Will put handles there, I guess
        static constexpr taddr::tvalue address_space_reserved_start = 1UL << 31UL;
    };
}