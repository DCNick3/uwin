//
// Created by dcnick3 on 10/31/20.
//

#include "gtest/gtest.h"

#include "mem/mem_mapper.h"
#include "mem/mgr/pages_regions_container.h"
#include "mem/mgr/target_mem_mgr.h"
#include "util/align.h"
#include "win32/error.h"

using namespace uwin::mem::mgr;
using namespace uwin::mem;
using namespace uwin::util;
using namespace uwin;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

TEST(TargetMemMgr, TestRegionContainer) {

    pages_regions_container rmgr;

    auto reg1 = rmgr.alloc(0x1000);
    ASSERT_TRUE(is_aligned(reg1->begin(), consts::allocation_granularity));

    auto reg2 = rmgr.alloc(0x2000);
    ASSERT_TRUE(is_aligned(reg2->begin(), consts::allocation_granularity));

    auto reg3 = rmgr.alloc(0x3000);
    ASSERT_TRUE(is_aligned(reg3->begin(), consts::allocation_granularity));

    auto reg4 = rmgr.insert(tmem_region(align_up(reg3->end(), consts::allocation_granularity), 0x10000));
    ASSERT_TRUE(is_aligned(reg4->begin(), consts::allocation_granularity));

    ASSERT_EQ(reg4, rmgr.find_containing(tmem_region(reg4->begin() + 0x1000, 0x1000)));
    ASSERT_EQ(reg2, rmgr.find_containing(tmem_region(reg2->begin(), 0x1000)));
    ASSERT_EQ(reg2, rmgr.find_containing(tmem_region(reg2->end() - 0x1000, 0x1000)));

    ASSERT_EQ(reg1, rmgr.find_starting_with(reg1->begin()));
    ASSERT_EQ(reg2, rmgr.find_starting_with(reg2->begin()));
    ASSERT_EQ(reg3, rmgr.find_starting_with(reg3->begin()));
    ASSERT_EQ(reg4, rmgr.find_starting_with(reg4->begin()));
    ASSERT_EQ(rmgr.find_starting_with(0x1337 * consts::allocation_granularity), rmgr.end());

    int cnt = 0;
    for (auto& rg : rmgr) { cnt++; }
    ASSERT_EQ(cnt, rmgr.size());
    ASSERT_EQ(rmgr.size(), 4);

    rmgr.erase(reg1);

    ASSERT_EQ(rmgr.size(), 3);

    reg1 = rmgr.alloc(0x1000);
    ASSERT_TRUE(is_aligned(reg1->begin(), consts::allocation_granularity));

    ASSERT_EQ(rmgr.size(), 4);

    rmgr.erase(reg1);
    rmgr.erase(reg2);
    rmgr.erase(reg3);
    rmgr.erase(reg4);

    ASSERT_EQ(rmgr.size(), 0);
}

TEST(TargetMemMgr, Integration) {
    auto mem_mapper = create_host_mem_mapper();
    target_mem_mgr mgr(mem_mapper);
    auto rg = mgr.reserve_dynamic(consts::page_size * 100);

    mgr.commit(rg, tprot::rw);

    memset(mgr.guest_to_host(rg).begin(), 0xce, rg.size());

    mgr.reprotect(tmem_region(rg.begin(), consts::page_size * 3), tprot::r);

    mgr.uncommit(tmem_region(rg.begin() + consts::page_size, consts::page_size));
    mgr.uncommit(tmem_region(rg.begin() + consts::page_size, consts::page_size));

    ASSERT_THROW(mgr.reprotect(tmem_region(rg.begin(), consts::page_size * 3), tprot::r), win32::error);
    ASSERT_THROW(mgr.commit(tmem_region(rg.end(), consts::page_size), tprot::rw), win32::error);
    ASSERT_THROW(mgr.unreserve(rg.end()), win32::error); // align
    ASSERT_THROW(mgr.reserve_fixed(tmem_region(rg.end(), consts::page_size)), win32::error); // align
    ASSERT_NO_THROW(mgr.reserve_fixed(tmem_region(align_up(rg.end(), consts::allocation_granularity),
                   consts::page_size))); // align
    ASSERT_THROW(mgr.uncommit(tmem_region(rg.end(), consts::page_size)), win32::error);
    ASSERT_THROW(mgr.uncommit_whole_reserved_region(rg.end()), win32::error);
    ASSERT_THROW(mgr.reprotect(tmem_region(rg.end(), consts::page_size), tprot::rw), win32::error);

    mgr.uncommit_whole_reserved_region(rg.begin());

    mgr.unreserve(rg.begin());
}
