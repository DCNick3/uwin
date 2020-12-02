//
// Created by dcnick3 on 12/1/20.
//

#pragma once


#include "mem/mgr/target_mem_mgr.h"
#include "util/nocopy.h"

namespace uwin::mem::mgr {
    class region_holder : util::nocopy {
        enum class holder_type {
            reserve,
            commit,
            reserve_and_commit,
        };

        target_mem_mgr* _mem_mgr;
        holder_type _holder_type;
        tmem_region _region;

    public:
        region_holder();
        region_holder(target_mem_mgr& mem_mgr, holder_type holder_type, tmem_region region);

        region_holder(region_holder&& o) noexcept;

        region_holder& operator=(region_holder&& o) noexcept;

        static region_holder reserve_dynamic(target_mem_mgr& mem_mgr, taddr::tvalue size);

        static region_holder commit(target_mem_mgr& mem_mgr, tmem_region region, tprot prot);

        static region_holder reserve_and_commit(target_mem_mgr& mem_mgr, taddr::tvalue size, tprot prot);

        [[nodiscard]] tmem_region release();

        [[nodiscard]] tmem_region get() const;
        [[nodiscard]] target_mem_mgr* get_mgr() const;

        ~region_holder();
    };
}