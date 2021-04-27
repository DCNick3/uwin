//
// Created by dcnick3 on 11/6/20.
//

#pragma once

#include "mem/tptr.h"
#include "win32/types/wnd.h"
#include "win32/types/cursor.h"
#include "win32/types/icon.h"
#include "win32/types/menu.h"
#include "win32/types/hmodule.h"

#include <cstdint>

namespace uwin::win32::types {
    // names in CAPS are POD types that can be directly embedded into structures or pointed to in target memory
    // of course they must have the same layout as in the target, so marking them with [[gnu::packed]] might be a good idea


    typedef std::uint32_t BOOL;
    typedef std::int32_t HRESULT;

    struct [[gnu::packed]] STARTUPINFOA {
        std::uint32_t cb;
        mem::tptrpod<char> lpReserved;
        mem::tptrpod<char> lpDesktop;
        mem::tptrpod<char> lpTitle;
        std::uint32_t dwX;
        std::uint32_t dwY;
        std::uint32_t dwXSize;
        std::uint32_t dwYSize;
        std::uint32_t dwXCountChars;
        std::uint32_t dwYCountChars;
        std::uint32_t dwFillAttribute;
        std::uint32_t dwFlags;
        std::uint16_t wShowWindow;
        std::uint16_t cbReserved2;
        mem::tptrpod<std::uint8_t> lpReserved2;
        ht::handle_tvalue hStdInput;
        ht::handle_tvalue hStdOutput;
        ht::handle_tvalue hStdError;
    };
    static_assert(sizeof(STARTUPINFOA) == 17 * 4, "Unexpected STARTUPINFOA size");

    struct EXCEPTION_POINTERS {};
    struct OVERLAPPED {};
    struct EXCEPTION_RECORD {};
    struct [[gnu::packed]] CPINFO {
        std::uint32_t MaxCharSize;
        std::uint8_t DefaultChar[2];
        std::uint8_t LeadByte[12];
        std::uint8_t padding[2];
    };
    static_assert(sizeof(CPINFO) == 20, "Unexpected CPINFO size");

    struct WNDCLASSA {};
    struct MSG {};
    struct [[gnu::packed]] GUID {
        std::uint32_t Data1;
        std::uint16_t Data2;
        std::uint16_t Data3;
        std::uint8_t Data4[8];
    };
    struct SECURITY_ATTRIBUTES {};
    struct [[gnu::packed]] MEMORY_BASIC_INFORMATION {
        mem::tcaddrpod BaseAddress;
        mem::tcaddrpod AllocationBase;
        std::uint32_t  AllocationProtect;
        std::uint32_t  RegionSize;
        std::uint32_t  State;
        std::uint32_t  Protect;
        std::uint32_t  Type;
    };
    static_assert(sizeof(MEMORY_BASIC_INFORMATION) == 28, "Unexpected MEMORY_BASIC_INFORMATION size");
}
