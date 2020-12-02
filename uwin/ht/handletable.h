//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "win32/types/types.h"
#include "win32/error.h"
#include "ht/kobj.h"
#include "ht/handlelike_allocator.h"

#include <map>
#include <memory>

namespace uwin::ht {
    class handletable {
        handlelike_allocator& _handlelike_allocator;
        std::map<handle_tvalue, std::shared_ptr<kobj>> _table;

    public:
        inline explicit handletable(handlelike_allocator &handlelike_allocator) :
                    _handlelike_allocator(handlelike_allocator)
        {}

        template<typename T>
        [[nodiscard]] std::shared_ptr<T> try_get(handle<T> handle) noexcept {
            static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");
            auto it = _table.find(handle.value());
            if (it == _table.end())
                return std::shared_ptr<T>(nullptr);
            auto res = std::dynamic_pointer_cast<T>(it->second);
            assert(res.get() != nullptr);
            return res;
        }

        template<typename T>
        [[nodiscard]] std::shared_ptr<T> get(handle<T> handle) {
            static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");
            auto res = try_get(handle);
            if (res.get() == nullptr)
                throw win32::error(win32::error_code::ERROR_INVALID_HANDLE);
            return res;
        }


        // note: put does not increase the refcount
        template<typename T>
        [[nodiscard]] handle<T> put(std::shared_ptr<T> obj) noexcept {
            static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");
            //assert(static_cast<kobj*>(obj)->refcount() == 1);

            auto handle = _handlelike_allocator.allocate();
            _table.insert(std::make_pair(handle, obj));

            return ht::handle<T>(handle);
        }

        template<typename T, typename... Args>
        [[nodiscard]] handle<T> emplace(Args&&... args) {
            return put(std::make_shared<T>(std::forward<Args>(args)...));
        }

        template<typename T>
        [[nodiscard]] handle<T> clone(handle<T> handle) {
            static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");

            // TODO: exception safety?

            return put(get(handle));
        }

        template<typename T>
        void close(handle<T> handle) {
            static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");

            _table.erase(handle.value());
        }

    };
}