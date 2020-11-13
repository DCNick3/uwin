//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include "mem/mgr/target_mem_mgr.h"
#include "mem/tptr.h"

namespace uwin {
    namespace ctx {
        class dll;

        class process {
        public:
            std::unique_ptr<mem::mgr::target_mem_mgr> _mem_mgr;
            std::unique_ptr<dll> _dll;

            template<typename T>
            inline auto guest_to_host(mem::tptr<T> addr) const { return _mem_mgr->guest_to_host(addr); }

            template<typename T>
            inline auto guest_to_host(mem::tcptr<T> addr) const { return _mem_mgr->guest_to_host(addr); }

            inline mem::hmem_region guest_to_host(mem::tmem_region const& region) const { return _mem_mgr->guest_to_host(region); }

            template<typename T>
            inline T& deref(mem::tptr<T, false> ptr) const { return _mem_mgr->deref(ptr); }

            template<typename T>
            inline T const& deref(mem::tptr<T, true> ptr) const { return _mem_mgr->deref(ptr); }
        };
    }
}