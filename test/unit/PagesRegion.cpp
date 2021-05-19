//
// Created by dcnick3 on 11/2/20.
//


#include "gtest/gtest.h"

#include "mem/mgr/pages_region.h"
#include "util/visit.h"

using namespace uwin;
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

    // check a query that does not begin with the region returned, but the returned region is the same as reserved region
    auto q = prg.query(taddr(0x23000));
    util::visit(q,
                [&](query_results::reserved const& r) {
                    FAIL() << "should have got committed result";
                }, [&](query_results::committed const& c) {
                ASSERT_EQ(c.region, trg);
                ASSERT_EQ(c.base_region, trg);
                ASSERT_EQ(c.protection, tprot::rw);
            });

    prg.uncommit_pages(tmem_region(0x21000, 0x1000));
    ASSERT_TRUE(prg.has_uncommited_pages(trg));
    ASSERT_FALSE(prg.has_uncommited_pages(tmem_region(0x20000, 0x1000)));
    ASSERT_FALSE(prg.has_uncommited_pages(tmem_region(0x22000, 0xe000)));

    // check a query that does not begin with the region returned, returned region != reserved region
    q = prg.query(taddr(0x23000));
    util::visit(q,
                [&](query_results::reserved const& r) {
        FAIL() << "should have got committed result";
    }, [&](query_results::committed const& c) {
        ASSERT_EQ(c.region, tmem_region(0x22000, 0xe000));
        ASSERT_EQ(c.base_region, trg);
        ASSERT_EQ(c.protection, tprot::rw);
    });

    prg.reprotect_pages(tmem_region(0x22000, 0xe000), tprot::r);
    prg.uncommit_pages(trg);
    ASSERT_TRUE(prg.has_uncommited_pages(trg));
}
