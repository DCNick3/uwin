//
// Created by dcnick3 on 11/1/20.
//

#pragma once

#include <cstdint>

namespace uwin::mem::mgr {
    enum class tprot : std::uint8_t {
        none = 0U,
        r = (1U << 0U),
        w = (1U << 1U),
        x = (1U << 2U),
        rw = r | w,
        rx = r | x,
        wx = w | x, // does this even make sense? =)
        rwx = r | w | x,
    };


}
