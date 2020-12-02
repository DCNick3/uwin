//
// Created by dcnick3 on 11/30/20.
//


#include "gtest/gtest.h"

#include <set>

#include "mem/mgr/target_mem_mgr.h"
#include "ht/handlelike_allocator.h"
#include "ht/kobj.h"
#include "ht/kobjref.h"
#include "ht/handletable.h"

#include "ctx/dll.h"
#include "ctx/ldr.h"

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

struct mock_kobj : public uwin::ht::kobj {
    bool _destroy_called = false;
    ~mock_kobj() override {
        _destroy_called = true;
    }
};

TEST(HandleTable, HandlelikeAllocator) {
    uwin::ht::handlelike_allocator allocator;

    const int count = 100;

    std::set<uwin::ht::handle_tvalue> generated;

    for (int i = 0; i < count; i++) {
        auto val = allocator.allocate();

        ASSERT_GE(val, uwin::mem::mgr::consts::address_space_reserved_start);
        ASSERT_LT(val, uwin::mem::mgr::consts::address_space_size);

        generated.emplace(val);
    }

    ASSERT_EQ(generated.size(), count);
}

TEST(HandleTable, HandleTable) {
    uwin::ht::handlelike_allocator allocator;

    {
        uwin::ht::handletable ht(allocator);
        auto obj1 = std::make_shared<mock_kobj>();
        auto h1_1 = ht.put(obj1);
        auto h1_2 = ht.clone(h1_1);

        ASSERT_EQ(obj1.get(), ht.get(h1_1).get());
        ASSERT_EQ(obj1.get(), ht.get(h1_2).get());
        ASSERT_EQ(obj1.use_count(), 3);

        ht.close(h1_1);

        ASSERT_THROW((void)ht.get(h1_1), uwin::win32::error);
        ASSERT_EQ(ht.try_get(h1_1).get(), nullptr);
        ASSERT_EQ(obj1.get(), ht.get(h1_2).get());
        ASSERT_EQ(obj1.use_count(), 2);

        ht.close(h1_2);

        ASSERT_THROW((void)ht.get(h1_2), uwin::win32::error);
        ASSERT_EQ(ht.try_get(h1_2), nullptr);
        ASSERT_EQ(obj1.use_count(), 1);
        obj1.reset();

        auto h2 = ht.emplace<mock_kobj>();
        auto h3 = ht.put(std::make_shared<mock_kobj>());
        ASSERT_NE(ht.get(h2).get(), ht.get(h3).get());
        ht.close(h2);
        ht.close(h3);
        ASSERT_THROW((void)ht.get(h2), uwin::win32::error);
        ASSERT_THROW((void)ht.get(h3), uwin::win32::error);
        ASSERT_EQ(ht.try_get(h2).get(), nullptr);
        ASSERT_EQ(ht.try_get(h3).get(), nullptr);
    }
}