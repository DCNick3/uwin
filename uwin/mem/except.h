//
// Created by dcnick3 on 10/26/20.
//

#pragma once

#include <stdexcept>

namespace uwin {
    namespace mem {
        class out_of_target_address_space : std::runtime_error {
        public:
            out_of_target_address_space() : runtime_error("out of target address space") {}
        };
    }
}