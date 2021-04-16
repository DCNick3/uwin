
#include "win32/ldr/target_module.h"
#include "win32/types/types.h"

namespace uwin::win32::ldr {

    mem::tcaddr target_module::try_resolve(const std::string &symbol_name) const {
        throw loader_exception("not implemented");
    }

    types::hmodule target_module::handle() const {
        return _image_region.begin().as_const().as<types::module_tag>();
    }
}
