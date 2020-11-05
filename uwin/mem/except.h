//
// Created by dcnick3 on 10/26/20.
//

#pragma once

#include <exception>

namespace uwin {
    namespace mem {
        class out_of_target_address_space : std::exception {
            inline const char* what() const noexcept override {
                return "out of target address space";
            }
        };
    }
}