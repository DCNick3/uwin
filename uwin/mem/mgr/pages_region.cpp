//
// Created by dcnick3 on 10/31/20.
//

#include "mem/mgr/pages_region.h"
#include "win32/error.h"

namespace uwin::mem::mgr {

    static void throw_rg_alignment() {
        throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS,
                           "specified region has invalid alignment.");
    }

    bool pages_region::has_uncommited_pages(tmem_region region) const {
        if (!region.is_aligned(consts::page_size)) throw_rg_alignment();
        assert(does_contain(region));

        auto pages = region.relative_to(*this) / consts::page_size;
        for (auto i = pages.begin(); i < pages.end(); ++i) {
            if (!_page_states[i.value()].commited)
                return true;
        }
        return false;
    }

    bool pages_region::has_commited_pages(tmem_region region) const {
        if (!region.is_aligned(consts::page_size)) throw_rg_alignment();
        assert(does_contain(region));

        auto pages = region.relative_to(*this) / consts::page_size;
        for (auto i = pages.begin(); i < pages.end(); ++i) {
            if (_page_states[i.value()].commited)
                return true;
        }
        return false;
    }

    void pages_region::commit_pages(tmem_region region, tprot prot) const {
        if (!region.is_aligned(consts::page_size)) throw_rg_alignment();
        assert(does_contain(region));

        auto pages = region.relative_to(*this) / consts::page_size;
        for (auto i = pages.begin(); i < pages.end(); ++i) {
            _page_states[i.value()] = page(prot);
        }
    }

    // implementation is the same, but requires that there are no uncommited pages there
    void pages_region::reprotect_pages(tmem_region region, tprot prot) const {
        if (!region.is_aligned(consts::page_size)) throw_rg_alignment();
        assert(does_contain(region));
        if (has_uncommited_pages(region))
            throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS,
                               "specified region has uncommited pages.");

        auto pages = region.relative_to(*this) / consts::page_size;
        for (auto i = pages.begin(); i < pages.end(); ++i) {
            _page_states[i.value()] = page(prot);
        }
    }

    bool pages_region::uncommit_pages(tmem_region region) const {
        if (!region.is_aligned(consts::page_size)) throw_rg_alignment();
        assert(does_contain(region));

        bool res = false;

        auto pages = region.relative_to(*this) / consts::page_size;
        for (auto i = pages.begin(); i < pages.end(); ++i) {
            res |= _page_states[i.value()].commited;
            _page_states[i.value()] = page();
        }
        return res;
    }

    pages_region::query_result pages_region::query(taddr ptr) const {
        assert(util::is_aligned(ptr, consts::page_size));
        auto first_page_index = (tmem_region(ptr, consts::page_size).relative_to(*this) / consts::page_size).begin().value();
        auto poked_page_state = _page_states[first_page_index];
        taddr::tvalue end_page_index = first_page_index + 1;
        while (end_page_index < size() / consts::page_size && _page_states[end_page_index] == poked_page_state)
            end_page_index++;
        while (first_page_index > 0 && _page_states[first_page_index - 1] == poked_page_state)
            first_page_index--;

        auto pages_size = end_page_index - first_page_index;

        tmem_region base_region(*this); // NOLINT(cppcoreguidelines-slicing)

        auto region = tmem_region(this->begin() + first_page_index * consts::page_size, pages_size * consts::page_size);
        if (poked_page_state.commited)
            return query_results::committed{region, base_region, poked_page_state.prot()};
        return query_results::reserved{region, base_region};
    }
}