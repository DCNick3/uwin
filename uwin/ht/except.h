//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include <stdexcept>

namespace uwin::ht {
    class destroyed_kobj_access_error : public std::runtime_error {
    public:
        inline explicit destroyed_kobj_access_error() : runtime_error("Access to destroyed kobj happened") {}
    };
}