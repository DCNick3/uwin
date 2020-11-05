//
// Created by dcnick3 on 11/2/20.
//


#include "gtest/gtest.h"

#include "mem/mgr/pages_region.h"

using namespace uwin::mem;
using namespace uwin::mem::mgr;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"


TEST(PagesRegion, Basic) {
    tmem_region trg(0x20000, 0x10000);
    pages_region prg(trg);

    ASSERT_TRUE(prg.has_uncommited_pages(trg));
    prg.commit_pages(trg, tprot::rw);
    ASSERT_FALSE(prg.has_uncommited_pages(trg));
    prg.uncommit_pages(tmem_region(0x21000, 0x1000));
    ASSERT_TRUE(prg.has_uncommited_pages(trg));
    ASSERT_FALSE(prg.has_uncommited_pages(tmem_region(0x20000, 0x1000)));
    ASSERT_FALSE(prg.has_uncommited_pages(tmem_region(0x22000, 0xe000)));

    prg.reprotect_pages(tmem_region(0x22000, 0xe000), tprot::r);
    prg.uncommit_pages(trg);
    ASSERT_TRUE(prg.has_uncommited_pages(trg));
}
