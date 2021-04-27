//
// Created by dcnick3 on 4/23/21.
//

#pragma once

namespace uwin::win32::kobj {
    // it's a genetic object that can be closed with CloseHandle
    // (As opposed to the ones that can't, like FindFirstFile/FindClose. Those are not really handles in real dlls, but
    //  we want to model them as such, hence this distinction)
    class closable : public ht::kobj {

    };
}
