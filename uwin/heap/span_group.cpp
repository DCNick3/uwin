//
// Created by dcnick3 on 12/3/20.
//

#include "heap/span_group.h"
#include "heap/consts.h"

namespace  uwin::heap {
    mem::taddr span_group::alloc(mem::taddr::tvalue size) {
        assert(size <= consts::large_block_boundary);

        auto it = _span_set.lower_bound(size);
        if (it == _span_set.end()) {
            it = _span_set.emplace(_mem_mgr);
            _span_iterators.emplace(&*it, it);
        }

        auto node = _span_set.extract(it);

        auto res = node.value().alloc(size);

        _allocated_size += node.value().size(res);

        // TODO: exception safety (well, who cares?)

        _span_set.insert(std::move(node));

        return res;
    }

    void span_group::free(mem::taddr ptr) {
        auto& span = span::from_ptr(_mem_mgr, ptr);
        auto itit = _span_iterators.find(&span);
        assert(itit != _span_iterators.end());
        auto it = itit->second;

        auto node = _span_set.extract(it);

        // TODO: exception safety (well, who cares?)

        auto size = span.size(ptr);

        span.free(ptr);

        _allocated_size -= size;

        if (!node.value().empty())
            _span_set.insert(std::move(node));
        else
            _span_iterators.erase(itit);
    }
}
