#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"
//
// Created by dcnick3 on 4/16/21.
//

#include "gtest/gtest.h"
#include "win32/dll/KERNEL32/impl.h"

using uwin::win32::dll::KERNEL32_impl;

TEST(Modules, NameNormalization) {
    auto test = [](const std::string& inp) {
        return KERNEL32_impl::normalize_module_name(inp);
    };

    ASSERT_EQ(test("kernel32"), "KERNEL32.DLL");
    ASSERT_EQ(test("kernel32."), "KERNEL32");
    ASSERT_EQ(test("kernel32.dll"), "KERNEL32.DLL");
    ASSERT_EQ(test("KERnel32.dll."), "KERNEL32.DLL");
    ASSERT_EQ(test("TEST.exe"), "TEST.EXE");
    ASSERT_EQ(test("TEST"), "TEST.DLL");
}

#pragma clang diagnostic pop