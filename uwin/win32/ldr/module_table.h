#pragma clang diagnostic push
#pragma ide diagnostic ignored "modernize-use-nullptr"
//
// Created by dcnick3 on 4/16/21.
//

#pragma once

#include <set>
#include <variant>
#include <functional>

#include "win32/dll/native_module.h"
#include "win32/ldr/target_module.h"

namespace uwin::win32::ldr {
    class module_table {
        typedef std::variant<
            std::reference_wrapper<dll::native_module>,
            std::unique_ptr<ldr::target_module>
        > module_entry;

        static inline std::string const& get_name(module_entry const& entry) {
            if (entry.index() == 0)
                return std::get<0>(entry).get().name();
            else if (entry.index() == 1)
                return std::get<1>(entry)->name();
            else
                std::terminate();
        }

        static inline types::hmodule get_hmodule(module_entry const& entry) {
            if (entry.index() == 0)
                return std::get<0>(entry).get().handle();
            else if (entry.index() == 1)
                return std::get<1>(entry)->handle();
            else
                std::terminate();
        }

        static inline module& get_module(module_entry& entry) {
            if (entry.index() == 0)
                return std::get<0>(entry).get();
            else if (entry.index() == 1)
                return *std::get<1>(entry);
            else
                std::terminate();
        }

        struct comp {
            typedef std::true_type is_transparent;
            inline bool operator()(module_entry const& lhs, types::hmodule const& rhs) const {
                return get_hmodule(lhs) < rhs;
            }
            inline bool operator()(types::hmodule const& lhs, module_entry const& rhs) const {
                return lhs < get_hmodule(rhs);
            }
            inline bool operator()(module_entry const& lhs, module_entry const& rhs) const {
                return get_hmodule(lhs) < get_hmodule(rhs);
            }
        };

        // module handle is the primary key, module name - secondary
        // TODO: how does windows 95 handle duplicate module names?
        std::set<module_entry, comp> _modules;
        std::map<std::string, decltype(_modules)::iterator> _name_to_module;

    public:
        inline module_table() = default;

        void register_native_module(dll::native_module& native_module);

        template<typename ...Args>
        target_module& emplace_target_module(Args&&... args) {
            auto it = _modules.emplace(std::make_unique<win32::ldr::target_module>(std::forward<Args>(args)...)).first;
            _name_to_module.emplace(std::get<1>(*it)->name(), it);
            return *std::get<1>(*it);
        }

        module* try_get_module(types::hmodule hmodule);
        module* try_get_module(std::string const& name);

        win32::ldr::module& get_module(const std::string& name);
        win32::ldr::module& get_module(types::hmodule hmodule);
    };
}
#pragma clang diagnostic pop