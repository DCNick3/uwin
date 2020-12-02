//
// Created by dcnick3 on 11/6/20.
//

#pragma once

#include "mem/tptr.h"
#include "win32/types/wnd.h"

#include <cstdint>

namespace uwin::win32::types {
    // names in CAPS are POD types that can be directly embedded into structures or pointed to in target memory
    // they must have the same layout as in the target

    typedef uwin::mem::taddr::tvalue hmodule;

    class obj {};

    typedef std::uint32_t BOOL;

    class STARTUPINFOA {};
    class EXCEPTION_POINTERS {};
    class OVERLAPPED {};
    class EXCEPTION_RECORD {};
    class CPINFO {};
}
