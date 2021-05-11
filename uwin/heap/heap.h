//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "ht/kobj.h"
#include "mem/tptr.h"
#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "str/str.h"
#include "heap/consts.h"
#include "heap/span_group.h"

#include <type_traits>
#include <set>

namespace uwin::heap {
    class heap : public ht::kobj {

        mem::mgr::target_mem_mgr& _mem_mgr;
        span_group _span_group; // TODO: create separate span groups for different sizes

        std::set<mem::taddr> _raw_allocations;

    public:
        inline heap(mem::mgr::target_mem_mgr& mem_mgr, mem::taddr::tvalue initial_size, mem::taddr::tvalue maximum_size)
            : _mem_mgr(mem_mgr), _span_group(_mem_mgr) {
            if (maximum_size != 0)
                throw util::not_implemented_error("Non-growable heap");
            assert(util::is_aligned(initial_size, mem::mgr::consts::page_size));
            assert(util::is_aligned(maximum_size, mem::mgr::consts::page_size));
            // TODO: do not ignore initial_size
        }

        mem::taddr alloc(mem::taddr::tvalue size);

        mem::taddr realloc(mem::taddr old_ptr, mem::taddr::tvalue new_size);

        inline mem::taddr::tvalue size(mem::taddr ptr) {
            return span_group::size(_mem_mgr, ptr);
        }

        void free(mem::taddr ptr);


        [[nodiscard]] inline mem::tptr<char> alloc_str(str::narrow const& s) {
            auto tptr = alloc(s.size() + 1);
            auto hptr = _mem_mgr.ptr(tptr);
            memcpy(hptr, s.c_str(), s.size() + 1);
            return tptr.as<char>();
        }
        [[nodiscard]] inline mem::tptr<char16_t> alloc_str(str::wide const& s) {
            static_assert(sizeof(char16_t) == 2);
            auto tptr = alloc(s.size() * 2 + 2);
            auto hptr = _mem_mgr.ptr(tptr);
            memcpy(hptr, s.c_str(), s.size() + 2);
            return tptr.as<char16_t>();
        }

        [[nodiscard]] inline mem::tptr<std::uint8_t> alloc_bytes(std::initializer_list<std::uint8_t> l) {
            auto tptr = alloc(l.size());
            auto hptr = _mem_mgr.ptr(tptr);
            for (auto v : l) {
                *hptr = v;
                hptr++;
            }
            return tptr;
        }

        [[nodiscard]] inline mem::tptr<std::uint8_t> alloc_bytes(std::vector<std::uint8_t> const& l) {
            auto tptr = alloc(l.size());
            auto hptr = _mem_mgr.ptr(tptr);
            for (auto v : l) {
                *hptr = v;
                hptr++;
            }
            return tptr;
        }

        ~heap() override;
    };
}