//
// Created by dcnick3 on 10/31/20.
//

#include "gtest/gtest.h"

#include "mem/mem_mapper.h"

using namespace uwin::mem;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

TEST(MemMapper, Basic) {
    auto mapper = create_host_mem_mapper();
    auto page_size = mapper->page_size();

    auto reserved = mapper->host_reserve(page_size * 100);

    auto allocated = mapper->host_map_dynamic(page_size, hprot::rw);
    memset(allocated.begin(), 0, allocated.size());

    auto fixmapped = mapper->host_map_fixed(reserved, hprot::r);
    memcpy(allocated.begin(), fixmapped.begin(), std::min(allocated.size(), fixmapped.size()));

    mapper->host_unmap(fixmapped);
    mapper->host_unmap(allocated);

    mapper->host_unreserve(reserved);
}
