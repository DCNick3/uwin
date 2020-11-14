//
// Created by dcnick3 on 10/27/20.
//

#pragma once

namespace uwin::util {

    template<typename T1, typename T2>
    inline constexpr T1 align_down(T1 value, T2 alignment) {
        return value - (value % alignment);
    }

    template<typename T1, typename T2>
    inline constexpr T1 align_up(T1 value, T2 alignment) {
        return align_down(value + alignment - 1, alignment);
    }

    template<typename T1, typename T2>
    inline constexpr bool is_aligned(T1 value, T2 alignment) {
        return value % alignment == 0;
    }
}