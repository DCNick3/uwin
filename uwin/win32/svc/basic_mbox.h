//
// Created by dcnick3 on 12/25/20.
//

#pragma once

#include "win32/svc/mbox.h"

namespace uwin::win32::svc {
    class basic_mbox : public mbox {
    public:
        RESULT show(str::native text, str::native caption, TYPE type) override;
    };
}
