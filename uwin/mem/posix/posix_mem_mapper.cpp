//
// Created by dcnick3 on 10/25/20.
//

// use gnu extensions if possible (for MAP_ANONYMOUS and MAP_NORESERVE)
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include "mem/mem_mapper.h"
#include "mem/posix/posix_mem_mapper.h"

#include <fmt/core.h>

#include <sys/mman.h>
#include <unistd.h>

#include <exception>

#include <cstdio>
#include <cstring>

namespace uwin {
    namespace mem {
        const uint32_t page_size = sysconf(_SC_PAGESIZE);

        static int prot_to_posix(hprot prot) {
            switch (prot) {
                case hprot::none: return PROT_NONE;
                case hprot::r:    return PROT_READ;
                case hprot::rw:   return PROT_READ | PROT_WRITE;
                case hprot::rx:   return PROT_READ | PROT_EXEC;
                default:         std::terminate();
            }
        }


        hmem_region posix_mem_mapper::host_reserve(std::size_t size) {
            auto posix_prot = prot_to_posix(hprot::none);

            int flags = MAP_PRIVATE;
            // MAP_ANONYMOUS is not so posix, but mostly available
#ifdef MAP_ANONYMOUS
            flags |= MAP_ANONYMOUS;
#else
            static_assert(false, "Missing implementation for posix_mem_mapper::host_reserve without MAP_ANONYMOUS");
#endif
            // MAP_NORESERVE is linux extension. But it doesn't hurt to try
#ifdef MAP_NORESERVE
            flags |= MAP_NORESERVE;
#endif

            void* res = mmap(nullptr, size, posix_prot,
                             flags, -1, 0);

            if (res == MAP_FAILED)
                throw posix_mem_mapper_except(fmt::format("mmap for posix_mem_mapper::host_reserve({})",
                                                          size), errno);

            return {reinterpret_cast<std::uint8_t*>(res), size};
        }

        void posix_mem_mapper::host_unreserve(hmem_region region) {
            return host_unmap(region);
        }

        hmem_region mem::posix_mem_mapper::host_map_dynamic(std::size_t size, hprot prot) {
            auto posix_prot = prot_to_posix(prot);

            int flags = MAP_PRIVATE;
            // MAP_ANONYMOUS is not so posix, but mostly available
#ifdef MAP_ANONYMOUS
            flags |= MAP_ANONYMOUS;
#else
            static_assert(false, "Missing implementation for posix_mem_mapper::host_unreserve without MAP_ANONYMOUS");
#endif
            void* res = mmap(nullptr, size, posix_prot,
                             flags, -1, 0);

            if (res == MAP_FAILED)
                throw posix_mem_mapper_except(
                        fmt::format("mmap for posix_mem_mapper::host_map_dynamic({}, {})",
                                                          size, prot), errno);

            return {reinterpret_cast<std::uint8_t*>(res), size};
        }

        hmem_region posix_mem_mapper::host_map_fixed(hmem_region region, hprot prot) {
            auto posix_prot = prot_to_posix(prot);

            int flags = MAP_PRIVATE | MAP_FIXED;
            // MAP_ANONYMOUS is not so posix, but mostly available
#ifdef MAP_ANONYMOUS
            flags |= MAP_ANONYMOUS;
#else
            static_assert(false, "Missing implementation for posix_mem_mapper::host_map_fixed without MAP_ANONYMOUS");
#endif
            void* res = mmap(region.begin(), region.size(), posix_prot,
                             flags, -1, 0);

            if (res == MAP_FAILED)
                throw posix_mem_mapper_except(fmt::format("mmap for posix_mem_mapper::host_map_fixed({}, {})",
                      region, prot), errno);

            return region;
        }

        void posix_mem_mapper::host_reprotect(hmem_region region, hprot prot) {
            auto posix_prot = prot_to_posix(prot);

            int r = mprotect(region.begin(), region.size(), posix_prot);

            if (r != 0)
                throw posix_mem_mapper_except(fmt::format("mprotect for posix_mem_mapper::host_reprotect({}, {})",
                                                          region, prot), errno);
        }

        void posix_mem_mapper::host_unmap(hmem_region region) {
            if (munmap(region.begin(), region.size()) != 0)
                throw posix_mem_mapper_except(fmt::format("munmap for posix_mem_mapper::host_unmap({})",
                                                          region), errno);
        }

        std::size_t posix_mem_mapper::page_size() {
            return (std::size_t)sysconf(_SC_PAGESIZE);
        }


        std::shared_ptr<base_mem_mapper> create_host_mem_mapper() {
            return std::make_shared<posix_mem_mapper>();
        }

    }
}
