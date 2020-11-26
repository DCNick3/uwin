//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include <streambuf>
#include <istream>
#include <span>

namespace uwin::util {
    class spanbuf : public std::basic_streambuf<char> {
    public:
        spanbuf(std::span<const std::uint8_t> data) {
            setg((char*)data.data(), (char*)data.data(), (char*)data.data() + data.size());
        }
    };

    class spanstream : public std::istream {
    public:
        spanstream(std::span<const std::uint8_t> data) :
                std::istream(&_buffer),
                _buffer(data) {
            rdbuf(&_buffer);
        }

    private:
        spanbuf _buffer;
    };
}