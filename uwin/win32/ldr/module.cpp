//
// Created by dcnick3 on 11/15/20.
//

#include "win32/ldr/module.h"

namespace uwin::win32::ldr {

    mem::tcaddr module::try_resolve(const std::string &symbol_name) const {
        throw loader_exception("not implemented");
    }
}
