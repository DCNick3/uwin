//
// Created by dcnick3 on 10/27/20.
//

#pragma once

#include "util/align.h"

#include <cstdint>

namespace uwin {
    namespace mem {

        template<typename T>
        class tptr {
        public:
            using tvalue = std::uint32_t;
            using tsvalue = std::int32_t;

        private:
            tvalue _value;

        public:
            inline tptr(tvalue value) : _value(value) {} // NOLINT(google-explicit-constructor)

            constexpr inline tvalue value() const { return _value; }

            constexpr inline tptr operator+(tvalue o) const { return _value + o; }
            constexpr inline tptr operator-(tvalue o) const { return _value - o; }

            constexpr inline tsvalue operator-(tptr<T> o) const { return _value - o._value; }

            constexpr inline tvalue operator%(tvalue o) const { return _value % o; }
            constexpr inline tvalue operator/(tvalue o) const { return _value / o; }

            constexpr inline tptr align_up(tvalue alignment) const { return util::align_up(*this, alignment); }
            constexpr inline tptr align_down(tvalue alignment) const { return util::align_down(*this, alignment); }
            constexpr inline bool is_aligned(tvalue alignment) const { return util::is_aligned(*this, alignment); }

            constexpr inline tptr& operator+=(tvalue o) { _value += o; return *this; }
            constexpr inline tptr& operator-=(tvalue o) { _value += o; return *this; }

            constexpr inline tptr& operator++() { return operator+=(1); }
            constexpr inline tptr& operator--() { return operator-=(1); }

            constexpr inline bool operator==(tptr<T> o) const { return _value == o._value; }
            constexpr inline bool operator==(tvalue o) const { return _value == o; }
            constexpr inline bool operator!=(tptr<T> o) const { return _value != o._value; }
            constexpr inline bool operator!=(tvalue o) const { return _value != o; }

            constexpr inline bool operator<(tptr<T> o) const { return _value < o._value; }
            constexpr inline bool operator<(tvalue o) const { return _value < o; }
            constexpr inline bool operator<=(tptr<T> o) const { return _value <= o._value; }
            constexpr inline bool operator<=(tvalue o) const { return _value <= o; }
            constexpr inline bool operator>(tptr<T> o) const { return _value > o._value; }
            constexpr inline bool operator>(tvalue o) const { return _value > o; }
            constexpr inline bool operator>=(tptr<T> o) const { return _value >= o._value; }
            constexpr inline bool operator>=(tvalue o) const { return _value >= o; }

            template<typename DT>
            constexpr inline tptr<DT> as() const { return tptr<DT>(_value); }

            inline tptr<std::uint8_t> as_taddr() const { return as<std::uint8_t>(); }
        };

        using taddr = tptr<std::uint8_t>;
    }
}