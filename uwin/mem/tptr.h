//
// Created by dcnick3 on 10/27/20.
//

#pragma once

#include "util/align.h"

#include <cstdint>

namespace uwin::mem {
    template<typename T, bool C = false>
    class tptr {
    public:
        using tvalue = std::uint32_t;
        using tsvalue = std::int32_t;

        static constexpr bool constant = C;

    private:
        tvalue _value;

    public:
        inline tptr(tvalue value) : _value(value) {} // NOLINT(google-explicit-constructor)

        [[nodiscard]] constexpr inline tvalue value() const { return _value; }

        constexpr inline tptr operator+(tvalue o) const { return _value + o; }

        constexpr inline tptr operator-(tvalue o) const { return _value - o; }

        template<bool C1>
        constexpr inline tsvalue operator-(tptr<T, C1> o) const { return _value - o._value; }

        constexpr inline tvalue operator%(tvalue o) const { return _value % o; }

        constexpr inline tvalue operator/(tvalue o) const { return _value / o; }

        constexpr inline tptr align_up(tvalue alignment) const { return util::align_up(*this, alignment); }

        constexpr inline tptr align_down(tvalue alignment) const { return util::align_down(*this, alignment); }

        [[nodiscard]] constexpr inline bool is_aligned(tvalue alignment) const {
            return util::is_aligned(*this, alignment);
        }

        constexpr inline tptr &operator+=(tvalue o) {
            _value += o;
            return *this;
        }

        constexpr inline tptr &operator-=(tvalue o) {
            _value += o;
            return *this;
        }

        constexpr inline tptr &operator++() { return operator+=(1); }

        constexpr inline tptr &operator--() { return operator-=(1); }

        template<bool C1>
        constexpr inline bool operator==(tptr<T, C1> o) const { return _value == o._value; }

        constexpr inline bool operator==(tvalue o) const { return _value == o; }

        template<bool C1>
        constexpr inline bool operator!=(tptr<T, C1> o) const { return _value != o._value; }

        constexpr inline bool operator!=(tvalue o) const { return _value != o; }

        template<bool C1>
        constexpr inline bool operator<(tptr<T, C1> o) const { return _value < o._value; }

        constexpr inline bool operator<(tvalue o) const { return _value < o; }

        template<bool C1>
        constexpr inline bool operator<=(tptr<T, C1> o) const { return _value <= o._value; }

        constexpr inline bool operator<=(tvalue o) const { return _value <= o; }

        template<bool C1>
        constexpr inline bool operator>(tptr<T, C1> o) const { return _value > o._value; }

        constexpr inline bool operator>(tvalue o) const { return _value > o; }

        template<bool C1>
        constexpr inline bool operator>=(tptr<T, C1> o) const { return _value >= o._value; }

        constexpr inline bool operator>=(tvalue o) const { return _value >= o; }

        template<typename DT>
        constexpr inline tptr<DT, C> as() const { return tptr<DT, C>(_value); }

        inline auto as_taddr() const { return as<std::uint8_t>(); }

        template<typename MGR>
        inline auto to_host(MGR const &mgr) const { return mgr.guest_to_host(*this); }

        template<typename MGR>
        inline auto deref(MGR const &mgr) const { return mgr.deref(*this); }
    };

    using taddr = tptr<std::uint8_t>;

    template<typename T>
    using tcptr = tptr<T, true>;

    using tcaddr = tcptr<std::uint8_t>;
}