//
// Created by dcnick3 on 12/1/20.
//


#include "gtest/gtest.h"

#include "heap/block.h"
#include "heap/span.h"
#include "heap/consts.h"
#include "heap/span_group.h"
#include "heap/heap.h"

#include <random>
#include <span>

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

template<typename Generator>
auto randfill(Generator& g, std::span<std::uint8_t> buffer) {
    //std::uniform_int_distribution<std::uint8_t> dist;
    memset(&*buffer.begin(), 0xcc, buffer.size_bytes());
}

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
        ASSERT_TRUE(span.empty());
        auto ptr1 = span.alloc(0x10);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_EQ(&span.from_ptr(mgr, ptr1), &span);
        ASSERT_FALSE(span.empty());

        auto ptr2 = span.alloc(0x20);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_EQ(&span.from_ptr(mgr, ptr2), &span);
        ASSERT_FALSE(span.empty());

        auto ptr3 = span.alloc(0x30);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_EQ(&span.from_ptr(mgr, ptr3), &span);
        ASSERT_FALSE(span.empty());

        span.free(ptr1);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_FALSE(span.empty());

        auto ptr4 = span.alloc(0x8);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_FALSE(span.empty());
        ASSERT_EQ(&span.from_ptr(mgr, ptr4), &span);

        span.free(ptr2);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_FALSE(span.empty());

        span.free(ptr3);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_FALSE(span.empty());

        span.free(ptr4);
        ASSERT_EQ(span.max_hole_size(), span.compute_max_hole_size());
        ASSERT_TRUE(span.empty());
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
            ASSERT_EQ(&span.from_ptr(mgr, addr), &span);
            ASSERT_EQ(span.size(addr), uwin::util::align_up(size, uwin::heap::consts::block_allocation_granularity));
            addresses.push_back(addr);
            address_set.insert(addr);
        };

        for (int i = 0; i < 10000; i++) {
            if (addresses.empty()) {
                ASSERT_TRUE(span.empty());
                alloc();
            } else if (uwin::util::align_down(span.compute_max_hole_size(),
                          uwin::heap::consts::block_allocation_granularity) == 0) {
                ASSERT_FALSE(span.empty());
                free();
            } else {
                ASSERT_FALSE(span.empty());
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

TEST(Heap, SpanGroup) {
    // A bit of fuzzing would not hurt, right?

    auto mapper = uwin::mem::create_host_mem_mapper();
    uwin::mem::mgr::target_mem_mgr mgr(mapper);
    {
        uwin::heap::span_group span_group(mgr);

        std::vector<uwin::mem::taddr> addresses;
        std::set<uwin::mem::taddr> address_set;

        std::default_random_engine eng(42); // NOLINT(cert-msc51-cpp)

        std::size_t max_memory_usage = 64 * 1024 * 1024; // 64 MiB
        std::size_t current_memory_usage = 0;

        auto free = [&]() {
            auto index = std::uniform_int_distribution<int>(0, addresses.size() - 1)(eng);
            auto addr = addresses[index];
            auto size = span_group.size(addr);
            randfill(eng, std::span(mgr.ptr(addr), size));
            ASSERT_EQ(span_group.size(addr), size);
            span_group.free(addr);
            current_memory_usage -= size;
            addresses.erase(addresses.begin() + index);
            address_set.erase(addr);
        };

        auto alloc = [&]() {
            uwin::mem::taddr::tvalue max_size = uwin::util::align_down(max_memory_usage - current_memory_usage,
                                                                       uwin::heap::consts::block_allocation_granularity);
            max_size = std::min(uwin::heap::consts::large_block_boundary, max_size);

            auto size = std::uniform_int_distribution<int>(1, max_size)(eng);
            auto addr = span_group.alloc(size);
            randfill(eng, std::span(mgr.ptr(addr), size));
            auto used_size = uwin::util::align_up(size, uwin::heap::consts::block_allocation_granularity);
            ASSERT_EQ(span_group.size(addr),
                      used_size);
            current_memory_usage += span_group.size(addr);
            addresses.push_back(addr);
            address_set.insert(addr);
        };

        for (int i = 0; i < 100000; i++) {
            if (addresses.empty()) {
                ASSERT_TRUE(span_group.empty());
                alloc();
            } else if (uwin::util::align_down(max_memory_usage - current_memory_usage,
                                              uwin::heap::consts::block_allocation_granularity) == 0) {
                ASSERT_FALSE(span_group.empty());
                free();
            } else {
                ASSERT_FALSE(span_group.empty());
                if (std::uniform_int_distribution<int>(0, 1)(eng) != 0)
                    alloc();
                else
                    free();
            }

            ASSERT_EQ(current_memory_usage, span_group.allocated_size());
            ASSERT_LE(span_group.allocated_size(), span_group.used_size());
        }
    }
}

TEST(Heap, Heap) {
    // A bit of fuzzing would not hurt, right?

    auto mapper = uwin::mem::create_host_mem_mapper();
    uwin::mem::mgr::target_mem_mgr mgr(mapper);
    {
        uwin::heap::heap heap(mgr, 0, 0);

        std::vector<uwin::mem::taddr> addresses;
        std::set<uwin::mem::taddr> address_set;

        std::default_random_engine eng(42); // NOLINT(cert-msc51-cpp)

        std::size_t max_memory_usage = 500 * 1024 * 1024; // 512 MiB
        std::size_t current_memory_usage = 0;

        auto free = [&]() {
            auto index = std::uniform_int_distribution<int>(0, addresses.size() - 1)(eng);
            auto addr = addresses[index];
            auto size = heap.size(addr);
            if (std::uniform_int_distribution<int>(0, 100)(eng) == 0)
                randfill(eng, std::span(mgr.ptr(addr), size));
            ASSERT_EQ(heap.size(addr), size);
            heap.free(addr);
            current_memory_usage -= size;
            addresses.erase(addresses.begin() + index);
            address_set.erase(addr);
        };

        auto alloc = [&]() {
            uwin::mem::taddr::tvalue max_size = uwin::util::align_down(max_memory_usage - current_memory_usage,
                                                                       uwin::heap::consts::block_allocation_granularity);

            auto size = std::uniform_int_distribution<int>(1, max_size)(eng);
            auto addr = heap.alloc(size);
            ASSERT_GE(heap.size(addr), size);
            if (std::uniform_int_distribution<int>(0, 100)(eng) == 0)
                randfill(eng, std::span(mgr.ptr(addr), size));
            ASSERT_GE(heap.size(addr), size);
            current_memory_usage += heap.size(addr);
            addresses.push_back(addr);
            address_set.insert(addr);
        };

        for (int i = 0; i < 10000; i++) {
            auto left_memory = uwin::util::align_down((ssize_t)max_memory_usage - (ssize_t)current_memory_usage,
                                                      uwin::heap::consts::block_allocation_granularity);
            if (addresses.empty()) {
                alloc();
            } else if (left_memory <= 0) {
                free();
            } else {
                if (std::uniform_int_distribution<int>(0, 1)(eng) != 0)
                    alloc();
                else
                    free();
            }
        }
    }
}