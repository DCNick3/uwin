//
// Created by dcnick3 on 12/1/20.
//

#pragma once

#include "heap/block.h"
#include "heap/block_hdr.h"
#include "heap/span_hdr.h"
#include "mem/mgr/target_mem_mgr.h"
#include "mem/mgr/region_holder.h"
#include "util/nocopy.h"

#include <list>

namespace uwin::heap {
    class span : util::nocopy, util::nomove {
    public:
        typedef std::list<block> inner_container;
        typedef inner_container::iterator iterator;
        typedef inner_container::const_iterator const_iterator;

    private:
        mem::taddr::tvalue _max_hole_size;
        mem::mgr::region_holder _holder;

        inner_container _inner_container;

        // these do not update the _max_hole_size
        iterator merge_blocks(iterator one, iterator another);
        std::pair<iterator, iterator> split_blocks(iterator one, mem::taddr::tvalue first_data_size);

        static constexpr std::size_t block_header_size = sizeof(block_hdr);
        static constexpr std::size_t span_header_size = sizeof(span_hdr);

        [[nodiscard]] iterator ptr_to_iterator(mem::taddr ptr) const;
        [[nodiscard]] static mem::taddr iterator_to_ptr(iterator it);

    public:
        explicit span(mem::mgr::region_holder memory_region);
        explicit span(mem::mgr::target_mem_mgr& mem_mgr);

        [[nodiscard]] inline iterator begin() { return _inner_container.begin(); }
        [[nodiscard]] inline iterator end() { return _inner_container.end(); }

        [[nodiscard]] inline const_iterator begin() const { return _inner_container.begin(); }
        [[nodiscard]] inline const_iterator end() const { return _inner_container.end(); }

        [[nodiscard]] std::pair<iterator, mem::taddr::tvalue> best_fit(mem::taddr::tvalue size);

        [[nodiscard]] mem::taddr alloc(mem::taddr::tvalue size);
        void free(mem::taddr ptr);

        [[nodiscard]] inline mem::taddr::tvalue max_hole_size() const {
            return _max_hole_size;
        }

        [[nodiscard]] mem::taddr::tvalue compute_max_hole_size() const;
    };

}
