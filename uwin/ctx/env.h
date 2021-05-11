//
// Created by dcnick3 on 12/13/20.
//

#pragma once

#include "mem/tptr.h"
#include "ctx/process_heap.h"
#include "ctx/env_param.h"
#include "util/str.h"
#include "win32/uconv.h"

#include <vector>
#include <str/str.h>

namespace uwin::ctx {
    class env {
    public:
        inline env(env_param const& env_param, process_heap& heap,
                   win32::uconv const& uconv)
                : _command_line(env_param.command_line) {
            _command_line_ansi_ptr = heap->alloc_str(uconv.native_to_ansi(env_param.command_line));
            _command_line_wide_ptr = heap->alloc_str(uconv.native_to_wide(env_param.command_line));

            for (auto const& str : env_param.environment) {
                _environment.insert(_environment.end(), str.begin(), str.end());
                _environment.push_back(0);
            }
            _environment.push_back(0);
        }
        str::native _command_line;
        mem::tptr<char> _command_line_ansi_ptr{0};
        mem::tptr<char16_t> _command_line_wide_ptr{0};
        std::vector<std::uint8_t> _environment{0};
    };
}