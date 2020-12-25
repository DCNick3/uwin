//
// Created by dcnick3 on 12/25/20.
//

#include "win32/svc/dummy_locale.h"

#include "util/except.h"

namespace uwin::win32::svc {

    std::string dummy_locale::ascii_to_native(std::string_view view) {
        return std::string(view);
    }

    std::u16string dummy_locale::ascii_to_wide(std::string_view view) {
        throw util::not_implemented_error("dummy_locale::ascii_to_native");
    }

    std::string dummy_locale::wide_to_native(std::u16string_view view) {
        throw util::not_implemented_error("dummy_locale::ascii_to_native");
    }

    std::string dummy_locale::wide_to_ascii(std::u16string_view view) {
        throw util::not_implemented_error("dummy_locale::ascii_to_native");
    }

    std::string dummy_locale::native_to_ascii(std::string_view view) {
        return std::string(view);
    }

    std::u16string dummy_locale::native_to_wide(std::string_view view) {
        throw util::not_implemented_error("dummy_locale::ascii_to_native");
    }
}
