
#include "win32/ldr/module_table.h"

namespace uwin::win32::ldr {

    void module_table::register_native_module(dll::native_module &native_module) {
        auto it = _modules.emplace(std::ref(native_module)).first;
        _name_to_module.emplace(std::get<0>(*it).get().name(), it);
    }

    module *module_table::try_get_module(types::hmodule hmodule) {
        auto it = _modules.find(hmodule);
        if (it == _modules.end())
            return nullptr;
        // here we change const'ness of an std::set reference
        // but is's okay, because it only needs to ensure that the value of key would not change
        // and it is part of ldr::module contract not to change the hmodule (does not make much sense anyway)
        return &get_module(const_cast<module_entry&>(*it));
    }

    module *module_table::try_get_module(const str::native& name) {
        auto it = _name_to_module.find(name);
        if (it == _name_to_module.end())
            return nullptr;
        // same reasoning about changing constness as above
        return &get_module(const_cast<module_entry&>(*it->second));
    }

    win32::ldr::module &module_table::get_module(const str::native& name) {
        auto res = try_get_module(name);
        if (res == nullptr)
            throw uwin::win32::ldr::loader_exception(fmt::format("can't find module with name {}", name));
        return *res;
    }

    win32::ldr::module &module_table::get_module(types::hmodule hmodule) {
        auto res = try_get_module(hmodule);
        if (res == nullptr)
            throw uwin::win32::ldr::loader_exception(fmt::format("can't find module with hmodule {}", hmodule));
        return *res;
    }
}

