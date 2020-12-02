//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include "mem/mgr/target_mem_mgr.h"
#include "mem/tptr.h"
#include "ht/handlelike_allocator.h"
#include "ht/handletable.h"

namespace uwin::ctx {
    class dll;
    class ldr;

    class process {
    public:
        std::unique_ptr<mem::mgr::target_mem_mgr> _mem_mgr;
        std::unique_ptr<dll> _dll;
        std::unique_ptr<ldr> _ldr;
        std::unique_ptr<ht::handlelike_allocator> _handlelike_allocator;
        std::unique_ptr<ht::handletable> _handle_table;

        template<typename T>
        inline auto guest_to_host(mem::tptr<T> addr) const { return _mem_mgr->ptr(addr); }

        template<typename T>
        inline auto guest_to_host(mem::tcptr<T> addr) const { return _mem_mgr->ptr(addr); }

        [[nodiscard]] inline mem::hmem_region
        guest_to_host(mem::tmem_region const &region) const { return _mem_mgr->ptr(region); }

        template<typename T>
        inline T &deref(mem::tptr<T, false> ptr) const { return _mem_mgr->deref(ptr); }

        template<typename T>
        inline T const &deref(mem::tptr<T, true> ptr) const { return _mem_mgr->deref(ptr); }

        template<typename T>
        [[nodiscard]] inline std::shared_ptr<T> ht_try_get(ht::handle<T> handle) { return _handle_table->try_get(handle); }
        template<typename T>
        [[nodiscard]] inline std::shared_ptr<T> ht_get(ht::handle<T> handle) { return _handle_table->get(handle); }
        template<typename T>
        [[nodiscard]] inline ht::handle<T> ht_put(std::shared_ptr<T> obj) { return _handle_table->put(obj); }
        template<typename T, typename... Args>
        [[nodiscard]] ht::handle<T> ht_emplace(Args&&... args) { return _handle_table->emplace<T>(std::forward<Args>(args)...); }
        template<typename T>
        [[nodiscard]] inline ht::handle<T> ht_clone(ht::handle<T> handle) { return _handle_table->clone(handle); }
        template<typename T>
        inline void ht_close(ht::handle<T> handle) { return _handle_table->close(handle); }
    };
}