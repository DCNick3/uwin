//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include <vector>
#include <cstdint>
#include <string>
#include <fstream>
#include <cstdarg>
#include "str/str.h"

namespace uwin::util {
    inline std::string ascii_to_upper(std::string_view view) {
        std::string res = std::string(view);
        for (auto& c : res) {
            if (c >= 'a' && c <= 'z') {
                c = c - 0x20;
            }
        }
        return res;
    }

    inline std::string ascii_to_lower(std::string_view view) {
        std::string res = std::string(view);
        for (auto& c : res) {
            if (c >= 'A' && c <= 'Z') {
                c = c + 0x20;
            }
        }
        return res;
    }

    // using C features here for interoperability with remill runtime
    inline std::string format_printf_style_vl(char const* format, va_list arg) {
        int size_s = std::vsnprintf(nullptr, 0, format, arg) + 1; // Extra space for '\0'
        if (size_s <= 0) { throw std::runtime_error("Error during formatting."); }
        auto size = static_cast<size_t>( size_s );
        auto buf = std::make_unique<char[]>(size);
        std::vsnprintf(buf.get(), size, format, arg);
        return std::string(buf.get(), buf.get() + size - 1); // We don't want the '\0' inside
    }

    inline std::string format_printf_style_va(char const* format, ...) {
        va_list arg;
        va_start(arg, format);
        return format_printf_style_vl(format, arg);
        va_end(arg);
    }

    template<typename ...Args>
    inline std::string format_printf_style(const std::string& format, Args... args) {
        return format_printf_style_va(format.c_str(), args...);
    }
}