//
// Created by dcnick3 on 12/5/20.
//

#pragma once

#include "win32/error_code.h"

namespace uwin::ctx {
    class thread {
    public:
        win32::error_code _win32_last_error{win32::error_code::ERROR_SUCCESS};
    };
}