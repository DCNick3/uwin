//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include <nonstd/span.hpp>

#include <vector>
#include <cstdint>

namespace uwin {
    namespace win32 {
        namespace ldr {
            class pe_image {
                nonstd::span<std::uint8_t> _data;

                void parse();
                void pe_assert(bool condition);
            public:
                explicit pe_image(nonstd::span<std::uint8_t> data) : _data(data) {
                    parse();
                }


            };
        }
    }
}