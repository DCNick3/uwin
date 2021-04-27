//
// Created by dcnick3 on 11/3/20.
//


#include "gtest/gtest.h"

#include "win32/error.h"

using namespace uwin::win32;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

TEST(Win32Error, Basic) {
    ASSERT_STREQ(error_code_to_slug(error_code::ERROR_ACCESS_DENIED), "ERROR_ACCESS_DENIED");
    ASSERT_STREQ(error_code_to_desc(error_code::ERROR_ACCESS_DENIED), "Access is denied.");
    try {
        throw error(error_code::ERROR_ACCESS_DENIED);
    } catch (error const& e) {
        ASSERT_EQ(e.code(), error_code::ERROR_ACCESS_DENIED);
    }
}