//
// Created by dcnick3 on 11/29/20.
//

#include "impl.h"

namespace uwin::win32::dll {

    uint32_t KERNEL32_impl::GetVersion() {
        return 0xc0000004; // As returned by windows 95
    }
}