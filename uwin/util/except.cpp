//
// Created by dcnick3 on 10/28/20.
//

#include "util/except.h"

#if defined(__GNUC__) || defined(__clang_major__)

#include <cxxabi.h>

#endif

namespace uwin::util {
    std::string get_nice_current_exception_type_name() {
#if (defined(__GNUC__) || defined(__clang_major__))
        int status;
        auto ptr = abi::__cxa_demangle(abi::__cxa_current_exception_type()->name(), nullptr, nullptr, &status);
        if (status != 0)
            std::terminate();
        std::string res(ptr);
        free(ptr);
        return std::move(res);
#else
        return typeid(std::current_exception()).name();
#endif
    }
}
