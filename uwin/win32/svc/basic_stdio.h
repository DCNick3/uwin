//
// Created by dcnick3 on 5/13/21.
//

#pragma once

#include "win32/svc/stdio.h"
#include "str/uconv.h"

namespace uwin::win32::svc {
    class basic_stdio : public stdio {
        void write_stdout(str::native_view data) override;
        void write_stderr(str::native_view data) override;
    };
}
