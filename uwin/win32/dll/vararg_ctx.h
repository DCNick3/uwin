//
// Created by dcnick3 on 12/25/20.
//

#pragma once

#include <cstdint>

namespace uwin::win32::dll {
    class vararg_ctx {
        std::uint32_t const* _top_ptr;
    public:
        inline explicit vararg_ctx(const uint32_t *top_ptr) : _top_ptr(top_ptr) {}

        inline std::uint32_t pop_u32() {
            return *(_top_ptr++);
        }
        inline std::int32_t pop_s32() {
            auto res = pop_u32();
            return *reinterpret_cast<std::int32_t*>(&res);
        }
    };
}