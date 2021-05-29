//
// Created by dcnick3 on 5/13/21.
//

#include "win32/svc/basic_stdio.h"
#include "log/log.h"

namespace uwin::win32::svc {

    void basic_stdio::write_stdout(str::native_view text) {
        log::info("basic_stdio::write_stdout({})", text);
    }

    void basic_stdio::write_stderr(str::native_view text) {
        log::info("basic_stdio::write_stderr({})", text);
    }
}
