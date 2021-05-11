//
// Created by dcnick3 on 11/29/20.
//

#pragma once

#include "win32/dll/KERNEL32_iface.h"
#include "win32/ldr/module_table.h"
#include "ctx/process_heap.h"

namespace uwin::win32::dll {
    class KERNEL32_impl : public KERNEL32_iface {
    private:
        ht::handletable& _handletable;
        ctx::env& _env;
        ctx::process_heap& _process_heap;
        ldr::module_table& _module_table;

    public:
        explicit inline KERNEL32_impl(mem::mgr::target_mem_mgr &target_mem_mgr, ht::handletable &handletable, ctx::env &env,
                               ctx::process_heap &process_heap, uconv &uconv,
                                      ldr::module_table& module_table)
            : KERNEL32_iface(target_mem_mgr, uconv),
              _handletable(handletable),
              _env(env), _process_heap(process_heap),
              _module_table(module_table) {}

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

        mem::tptr<void>
        VirtualAlloc(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flAllocationType,
                     std::uint32_t flProtect) override;

        bool VirtualFree(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t dwFreeType) override;

        bool VirtualProtect(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flNewProtect,
                            uwin::mem::tptr<uint32_t> lpflOldProtect) override;

        uint32_t VirtualQuery(uwin::mem::tcptr<void> committed,
                              uwin::mem::tptr<uwin::win32::types::MEMORY_BASIC_INFORMATION> lpBuffer,
                              std::uint32_t dwLength) override;

        void GetStartupInfoA(uwin::mem::tptr<uwin::win32::types::STARTUPINFOA> lpStartupInfo) override;

        ht::handle <uwin::ht::kobj> GetStdHandle(std::uint32_t nStdHandle) override;

        uint32_t SetHandleCount(std::uint32_t uNumber) override;

        mem::tptr<char> GetCommandLineA() override;

        mem::tptr<char16_t> GetEnvironmentStringsW() override;
        mem::tptr<char> GetEnvironmentStrings() override;

        mem::tptr<char> GetEnvironmentStringsA() override;

        bool FreeEnvironmentStringsA(uwin::mem::tptr<char> arg0) override;

        uint32_t GetOEMCP() override;

        uint32_t GetACP() override;

        bool GetCPInfo(std::uint32_t CodePage, uwin::mem::tptr<uwin::win32::types::CPINFO> lpCPInfo) override;

        bool GetStringTypeW(std::uint32_t dwInfoType, uwin::mem::tcptr<char16_t> lpSrcStr, std::int32_t cchSrc,
                            uwin::mem::tptr<uint16_t> lpCharType) override;

        bool GetStringTypeA(std::uint32_t Locale, std::uint32_t dwInfoType, uwin::mem::tcptr<char> lpSrcStr,
                            std::int32_t cchSrc, uwin::mem::tptr<uint16_t> lpCharType) override;

        int32_t LCMapStringA(std::uint32_t Locale, std::uint32_t dwMapFlags, uwin::mem::tcptr<char> lpSrcStr,
                             std::int32_t cchSrc, uwin::mem::tptr<char> lpDestStr, std::int32_t cchDest) override;

        int32_t LCMapStringW(std::uint32_t Locale, std::uint32_t dwMapFlags, uwin::mem::tcptr<char16_t> lpSrcStr,
                             std::int32_t cchSrc, uwin::mem::tptr<char16_t> lpDestStr, std::int32_t cchDest) override;

        uint32_t GetModuleFileNameA(uwin::win32::types::hmodule hModule, uwin::mem::tptr<char> lpFilename,
                                    std::uint32_t nSize) override;

        bool IsBadReadPtr(uwin::mem::tcptr<void> lp, std::uint32_t ucb) override;

        bool IsBadWritePtr(uwin::mem::tptr<void> lp, std::uint32_t ucb) override;

        types::hmodule GetModuleHandleA(uwin::mem::tcptr<char> lpModuleName) override;

        mem::tptr<void> GetProcAddress(uwin::win32::types::hmodule hModule, uwin::mem::tcptr<char> lpProcName) override;

        void ExitProcess(std::uint32_t uExitCode) override;

        uint32_t GetCurrentThreadId() override;

        mem::tptr<void> SetUnhandledExceptionFilter(uwin::mem::tptr<void> lpTopLevelExceptionFilter) override;

        ht::handle<uwin::ht::kobj>
        CreateEventA(uwin::mem::tptr<uwin::win32::types::SECURITY_ATTRIBUTES> lpEventAttributes, bool bManualReset,
                     bool bInitialState, uwin::mem::tcptr<char> lpName) override;

        mem::tptr<char16_t> GetCommandLineW() override;

        static str::native normalize_module_name(str::native_view unnormalized);
    };
}
