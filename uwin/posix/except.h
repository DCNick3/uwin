//
// Created by dcnick3 on 10/28/20.
//

#pragma once

#include <stdexcept>
#include <string>

namespace uwin {
    namespace posix {

        class exception : public std::runtime_error {
            std::string _what;
        public:
            exception(std::string const& what_failed, int err);

            inline const char *what() const noexcept override {
                return _what.c_str();
            }
        };
    }
}