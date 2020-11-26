//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include <vector>
#include <cstdint>
#include <string>
#include <fstream>

namespace uwin::util {
    inline std::vector<std::uint8_t> read_file(std::string const& path) {
        std::ifstream input(path, std::ios::binary);

        return {std::istreambuf_iterator<char>(input), {}};
    }
}