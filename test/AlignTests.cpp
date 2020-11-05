//
// Created by dcnick3 on 10/31/20.
//

#include "gtest/gtest.h"

#include "util/align.h"
#include "mem/tptr.h"

using namespace uwin::util;
using namespace uwin::mem;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

TEST(AlignTests, AlignIntDown) {
    ASSERT_EQ(align_down(0, 1), 0);
    ASSERT_EQ(align_down(0, 12), 0);
    ASSERT_EQ(align_down(1, 1), 1);
    ASSERT_EQ(align_down(12, 1), 12);
    ASSERT_EQ(align_down(11, 12), 0);
    ASSERT_EQ(align_down(17, 16), 16);
}

TEST(AlignTests, AlignIntUp) {
    ASSERT_EQ(align_up(0, 1), 0);
    ASSERT_EQ(align_up(0, 12), 0);
    ASSERT_EQ(align_up(1, 1), 1);
    ASSERT_EQ(align_up(12, 1), 12);
    ASSERT_EQ(align_up(11, 12), 12);
    ASSERT_EQ(align_up(17, 16), 32);
}

TEST(AlignTests, AlignTaddrDown) {
    ASSERT_EQ(align_down(taddr(0), 1), taddr(0));
    ASSERT_EQ(align_down(taddr(0), 12), taddr(0));
    ASSERT_EQ(align_down(taddr(1), 1), taddr(1));
    ASSERT_EQ(align_down(taddr(12), 1), taddr(12));
    ASSERT_EQ(align_down(taddr(11), 12), taddr(0));
    ASSERT_EQ(align_down(taddr(17), 16), taddr(16));
}

TEST(AlignTests, AlignTaddrUp) {
    ASSERT_EQ(align_up(taddr(0), 1), taddr(0));
    ASSERT_EQ(align_up(taddr(0), 12), taddr(0));
    ASSERT_EQ(align_up(taddr(1), 1), taddr(1));
    ASSERT_EQ(align_up(taddr(12), 1), taddr(12));
    ASSERT_EQ(align_up(taddr(11), 12), taddr(12));
    ASSERT_EQ(align_up(taddr(17), 16), taddr(32));
}
