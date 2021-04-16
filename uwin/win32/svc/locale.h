//
// Created by dcnick3 on 12/24/20.
//

#pragma once

#include <string>

namespace uwin::win32::svc {
    class locale {
    public:
        virtual std::string ascii_to_native(std::string_view view) = 0;
        virtual std::u16string ascii_to_wide(std::string_view view) = 0;
        virtual std::string wide_to_native(std::u16string_view view) = 0;
        virtual std::string wide_to_ascii(std::u16string_view view) = 0;
        virtual std::string native_to_ascii(std::string_view view) = 0;
        virtual std::u16string native_to_wide(std::string_view view) = 0;

        // ascii and wide corresponds to terms used in MS docs
        // native is an encoding native for host, probably should be utf8 (could be used to call native fs functions and stuff..)

        // todo: implement various string tests, like check whether the char is upper or lower case
    };
}