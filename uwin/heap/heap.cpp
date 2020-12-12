//
// Created by dcnick3 on 12/1/20.
//

#include "heap/heap.h"

namespace uwin::heap {

    mem::taddr heap::alloc(mem::taddr::tvalue size) {
        if (size > consts::large_block_boundary) {
            auto raw_size = util::align_up(size + sizeof(block_hdr), mem::mgr::consts::page_size);
            auto region = _mem_mgr.reserve_dynamic(raw_size);
            _mem_mgr.commit(region, mem::mgr::tprot::rw);
            auto host_ptr = _mem_mgr.ptr(region.begin());

            auto bl = new block(region.begin() + sizeof(block_hdr), region.size() - sizeof(block_hdr));
            new (host_ptr) block_hdr{bl};

            _raw_allocations.emplace(bl->_data_address);

            return bl->_data_address;
        } else
            return _span_group.alloc(size);
    }

    mem::taddr heap::realloc(mem::taddr old_ptr, mem::taddr::tvalue new_size) {
        auto new_ptr = alloc(new_size);
        auto old_data = _mem_mgr.ptr(old_ptr);
        auto new_data = _mem_mgr.ptr(new_ptr);
        memcpy(new_data, old_data, std::min(new_size, size(old_ptr)));
        return new_ptr;
    }

    void heap::free(mem::taddr ptr) {
        auto size = heap::size(ptr);
        // heap::size works for non-span allocated memory blocks too, due to usage of block header
        if (size > consts::large_block_boundary) {
            assert(_raw_allocations.contains(ptr));
            auto raw_ptr = ptr - sizeof(block_hdr);
            auto block = _mem_mgr.ptr(raw_ptr.as<block_hdr>())->p_block_obj;
            delete block;
            _mem_mgr.uncommit(mem::tmem_region(raw_ptr, size));
            _mem_mgr.unreserve(raw_ptr);
            _raw_allocations.erase(ptr);
        } else
            return _span_group.free(ptr);
    }

    heap::~heap() {
        while (!_raw_allocations.empty())
            free(*_raw_allocations.begin());
    }

}
