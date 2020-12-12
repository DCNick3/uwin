//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "mem/tptr.h"

#include <type_traits>

namespace uwin::ht {

    class kobj;

    template<typename T>
    class handle {
    public:
        typedef uwin::mem::taddr::tvalue tvalue;

        static_assert(std::is_base_of_v<kobj, T>, "T must be a descendant of kobj");
    private:
        tvalue _value;
    public:
        [[nodiscard]] inline tvalue value() const { return _value; }

        explicit inline handle(tvalue value) : _value(value) {}

        static constexpr tvalue invalid_value = 0xffffffff;

        [[nodiscard]] static handle invalid() { return handle(invalid_value); }

        [[nodiscard]] inline bool is_invalid() const { return _value == invalid_value; }

        template<typename D>
        [[nodiscard]] handle<D> cast() const {
            static_cast<D*>(static_cast<T*>(nullptr));
            return handle<D>(value());
        }

        [[nodiscard]] handle<kobj> as_kobj() const {
            return cast<kobj>();
        }
    };

    typedef handle<kobj>::tvalue handle_tvalue;
}