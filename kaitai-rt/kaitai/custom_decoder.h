#pragma once

#include <string>

namespace kaitai {

class custom_decoder {
public:
    virtual ~custom_decoder() {};
    virtual std::string decode(std::string src) = 0;
};

}
