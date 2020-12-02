//
// Created by dcnick3 on 12/2/20.
//

#pragma once

#include <cstdint>
#include <type_traits>

namespace uwin::heap {
    class span;

    struct [[gnu::packed]] span_hdr {
        span* p_span_obj;
    };

    static_assert(std::is_trivially_destructible_v<span_hdr>, "block_hdr should not requre a destructor");
}