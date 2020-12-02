//
// Created by dcnick3 on 12/1/20.
//

#pragma once

#include "mem/tptr.h"
#include <limits>

namespace uwin::heap {
    struct block {
        mem::taddr _data_address;
        // MSB of size is used bit
        mem::taddr::tvalue _size_and_used;

        inline block(mem::taddr data_address, mem::taddr::tvalue size)
            : _data_address(data_address), _size_and_used() {
            set(false, size);
        }

        static constexpr std::size_t used_bit_pos = std::numeric_limits<decltype(_size_and_used)>::digits - 1U;

        [[nodiscard]] inline bool is_used() const {
            return (_size_and_used >> used_bit_pos) != 0;
        }

        [[nodiscard]] inline mem::taddr::tvalue size() const {
            return _size_and_used & ~(1U << used_bit_pos);
        }

        inline void set_size(mem::taddr::tvalue new_size) {
            _size_and_used = new_size & ~(1U << used_bit_pos)
                             |
                             _size_and_used & (1U << used_bit_pos);
        }

        inline void set_used(bool used) {
            if (used) {
                _size_and_used |= (1U << used_bit_pos);
            } else {
                _size_and_used &= ~(1U << used_bit_pos);
            }
        }

        inline void set(bool used, mem::taddr::tvalue new_size) {
            _size_and_used = (used * (1U << used_bit_pos))
                             |
                    new_size & ~(1U << used_bit_pos);
        }
    };
}
