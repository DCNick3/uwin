//
// Created by dcnick3 on 12/1/20.
//

#pragma once

#include "heap/block_hdr.h"
#include "heap/span_hdr.h"
#include "mem/mgr/consts.h"
#include "util/align.h"

namespace uwin::heap::consts {

    static constexpr mem::taddr::tvalue large_block_boundary = 0x7fff0; // no exactly the same as windows, but, i guess, it works
    static constexpr mem::taddr::tvalue span_size = 0x80000;

    static constexpr mem::taddr::tvalue block_allocation_granularity = 8;

    static_assert(span_size - large_block_boundary >= sizeof(block_hdr), "block header won't fit");
    static_assert(std::is_standard_layout_v<block_hdr> &&
                  std::is_trivially_constructible_v<block_hdr>, "Expected block header to be POD");
    static_assert(util::is_aligned(span_size, mem::mgr::consts::page_size), "Span size must be page-aligned");

    static_assert(util::is_aligned(span_size, block_allocation_granularity),
                  "span should contain a whole number of blocks");
    static_assert(util::is_aligned(block_allocation_granularity, sizeof(block_hdr)),
                  "block should contain a whole number of block headers");
    static_assert(util::is_aligned(span_size - sizeof(span_hdr), sizeof(block_hdr)),
            "span should tightly fit a span header and a whole number of block headers");

}