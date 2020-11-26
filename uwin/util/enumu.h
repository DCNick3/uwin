//
// Created by dcnick3 on 11/25/20.
//

#pragma once

#include <type_traits>

namespace uwin {
    template<typename Enum>
    struct EnableBitMaskOperators {
        static const bool enable = false;
    };
}

template<typename Enum>
typename std::enable_if<uwin::EnableBitMaskOperators<Enum>::enable, Enum>::type
operator|(Enum lhs, Enum rhs) {
    using underlying = typename std::underlying_type<Enum>::type;
    return static_cast<Enum> (
            static_cast<underlying>(lhs) |
            static_cast<underlying>(rhs)
    );
}

template<typename Enum>
typename std::enable_if<uwin::EnableBitMaskOperators<Enum>::enable, Enum>::type
operator&(Enum lhs, Enum rhs) {
    using underlying = typename std::underlying_type<Enum>::type;
    return static_cast<Enum> (
            static_cast<underlying>(lhs) &
            static_cast<underlying>(rhs)
    );
}

// operator for bitmask check
template<typename Enum>
typename std::enable_if<uwin::EnableBitMaskOperators<Enum>::enable, bool>::type
operator%(Enum lhs, Enum rhs) {
    using underlying = typename std::underlying_type<Enum>::type;
    return (static_cast<underlying>(lhs) & static_cast<underlying>(rhs)) != 0;
}