//
// Created by dcnick3 on 10/31/20.
//

#pragma once

#include "mem/tptr.h"
#include "mem/mem_region.h"
#include "mem/mgr/consts.h"
#include "mem/mgr/tprot.h"
#include "mem/mgr/query_results.h"
#include "util/nocopy.h"
#include "util/nomove.h"

#include <cassert>
#include <variant>
#include <vector>

namespace uwin::mem::mgr {
    // here we have several hacks going on
    // we put regions to std::set sorted by beginning address (for effective lookup)
    // std::set requires that we will not modify the key. But it just assumes that the whole object is a key =)
    // due to that we can't really get a non-const reference or pointer to region from this
    // Here comes the hack: _page_states is declared mutable to allow its modification even through const ptr

    // It provides only data structure and does not do any actual virtual memory operations
    class pages_region : public tmem_region, util::nocopy, util::nomove {

        struct page {
            // Bitfields are used here
            bool access_r: 1;
            bool access_w: 1;
            bool access_x: 1; // Not actually used for host memory mapping, just emulation
            bool commited: 1;

            /// constructs uncommited page
            inline page() {
                commited = false;
            }

            /// constructs commited page
            inline explicit page(tprot prot) {
                auto prot_int = static_cast<std::uint8_t>(prot);
                access_r = (prot_int & static_cast<std::uint8_t>(tprot::r)) != 0;
                access_w = (prot_int & static_cast<std::uint8_t>(tprot::w)) != 0;
                access_x = (prot_int & static_cast<std::uint8_t>(tprot::x)) != 0;
                commited = true;
            }

            inline bool operator==(page const& o) const {
                if (!commited)
                    return !o.commited;
                return o.commited && access_r == o.access_r && access_w == o.access_w && access_x == o.access_x;
            }

            inline bool operator!=(page const& o) const {
                return !(*this == o);
            }

            [[nodiscard]] inline tprot prot() const {
                if (!commited)
                    // or should we throw?
                    return tprot::none;

                if (!access_r && !access_w && !access_x)
                    return tprot::none;
                if (!access_r && !access_w &&  access_x)
                    return tprot::x;
                if (!access_r &&  access_w && !access_x)
                    return tprot::w;
                if (!access_r &&  access_w &&  access_x)
                    return tprot::wx;
                if ( access_r && !access_w && !access_x)
                    return tprot::r;
                if ( access_r && !access_w &&  access_x)
                    return tprot::rx;
                if ( access_r &&  access_w && !access_x)
                    return tprot::rw;
                if ( access_r &&  access_w &&  access_x)
                    return tprot::rwx;

                std::terminate();
            }
        };

        static_assert(sizeof(page) == 1, "page struct is not packed enough");

        mutable std::vector<page> _page_states;
    public:

        struct cmp {
            // To allow comparison directly with taddr
            typedef std::true_type is_transparent;

            inline bool operator()(const pages_region &l, const pages_region &r) const {
                return l.begin().value() < r.begin().value();
            }

            inline bool operator()(const pages_region &l, const taddr &r) const {
                return l.begin().value() < r.value();
            }

            inline bool operator()(const taddr &l, const pages_region &r) const {
                return l.value() < r.begin().value();
            }
        };

        inline explicit pages_region(tmem_region region)
                : tmem_region(region) {
            assert(region.is_aligned(consts::page_size));
            auto page_count = region.size() / consts::page_size;
            {
                // TODO: use fixed-size vector, as in our case we don't need to change it's size
                // (it's unknown at compile time, so std::array would not work here)

                page p;
                _page_states.resize(page_count, p);
                _page_states.shrink_to_fit();
            }
        }


        typedef std::variant<query_results::reserved, query_results::committed> query_result;

        // precondition: passed region is fully contained within this pages_region
        bool has_uncommited_pages(tmem_region region) const;

        bool has_commited_pages(tmem_region region) const;

        void commit_pages(tmem_region region, tprot prot) const;

        bool uncommit_pages(tmem_region region) const;

        void reprotect_pages(tmem_region region, tprot prot) const;

        query_result query(taddr ptr) const;
    };

}