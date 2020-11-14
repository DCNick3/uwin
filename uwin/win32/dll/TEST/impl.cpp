//
// Created by dcnick3 on 11/14/20.
//


#include "impl.h"

namespace uwin::win32::dll {

    int32_t dll::TEST_impl::MessageBoxA(uwin::win32::types::handle<uwin::win32::types::wnd> hWnd,
                                        uwin::mem::tcptr<char> lpText, uwin::mem::tcptr<char> lpCaption,
                                        std::uint32_t uType) {
        auto text = str(lpText);
        auto caption = str(lpCaption);


        return 0;
    }
}