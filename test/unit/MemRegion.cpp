//
// Created by dcnick3 on 11/1/20.
//

#include "gtest/gtest.h"

#include "mem/mem_region.h"
#include "mem/mem_region.h"

using namespace uwin::mem;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

static bool tmem_intersects(taddr::tvalue base1, taddr::tvalue size1, taddr::tvalue base2, taddr::tvalue size2) {
    return tmem_region(base1, size1).intersects(tmem_region(base2, size2));
}

static bool tmem_does_contains(taddr::tvalue base1, taddr::tvalue size1, taddr::tvalue base2, taddr::tvalue size2) {
    return tmem_region(base1, size1).does_contain(tmem_region(base2, size2));
}

static bool hmem_intersects(uintptr_t base1, size_t size1, uintptr_t base2, size_t size2) {
    return hmem_region((uint8_t*)(base1), size1).intersects(hmem_region((uint8_t*)base2, size2));
}

TEST(MemRegion, TmemIntersection) {
    ASSERT_TRUE(!tmem_intersects(0, 1, 1, 1));
    ASSERT_TRUE( tmem_intersects(0, 1, 0, 1));
    ASSERT_TRUE(!tmem_intersects(1, 1, 0, 1));
    ASSERT_TRUE( tmem_intersects(0, 2, 1, 1));
    ASSERT_TRUE( tmem_intersects(1, 2, 2, 1));
    ASSERT_TRUE(!tmem_intersects(1, 2, 3, 1));
}

TEST(MemRegion, TmemDoesContain) {
    ASSERT_TRUE(!tmem_does_contains(0, 1, 1, 1));
    ASSERT_TRUE( tmem_does_contains(0, 1, 0, 1));
    ASSERT_TRUE(!tmem_does_contains(1, 2, 0, 3));
    ASSERT_TRUE( tmem_does_contains(1, 2, 1, 1));
    ASSERT_TRUE( tmem_does_contains(1, 2, 2, 1));
    ASSERT_TRUE(!tmem_does_contains(1, 2, 3, 1));
}

TEST(MemRegion, HmemIntersection) {
    ASSERT_TRUE(!hmem_intersects(0, 1, 1, 1));
    ASSERT_TRUE( hmem_intersects(0, 1, 0, 1));
    ASSERT_TRUE(!hmem_intersects(1, 1, 0, 1));
    ASSERT_TRUE( hmem_intersects(0, 2, 1, 1));
    ASSERT_TRUE( hmem_intersects(1, 2, 2, 1));
    ASSERT_TRUE(!hmem_intersects(1, 2, 3, 1));
}

TEST(MemRegion, TmemAlignGreedy) {
    ASSERT_EQ(tmem_region(0xfff, 2).align_greedy(0x1000), tmem_region(0, 0x2000));
    ASSERT_EQ(tmem_region(0xfff, 0x1).align_greedy(0x1000), tmem_region(0, 0x1000));
    ASSERT_EQ(tmem_region(0, 0x1000).align_greedy(0x1000), tmem_region(0, 0x1000));
    ASSERT_EQ(tmem_region(0, 0x1001).align_greedy(0x1000), tmem_region(0, 0x2000));
}