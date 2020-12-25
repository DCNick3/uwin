//
// Created by dcnick3 on 12/25/20.
//

#pragma once

#include "win32/svc/locale.h"

namespace uwin::win32::svc {
    class dummy_locale : public locale {
    public:
        std::string ascii_to_native(std::string_view view) override;

        std::u16string ascii_to_wide(std::string_view view) override;

        std::string wide_to_native(std::u16string_view view) override;

        std::string wide_to_ascii(std::u16string_view view) override;

        std::string native_to_ascii(std::string_view view) override;

        std::u16string native_to_wide(std::string_view view) override;
    };
}