//
// Created by dcnick3 on 10/28/20.
//

#ifdef _GNU_SOURCE
#undef _GNU_SOURCE
#endif
#ifdef _POSIX_C_SOURCE
#undef _POSIX_C_SOURCE
#endif
#define _POSIX_C_SOURCE 200809L

#include "posix/except.h"

#include <cstring>

namespace uwin::posix {
    exception::exception(const std::string &what_failed, int err) : std::runtime_error("meh") {
        char buffer[256];

        int r = strerror_r(err, buffer, sizeof(buffer));
        if (r != 0)
            std::terminate();

        _what = what_failed + " failed: " + buffer;
    }
}
