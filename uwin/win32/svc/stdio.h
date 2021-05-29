//
// Created by dcnick3 on 5/12/21.
//

#pragma once

#include <span>
#include <cstdint>
#include "str/str.h"

namespace uwin::win32::svc {
    class stdio {
        // this is very bare-bones API
        // Missing:
        // - Different encoding handling (well, it's not much of a problem if we are targeting win95, but might be otherwise)
        // - low level console functions https://docs.microsoft.com/en-us/windows/console/low-level-console-i-o
        //      (maybe should be implemented as a different interface)
        //      they assume the presence of the screen buffer, which... is not always easy to guarantee
        virtual void write_stdout(str::native_view text) = 0;
        virtual void write_stderr(str::native_view text) = 0;
    };
}