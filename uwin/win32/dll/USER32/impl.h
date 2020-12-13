//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include "win32/dll/USER32_iface.h"

namespace uwin::win32::dll {
    class USER32_impl : public USER32_iface {
    public:
        explicit USER32_impl(mem::mgr::target_mem_mgr &target_mem_mgr) : USER32_iface(target_mem_mgr) {}

        std::int32_t MessageBoxA(uwin::ht::handle<uwin::win32::types::wnd> hWnd, uwin::mem::tcptr<char> lpText,
                    uwin::mem::tcptr<char> lpCaption, std::uint32_t uType) override;
    };
}
