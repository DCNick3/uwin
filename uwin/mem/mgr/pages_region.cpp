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
}