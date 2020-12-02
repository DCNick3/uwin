//
// Created by dcnick3 on 12/1/20.
//


#include "gtest/gtest.h"

#include "heap/block.h"
#include "heap/span.h"
#include "heap/consts.h"

#include <random>

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

TEST(Heap, Block) {
    uwin::heap::block block(0, 0);
    ASSERT_FALSE(block.is_used());
    ASSERT_EQ(block.size(), 0);

    block.set_size(1);
    ASSERT_EQ(block.size(), 1);
    ASSERT_FALSE(block.is_used());

    block.set_size(0);
    ASSERT_EQ(block.size(), 0);
    ASSERT_FALSE(block.is_used());

    block.set_used(true);
    ASSERT_EQ(block.size(), 0);
    ASSERT_TRUE(block.is_used());

    block.set_used(false);
    ASSERT_EQ(block.size(), 0);
    ASSERT_FALSE(block.is_used());

    block.set_size(0x93371488U);
    ASSERT_EQ(block.size(), 0x13371488U);
    ASSERT_FALSE(block.is_used());

    block.set(true, 0xf2281488U);
    ASSERT_EQ(block.size(), 0x72281488U);
    ASSERT_TRUE(block.is_used());
}

TEST(Heap, Span) {
    auto mapper = uwin::mem::create_host_mem_mapper();
    uwin::mem::mgr::target_mem_mgr mgr(mapper);
    {
        uwin::heap::span span(mgr);

        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        auto ptr1 = span.alloc(0x10);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        auto ptr2 = span.alloc(0x20);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        auto ptr3 = span.alloc(0x30);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        span.free(ptr1);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        auto ptr4 = span.alloc(0x8);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        span.free(ptr2);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        span.free(ptr3);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());

        span.free(ptr4);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
    }
    // A bit of fuzzing would not hurt, right?
    {
        uwin::heap::span span(mgr);

        std::vector<uwin::mem::taddr> addresses;
        std::set<uwin::mem::taddr> address_set;

        std::default_random_engine eng(42); // NOLINT(cert-msc51-cpp)

        auto free = [&]() {
            auto index = std::uniform_int_distribution<int>(0, addresses.size() - 1)(eng);
            auto addr = addresses[index];
            span.free(addr);
            ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
            addresses.erase(addresses.begin() + index);
            address_set.erase(addr);
        };

        auto alloc = [&]() {
            auto max_size = uwin::util::align_down(span.compute_max_hole_size(),
                                                   uwin::heap::consts::block_allocation_granularity);

            auto size = std::uniform_int_distribution<int>(1, max_size)(eng);
            auto addr = span.alloc(size);
            ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
            addresses.push_back(addr);
            address_set.insert(addr);
        };

        for (int i = 0; i < 10000; i++) {
            if (addresses.empty()) {
                alloc();
            } else if (uwin::util::align_down(span.compute_max_hole_size(),
                          uwin::heap::consts::block_allocation_granularity) == 0) {
                free();
            } else {
                if (std::uniform_int_distribution<int>(0, 1)(eng) != 0)
                    alloc();
                else
                    free();
            }

            auto cnt = 0;
            for (auto& b : span) {
                if (b.is_used()) {
                    ASSERT_TRUE(address_set.contains(b._data_address));
                    cnt++;
                }
            }
            ASSERT_EQ(cnt, address_set.size());
        }
    }

}