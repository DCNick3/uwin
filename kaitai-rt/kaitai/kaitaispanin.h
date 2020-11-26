//
// Created by dcnick3 on 11/25/20.
//

#pragma once

#include <kaitai/kaitaiin.h>

namespace kaitai {
    class kspanin : public kin {
        std::span<const std::uint8_t> _data;
        std::span<const std::uint8_t> _left;

    public:
        explicit kspanin(std::span<const std::uint8_t> data);

        void close() override;

        [[nodiscard]] bool is_eof() const override;

        void seek(uint64_t pos) override;

        uint64_t pos() override;

        uint64_t size() override;

        void read(std::span<std::uint8_t> target) override;
    };
}