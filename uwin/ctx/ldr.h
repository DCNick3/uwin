//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include "win32/ldr/linkable.h"
#include "win32/ldr/module.h"
#include "win32/ldr/except.h"

#include "ctx/dll.h"

#include <set>

namespace uwin::ctx {
    class ldr {
        dll& _dll_ctx;

        struct module_cmp {
            typedef std::true_type is_transparent;
            bool operator()(const std::unique_ptr<win32::ldr::module>& lhs, const std::string& rhs) const {
                return lhs->name() < rhs;
            }
            bool operator()(const std::string& lhs, const std::unique_ptr<win32::ldr::module>& rhs) const {
                return lhs < rhs->name();
            }
            bool operator()(const std::unique_ptr<win32::ldr::module>& lhs, const std::unique_ptr<win32::ldr::module>& rhs) const {
                return lhs->name() < rhs->name();
            }
        };

        std::set<std::unique_ptr<win32::ldr::module>, module_cmp> _loaded_modules;

    public:
        explicit ldr(dll& dll_ctx) : _dll_ctx(dll_ctx)
        {}

        inline win32::ldr::linkable* try_resolve(std::string name) {
            for (auto & c: name) c = std::toupper(c);
            auto native = _dll_ctx.try_resolve(
                    name.substr(0, name.find_last_of('.')) // FIXME: don't remove the extension if not .dll
                    );
            if (native != nullptr) return native;
            auto target = _loaded_modules.find(name);
            if (target != _loaded_modules.end())
                return target->get();
            return nullptr;
        }

        inline win32::ldr::linkable& resolve(const std::string& name) {
            auto res = try_resolve(name);
            if (res == nullptr)
                throw uwin::win32::ldr::loader_exception(fmt::format("can't find linkable for {}", name));
            return *res;
        }

        template<typename ...Args>
        inline win32::ldr::module& emplace_module(Args&&... args) {
            return **_loaded_modules.emplace(new win32::ldr::module(std::forward<Args>(args)...)).first;
        }
    };
}