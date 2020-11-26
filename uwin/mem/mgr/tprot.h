//
// Created by dcnick3 on 11/1/20.
//

#pragma once

#include "util/enumu.h"

#include <fmt/format.h>

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

template<>
struct uwin::EnableBitMaskOperators<uwin::mem::mgr::tprot> {
    static const bool enable = true;
};

template<>
struct fmt::formatter<uwin::mem::mgr::tprot> : formatter<string_view> {
    using tprot = uwin::mem::mgr::tprot;
    // parse is inherited from formatter<string_view>.
    template<typename FormatContext>
    auto format(uwin::mem::mgr::tprot c, FormatContext &ctx) {
        std::string name = fmt::format("{}{}{}",
                                       c % tprot::r ? 'r' : '-',
                                       c % tprot::w ? 'w' : '-',
                                       c % tprot::x ? 'x' : '-');
        return formatter<string_view>::format(name, ctx);
    }
};