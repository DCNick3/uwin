//
// Created by dcnick3 on 11/14/20.
//


#include "impl.h"
#include "log/log.h"

namespace uwin::win32::dll {


    std::int32_t USER32_impl::MessageBoxA(uwin::ht::handle<uwin::win32::types::wnd> hWnd, uwin::mem::tcptr<char> lpText,
                             uwin::mem::tcptr<char> lpCaption, std::uint32_t uType) {
        auto text = nstr(lpText);
        auto caption = nstr(lpCaption);

        return static_cast<std::int32_t>(handle_error(svc::mbox::RESULT::ERROR, [&]() {
            return _mbox.show(text, caption, static_cast<svc::mbox::TYPE>(uType));
        }));
        /*svc::mbox::RESULT res;
        _current_thread->set_last_error(error_code::ERROR_SUCCESS);
        try {
            res =
        }
        catch (win32::error const& e) {
            _current_thread->set_last_error(e.code());
        }

        return static_cast<std::int32_t>(res);*/
    }
}