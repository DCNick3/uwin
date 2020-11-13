//
// Created by dcnick3 on 11/6/20.
//

#pragma once

#include <cstdint>

namespace uwin {
    namespace win32 {
        namespace types {
            class wnd;

            template<typename T>
            class handle {
                std::uint32_t _value;
            public:
                inline std::uint32_t value() const { return _value; }

                explicit inline handle(std::uint32_t value) : _value(value) {}
            };
        }
    }
}
