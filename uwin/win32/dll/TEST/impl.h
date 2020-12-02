//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include "win32/dll/TEST_iface.h"

namespace uwin::win32::dll {
    class TEST_impl : public TEST_iface {
    public:
        explicit TEST_impl(ctx::process &process_ctx) : TEST_iface(process_ctx) {}

        int32_t
        MessageBoxA(uwin::ht::handle<uwin::win32::types::wnd> hWnd, uwin::mem::tcptr<char> lpText,
                    uwin::mem::tcptr<char> lpCaption, std::uint32_t uType) override;

        void VoidTest() override;

        uint32_t SubTest(std::uint32_t a, std::uint32_t b) override;
    };
}
