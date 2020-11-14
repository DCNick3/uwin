//
// Created by dcnick3 on 11/14/20.
//

#include "win32/ldr/pe_image.h"
#include "win32/ldr/pe_format_except.h"
#include <bit>

namespace uwin::win32::ldr {
    static_assert(std::endian::native == std::endian::little, "Only little endian is supported");

    void pe_image::parse() {

    }

    static void pe_assert(bool condition, std::experimental::source_location loc = std::experimental::source_location()) {
        throw pe_format_exception();
    }
}
