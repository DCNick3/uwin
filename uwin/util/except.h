//
// Created by dcnick3 on 10/28/20.
//

#pragma once

#include <string>
#include <stdexcept>

namespace uwin::util {
    std::string get_nice_current_exception_type_name();

    class not_implemented_error : std::runtime_error {
    public:
        not_implemented_error();
        explicit not_implemented_error(const std::string &what);
    };
}
