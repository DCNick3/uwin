//
// Created by dcnick3 on 12/2/20.
//

#pragma once

#include <cstdint>
#include <type_traits>

namespace uwin::heap {
    class block;

    struct [[gnu::packed]] block_hdr {
        block* p_block_obj;
    };

    static_assert(std::is_trivially_destructible_v<block_hdr>, "block_hdr should not requre a destructor");
}