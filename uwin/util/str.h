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
}