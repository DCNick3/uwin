//
// Created by dcnick3 on 11/29/20.
//

#pragma once

#include "win32/dll/KERNEL32_iface.h"

namespace uwin::win32::dll {
    class KERNEL32_impl : public KERNEL32_iface {
    public:
        explicit KERNEL32_impl(ctx::process &process_ctx) : KERNEL32_iface(process_ctx) {}

        uint32_t GetVersion() override;

        ht::handle<uwin::ht::kobj>
        HeapCreate(std::uint32_t flOptions, std::uint32_t dwInitialSize, std::uint32_t dwMaximumSize) override;

        bool HeapDestroy(uwin::ht::handle<uwin::ht::kobj> hHeap) override;

        mem::tptr<void>
        HeapAlloc(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags, std::uint32_t dwBytes) override;

        mem::tptr<void>
        HeapReAlloc(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags, uwin::mem::tptr<void> lpMem,
                    std::uint32_t dwBytes) override;

        bool
        HeapFree(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags, uwin::mem::tptr<void> lpMem) override;

        uint32_t
        HeapSize(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags, uwin::mem::tcptr<void> lpMem) override;
    };
}
