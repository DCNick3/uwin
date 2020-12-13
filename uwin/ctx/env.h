//
// Created by dcnick3 on 12/13/20.
//

#pragma once

#include "mem/tptr.h"
#include "ctx/process_heap.h"

#include <vector>

namespace uwin::ctx {
    struct env_param {
        // TODO: make sure that boost::di will not inject it automagically
        std::string command_line;
        std::vector<std::string> environment;
    };

    class env {
    public:
        inline env(env_param const& env_param, process_heap& heap) {
            _command_line = heap->alloc_str(env_param.command_line);

            for (auto const& str : env_param.environment) {
                _environment.insert(_environment.end(), str.begin(), str.end());
                _environment.push_back(0);
            }
            _environment.push_back(0);
        }
        mem::tptr<char> _command_line{0};
        std::vector<std::uint8_t> _environment{0};
    };
}