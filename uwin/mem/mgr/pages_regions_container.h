//
// Created by dcnick3 on 10/31/20.
//

#include "mem/tptr.h"
#include "mem/mem_region.h"
#include "mem/mgr/pages_region.h"

#include <vector>
#include <set>

#include <cstdlib>

#pragma once

namespace uwin {
    namespace mem {
        namespace mgr {

            class pages_region_container_nonconformant_containing_region_requested : std::runtime_error {
            public:
                pages_region_container_nonconformant_containing_region_requested()
                        : runtime_error("A containing pages_region was requested from pages_region_container, but only intersected"
                                        " region found.") {}
            };

            class pages_regions_container {
            public:

            private:
                std::set<pages_region, pages_region::cmp> _regions;
                // invariant: no regions in the set overlap
                // invariant: no region in the set starts below initial_reserved_space

            public:
                using iterator = decltype(_regions)::iterator;

            private:
                // precondition: the specified range does not overlap any region
                iterator alloc_at(taddr begin, taddr::tvalue size);

            public:

                iterator alloc(taddr::tvalue size);

                iterator insert(tmem_region region);

                iterator find_starting_with(taddr addr) const;
                iterator find_containing(tmem_region region) const;

                inline iterator begin() const { return _regions.begin(); }

                inline iterator end() const { return _regions.end(); }

                inline std::size_t size() const { return _regions.size(); }

                void erase(iterator it);

                // check the overlap invariant. Raise if the invariant is broken
                void check_overlaps() const;
            };
        }
    }
}



