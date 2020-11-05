//
// Created by dcnick3 on 10/28/20.
//

#pragma once

#include <exception>
#include <string>

namespace uwin {
    namespace posix {

        class exception : public std::exception {
            std::string _what;
        public:
            exception(std::string const& what_failed, int err);

            inline const char *what() const noexcept override {
                return _what.c_str();
            }
        };
    }
}