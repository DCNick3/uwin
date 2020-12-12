//
// Created by dcnick3 on 12/2/20.
//

#pragma once

#include "heap/span.h"
#include "util/nocopy.h"
#include "util/nomove.h"

#include <set>
#include <map>

namespace uwin::heap {
    class span_group : util::nocopy, util::nomove {

        struct sorter {
            typedef int is_transparent;
            bool operator()(span const& a, span const& b) const {
                return a.max_hole_size() < b.max_hole_size();
            }
            bool operator()(span const& a, mem::taddr::tvalue b) const {
                return a.max_hole_size() < b;
            }
        };

        std::multiset<span, sorter> _span_set;
        std::map<const span*, decltype(_span_set)::const_iterator> _span_iterators;
        mem::mgr::target_mem_mgr& _mem_mgr;
        mem::taddr::tvalue _allocated_size{};

    public:
        inline explicit span_group(mem::mgr::target_mem_mgr& mem_mgr)
            : _mem_mgr(mem_mgr) {}

        [[nodiscard]] mem::taddr alloc(mem::taddr::tvalue size);
        void free(mem::taddr ptr);
        [[nodiscard]] static inline mem::taddr::tvalue size(mem::mgr::target_mem_mgr& mem_mgr, mem::taddr ptr) {
            return span::size(mem_mgr, ptr);
        }
        [[nodiscard]] inline mem::taddr::tvalue size(mem::taddr ptr) {
            return size(_mem_mgr, ptr);
        }

        [[nodiscard]] inline bool empty() const { return _span_set.empty(); }
        [[nodiscard]] inline mem::taddr::tvalue allocated_size() const { return _allocated_size; }
        [[nodiscard]] inline mem::taddr::tvalue used_size() const { return _span_set.size() * consts::span_size; }
    };
}