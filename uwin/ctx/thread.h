//
// Created by dcnick3 on 12/5/20.
//

#pragma once

#include "win32/error_code.h"
#include "ctx/process.h"

namespace uwin::ctx {
    class thread {
    public:
        win32::error_code _win32_last_error{win32::error_code::ERROR_SUCCESS};
        process& _process;

        inline void set_last_error(win32::error_code error_code) {
            _win32_last_error = error_code;
        }

        [[nodiscard]] inline win32::error_code get_last_error() const {
            return _win32_last_error;
        }
    };
}