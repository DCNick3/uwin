//
// Created by dcnick3 on 11/11/20.
//

#include "win32/dll/stub_exception.h"
namespace uwin {
    namespace win32 {
        namespace dll {

            const char *stub_exception::what() const noexcept {
                return _what.c_str();
            }
        }
    }
}

