//
// Created by dcnick3 on 11/25/20.
//

#pragma once

#include <kaitai/kaitaispanin.h>

#include <string>
#include <memory>

namespace kaitai {
    class kstrin : public kspanin {
        std::unique_ptr<std::string> _data;

    public:
        explicit inline kstrin(std::unique_ptr<std::string> data) :
            _data(std::move(data)),
            kspanin(std::span(reinterpret_cast<std::uint8_t*>(&(*data)[0]), data->size())) {

        }

        explicit inline kstrin(std::string const& data) :
            kstrin(std::make_unique<std::string>(data))
        {}
    };
}