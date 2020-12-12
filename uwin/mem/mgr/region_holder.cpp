//
// Created by dcnick3 on 12/1/20.
//

#include "mem/mgr/region_holder.h"

// seems that clang is drunk
#pragma ide diagnostic ignored "EndlessLoop"

namespace uwin::mem::mgr {

    region_holder::region_holder()
            : _mem_mgr(nullptr), _holder_type(holder_type::reserve), _region(0, 0) {
    }

    region_holder::region_holder(target_mem_mgr &mem_mgr, region_holder::holder_type holder_type, tmem_region region)
            : _mem_mgr(&mem_mgr), _holder_type(holder_type), _region(region) {

    }

    region_holder::region_holder(region_holder &&o) noexcept
            : _mem_mgr(o._mem_mgr), _holder_type(o._holder_type), _region(o._region) {
        o._region = tmem_region(0, 0);
    }

    region_holder &region_holder::operator=(region_holder &&o) noexcept {
        if (&o != this) {
            _mem_mgr = o._mem_mgr;
            _holder_type = o._holder_type;
            _region = o._region;
            o._region = tmem_region(0, 0);
        }
        return *this;
    }

    region_holder region_holder::reserve_dynamic(target_mem_mgr &mem_mgr, taddr::tvalue size) {
        auto region = mem_mgr.reserve_dynamic(size);
        return {mem_mgr, holder_type::reserve, region};
    }

    region_holder region_holder::commit(target_mem_mgr &mem_mgr, tmem_region region, tprot prot) {
        mem_mgr.commit(region, prot);
        return {mem_mgr, holder_type::commit, region};
    }

    region_holder
    region_holder::reserve_and_commit_aligned(target_mem_mgr &mem_mgr, taddr::tvalue size, taddr::tvalue alignment,
                                              tprot prot) {
        auto region = mem_mgr.reserve_dynamic_aligned(size, alignment);
        mem_mgr.commit(region, prot);

        return {mem_mgr, holder_type::reserve_and_commit, region};
    }

    tmem_region region_holder::release() {
        auto res = _region;
        _region = tmem_region(0, 0);
        return res;
    }

    tmem_region region_holder::get() const {
        return _region;
    }

    region_holder::~region_holder() {
        if (_region.begin() != 0) {
            switch (_holder_type) {
                case holder_type::reserve:
                    _mem_mgr->unreserve(_region.begin());
                    break;
                case holder_type::commit:
                    _mem_mgr->uncommit(_region);
                    break;
                case holder_type::reserve_and_commit:
                    _mem_mgr->uncommit(_region);
                    _mem_mgr->unreserve(_region.begin());
                    break;
                default:
                    std::terminate();
            }

            _region = tmem_region(0, 0);
        }
    }

    target_mem_mgr *region_holder::get_mgr() const {
        return _mem_mgr;
    }
}