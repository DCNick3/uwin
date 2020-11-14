//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include "ctx/process.h"
#include "ctx/dll.h"

namespace uwin::win32::ldr {
    class ldr {
        ctx::process &_ctx;
    public:
        ldr(ctx::process &ctx, std::vector<std::uint8_t> const &image) : _ctx(ctx) {
        }


    };
}



