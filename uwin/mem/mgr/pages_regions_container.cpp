//
// Created by dcnick3 on 10/31/20.
//

#include "pages_regions_container.h"
#include "mem/except.h"
#include "win32/error.h"
#include "util/visit.h"

#include <cassert>

namespace uwin::mem::mgr {

    pages_regions_container::iterator pages_regions_container::find_starting_with(taddr addr) const {
        assert(util::is_aligned(addr, consts::allocation_granularity));
        return _regions.find(addr);
    }

    pages_regions_container::query_result pages_regions_container::query(taddr addr) const {
        assert(util::is_aligned(addr, consts::page_size));
        auto it = _regions.upper_bound(addr);
        if (it == _regions.begin()) // it means that even the first region begins (strictly) after our region
            return query_results::free{{0, it->begin().value()}};
        it--;
        if (it->does_contain(addr)) {
            return query_results::reserved{it};
        } else {
            auto begin = it->begin() + it->size();
            it++;
            if (it == _regions.end())
                return query_results::free{{begin, std::numeric_limits<taddr::tvalue>::max() - begin.value() + 1}};
            return query_results::free{{begin, taddr::tvalue(it->begin() - begin)}};
        }
    }

    pages_regions_container::iterator pages_regions_container::find_containing(tmem_region region) const {
        assert(util::is_aligned(region.begin(), consts::page_size));
        auto it = _regions.upper_bound(region.begin());
        if (it == _regions.begin()) // it means that even the first region begins (strictly) after out region
            return end();
        it--;
        if (it->does_contain(region)) return it;

        return end();
    }

    pages_regions_container::iterator pages_regions_container::alloc(taddr::tvalue size, taddr::tvalue alignment) {
        assert(util::is_aligned(size, consts::page_size));
        // Here we just do a dumb O(n) first fit search

        static_assert(util::is_aligned(consts::initial_reserved_space, consts::allocation_granularity),
                      "initial_reserved_space is not aligned to allocation_granularity");

        auto addr = taddr(consts::initial_reserved_space);
        addr = addr.align_up(std::max(consts::allocation_granularity, alignment));
        for (auto const &region : _regions) {
            auto diff = region.begin() - addr;
            if (diff >= 0) { // might happen when some alignment is enforced
                if (diff >= size)
                    // we found a hole =)
                    return alloc_at(addr, size);
            }
            addr = util::align_up(region.end(), std::max(consts::allocation_granularity, alignment));
        }
        // we didn't find a hole, but reached the end of the list.
        // it means that there are no regions past the addr
        // If we would not hit the address_space_reserved_start - we are good to go
        if ((addr + size).value() > consts::address_space_reserved_start)
            throw out_of_target_address_space();

        return alloc_at(addr, size);
    }

    pages_regions_container::iterator pages_regions_container::insert(tmem_region requested_region) {
        for (auto const &region : _regions) {
            if (region.intersects(requested_region))
                throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS,
                                   "requested region intersects with another reserved region.");
        }
        return alloc_at(requested_region.begin(), requested_region.size());
    }

    pages_regions_container::iterator pages_regions_container::alloc_at(taddr begin, taddr::tvalue size) {
        assert(util::is_aligned(begin, consts::allocation_granularity));
        assert(util::is_aligned(size, consts::page_size));
        auto res = _regions.emplace(tmem_region(begin, size));
        // we assert that insertion actually happened
        assert(res.second);
        return res.first;
    }

    void pages_regions_container::check_overlaps() const {
        // TODO: implement =)
    }

    void pages_regions_container::erase(pages_regions_container::iterator it) {
        _regions.erase(it);
    }

    std::string pages_regions_container::dump_reservation_map() const {
        std::string map;
        taddr p(0);
        bool cont = true;
        while (cont) {
            auto res_var = query(p);
            util::visit(res_var, [&](query_results::reserved& reserved) {
                auto& rg = *reserved.region_it;
                map += fmt::format("R {:#010x}-{:#010x} ({:#010x})\n",
                                   rg.begin().value(), rg.end().value(), rg.size());
                p = rg.end();
            }, [&](query_results::free& free) {
                auto& rg = free.region;
                map += fmt::format("F {:#010x}-{:#010x} ({:#010x})\n", rg.begin().value(), rg.end().value(), rg.size());
                if (rg.end().value() == 0)
                    cont = false;
                p = rg.end();
            });
        }
        if (!map.empty())
            map.erase(map.size() - 1);
        return map;
    }
}
