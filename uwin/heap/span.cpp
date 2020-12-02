//
// Created by dcnick3 on 12/1/20.
//

#include "heap/span.h"
#include "heap/consts.h"
#include "win32/error.h"
#include "util/except.h"

namespace uwin::heap {

    span::iterator span::ptr_to_iterator(mem::taddr ptr) const {
        // a small bit of voodoo magic is used here
        // (basically from https://stackoverflow.com/questions/44929500/how-to-get-a-stdlisttiterator-from-an-element-of-that-list)
        // TODO: implement own linked list not to deal with this scary stuff

        // This does not work for clang (as reinterpret_cast is not necessarily constant)
        //static_assert(reinterpret_cast<size_t>(nullptr) == 0,
        //               "In this compiler, nullptr is not 0 and this will not work");

        iterator it{nullptr};
        auto iteroffset = reinterpret_cast<size_t>(&*it);

        auto mypointer = _holder.get_mgr()->ptr((ptr - sizeof(block_hdr)).as<block_hdr>())->p_block_obj;

        iterator ret_iter;
        *(reinterpret_cast<intptr_t*>(&ret_iter)) =
                reinterpret_cast<intptr_t>(mypointer) - iteroffset;

        return ret_iter;
    }

    mem::taddr span::iterator_to_ptr(span::iterator it) {
        return it->_data_address;
    }

    span::span(mem::mgr::region_holder memory_region)
            : _holder(std::move(memory_region)) {
        assert(_holder.get().size() == consts::span_size);

        auto ptr = _holder.get_mgr()->ptr(_holder.get().begin());

        auto span_header = new (ptr) span_hdr{this};

        auto block_ptr = ptr + span_header_size;

        auto& block_ref = _inner_container.emplace_back(
                _holder.get().begin() + span_header_size + block_header_size,
                consts::span_size - span_header_size - block_header_size);

        new (block_ptr) block_hdr{&block_ref};

        _max_hole_size = block_ref.size();
    }

    std::pair<span::iterator, mem::taddr::tvalue> span::best_fit(mem::taddr::tvalue size) {
        // perform (in O(1)) selection of first and second maximum hole and find a best fit
        // then return a new maximum hole size, if the best fit is taken
        auto best_fit = end();
        auto max_1 = end();
        auto max_2 = end();

        for (auto it = begin(); it != end(); ++it) {
            if (!it->is_used()) {
                if (it->size() >= size) {
                    if (best_fit == end() || best_fit->size() > it->size())
                        best_fit = it;
                }

                if (max_1 == end())
                    max_1 = it;
                else {
                    if (max_1->size() <= it->size()) {
                        max_2 = max_1;
                        max_1 = it;
                    } else {
                        if (max_2 == end() || max_2->size() <= it->size())
                            max_2 = it;
                    }
                }
            }
        }

        if (best_fit == end()) {
            // no match at all
            if (max_1 == end())
                // no holes
                return std::make_pair(end(), 0);
            else
                // hole does not change
                return std::make_pair(end(), max_1->size());
        } else {
            // there is a match
            if (best_fit != max_1)
                // we do not touch the biggest hole
                return std::make_pair(best_fit, max_1->size());
            else {
                // we do touch the biggest hole. The candidates to another biggest hole are second biggest hole and the residue space

                auto shrinked_size = best_fit->size() - block_header_size - size;
                // overflow check
                if (best_fit->size() < block_header_size + size)
                    shrinked_size = 0;

                if (max_2 == end())
                    return std::make_pair(best_fit, shrinked_size);

                return std::make_pair(best_fit, std::max(std::size_t(max_2->size()), shrinked_size));
            }
        }
    }

    span::span(mem::mgr::target_mem_mgr& mem_mgr)
        : span(mem::mgr::region_holder::reserve_and_commit(mem_mgr, consts::span_size, mem::mgr::tprot::rw)) {
    }

    span::iterator span::merge_blocks(span::iterator one, span::iterator another) {
        {
            auto one1 = one; one1++;
            assert(one1 == another);
            assert(one->_data_address + one->size() + block_header_size == another->_data_address);
        }

        assert(!another->is_used());

        one->set_size(one->size() + block_header_size + another->size());
        _inner_container.erase(another);

        return one;
    }

    std::pair<span::iterator, span::iterator> span::split_blocks(span::iterator one, mem::taddr::tvalue first_data_size) {
        assert(one->size() >= first_data_size + block_header_size);

        auto second_data_address = one->_data_address + block_header_size + first_data_size;
        auto second_size = one->size() - first_data_size - block_header_size;

        one->set_size(first_data_size);

        auto next = one; ++next;
        auto new_it = _inner_container.emplace(next, second_data_address, second_size);

        auto ptr = _holder.get_mgr()->ptr(one->_data_address + one->size());

        new (ptr) block_hdr{&*new_it};

        return std::make_pair(one, new_it);
    }

    mem::taddr span::alloc(mem::taddr::tvalue size) {
        size = util::align_up(size, consts::block_allocation_granularity);

        auto r = best_fit(size);

        if (r.first == end())
            throw win32::error(win32::error_code::ERROR_NOT_ENOUGH_MEMORY);

        assert(!r.first->is_used());
        assert(r.first->size() >= size);

        if (r.first->size() != size) {
            split_blocks(r.first, size);
        }

        _max_hole_size = r.second;

        r.first->set_used(true);

        return iterator_to_ptr(r.first);
    }

    void span::free(mem::taddr ptr) {
        auto it = ptr_to_iterator(ptr);

        it->set_used(false);

        if (it != begin()) {
            auto prev = it; --prev;
            if (!prev->is_used()) {
                it = merge_blocks(prev, it);
            }
        }

        auto next = it; ++next;
        if (next != end()) {
            if (!next->is_used()) {
                it = merge_blocks(it, next);
            }
        }

        _max_hole_size = std::max(_max_hole_size, it->size());
    }

    mem::taddr::tvalue span::compute_max_hole_size() const {
        mem::taddr::tvalue res(0);
        for (auto& block : _inner_container) {
            if (!block.is_used())
                res = std::max(res, block.size());
        }
        return res;
    }
}
