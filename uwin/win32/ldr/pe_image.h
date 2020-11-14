//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include <vector>
#include <cstdint>
#include <span>
#include <experimental/source_location>

namespace uwin::win32::ldr {
    class pe_image {
        std::span<std::uint8_t> _data;

        void parse();

        static void pe_assert(bool condition, std::experimental::source_location loc
            = std::experimental::source_location());

    public:
        explicit pe_image(std::span<std::uint8_t> data) : _data(data) {
            parse();
        }


    };
}