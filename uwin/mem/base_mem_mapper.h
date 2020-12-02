//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "mem/mem_region.h"
#include "util/nocopy.h"
#include "util/nomove.h"

#include <fmt/format.h>

#include <cstdint>
#include <cstdlib>

namespace uwin::mem {
    // Host protection. Enforces w^x access
    enum class hprot {
        none,
        r,
        rw,
        rx,
    };

    class base_mem_mapper : util::nocopy, util::nomove {
    public:
        inline base_mem_mapper() = default;
        // All of those functions should not return error codes, but fail by raising an exception
        // This class is just PAL (platform abstraction layer), it does not manage allocated memory
        // Deallocation is a task for target_mem_mgr

        // all addresses and sizes must pe host page-aligned (page size can be obtained with page_size() function)

        // reserves memory region for further mapping by host_map_fixed
        virtual hmem_region host_reserve(std::size_t size) = 0;

        // unreserves reserved memory region. Can't partially unreserve, must unreserve the full reserved region.
        // Must have already unmapped (if mapped) memory
        virtual void host_unreserve(hmem_region region) = 0;

        // maps memory region with an address chosen by the implementation; will not map to reserved regions
        virtual hmem_region host_map_dynamic(std::size_t size, hprot prot) = 0;

        // maps memory region at the fixed address.
        // Must belong to previously reserved memory region, must not map same region twice
        // map over already mapped page is no-op
        virtual hmem_region host_map_fixed(hmem_region region, hprot prot) = 0;

        // Changes memory protection for mapped region. Should not be used with other memory regions
        // Can work with partial regions
        virtual void host_reprotect(hmem_region region, hprot prot) = 0;

        // unmaps mapped memory region. Partial unmap is possible.
        // The whole region must be mapped.
        virtual void host_unmap(hmem_region region) = 0;

        virtual std::size_t page_size() = 0;

        virtual ~base_mem_mapper() = default;

        // TODO: write a middleware that checks those conditions
    };
}

template<>
struct fmt::formatter<uwin::mem::hprot> : formatter<string_view> {
    // parse is inherited from formatter<string_view>.
    template<typename FormatContext>
    auto format(uwin::mem::hprot c, FormatContext &ctx) {
        string_view name = "unknown";
        switch (c) {
            case uwin::mem::hprot::none:
                name = "none";
            case uwin::mem::hprot::r:
                name = "r";
            case uwin::mem::hprot::rw:
                name = "rw";
            case uwin::mem::hprot::rx:
                name = "rx";
        }
        return formatter<string_view>::format(name, ctx);
    }
};