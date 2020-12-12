//
// Created by dcnick3 on 12/11/20.
//

#pragma once

#include <stdexcept>

namespace uwin::xcute {
    class process_exit : std::exception {
    public:
        std::uint32_t _exit_code;
        inline explicit process_exit(std::uint32_t exit_code) : _exit_code(exit_code) {}
    };
}
