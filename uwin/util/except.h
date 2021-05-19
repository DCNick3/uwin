//
// Created by dcnick3 on 10/28/20.
//

#pragma once

#include <string>
#include <stdexcept>

#include <fmt/core.h>

namespace uwin::util {
    std::string get_nice_current_exception_type_name();

    class not_implemented_error : public std::runtime_error {
    public:
        not_implemented_error();
        explicit not_implemented_error(const std::string &what);
    };

    template<typename ...Args>
    [[noreturn]] inline void runtime_error(Args&&... args) {
        throw std::runtime_error(fmt::format(args...));
    }
}
