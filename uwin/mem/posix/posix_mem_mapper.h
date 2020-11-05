//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "mem/base_mem_mapper.h"
#include "posix/except.h"

namespace uwin {
    namespace mem {

        class posix_mem_mapper_except : public posix::exception {
        public:
            inline posix_mem_mapper_except(const std::string &what_failed, int err) : exception(
                    what_failed, err) {}
        };

        class posix_mem_mapper : public base_mem_mapper {
        public:
            posix_mem_mapper() = default;

            hmem_region host_reserve(std::size_t size) override;
            void host_unreserve(hmem_region region) override;

            hmem_region host_map_dynamic(std::size_t size, hprot prot) override;
            hmem_region host_map_fixed(hmem_region region, hprot prot) override;

            void host_reprotect(hmem_region region, hprot prot) override;

            void host_unmap(hmem_region region) override;

            std::size_t page_size() override;

            // does not require any special deallocation logic; not out responsibility
            ~posix_mem_mapper() override = default;
        };
    }
}