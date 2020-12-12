//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include "mem/mgr/target_mem_mgr.h"
#include "mem/tptr.h"
#include "ht/handlelike_allocator.h"
#include "ht/handletable.h"
#include "ht/handle_holder.h"
#include "ctx/thread.h"
#include "heap/heap.h"

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
        ht::handle_holder<heap::heap> _process_heap;
        mem::tptr<char> _command_line{0};
        std::unique_ptr<thread> _current_thread;
        std::vector<std::uint8_t> _environment{0};

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
        [[nodiscard]] inline std::shared_ptr<T> ht_try_get(ht::handle<T> handle) const { return _handle_table->try_get(handle); }
        template<typename T>
        [[nodiscard]] inline std::shared_ptr<T> ht_get(ht::handle<T> handle) const { return _handle_table->get(handle); }
        template<typename T>
        [[nodiscard]] inline ht::handle<T> ht_put(std::shared_ptr<T> obj) const { return _handle_table->put(obj); }
        template<typename T, typename... Args>
        [[nodiscard]] ht::handle<T> ht_emplace(Args&&... args) const { return _handle_table->emplace<T>(std::forward<Args>(args)...); }
        template<typename T>
        [[nodiscard]] inline ht::handle<T> ht_clone(ht::handle<T> handle) const { return _handle_table->clone(handle); }
        template<typename T>
        inline void ht_close(ht::handle<T> handle) const { return _handle_table->close(handle); }

        [[nodiscard]] inline mem::tptr<char> alloc_str(std::string const& s) const {
            auto tptr = _process_heap->alloc(s.size() + 1);
            auto hptr = _mem_mgr->ptr(tptr);
            memcpy(hptr, s.c_str(), s.size() + 1);
            return tptr.as<char>();
        }

        [[nodiscard]] inline mem::tptr<std::uint8_t> alloc_bytes(std::initializer_list<std::uint8_t> l) const {
            auto tptr = _process_heap->alloc(l.size());
            auto hptr = _mem_mgr->ptr(tptr);
            for (auto v : l) {
                *hptr = v;
                hptr++;
            }
            return tptr;
        }

        [[nodiscard]] inline mem::tptr<std::uint8_t> alloc_bytes(std::vector<std::uint8_t> const& l) const {
            auto tptr = _process_heap->alloc(l.size());
            auto hptr = _mem_mgr->ptr(tptr);
            for (auto v : l) {
                *hptr = v;
                hptr++;
            }
            return tptr;
        }

        inline void set_last_error(win32::error_code error_code) const {
            _current_thread->_win32_last_error = error_code;
        }

        [[nodiscard]] inline win32::error_code get_last_error() const {
            return _current_thread->_win32_last_error;
        }
    };
}