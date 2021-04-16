//
// Created by dcnick3 on 4/16/21.
//

#pragma once

#include "mem/mem_region.h"

namespace uwin::mem {
    namespace {
        extern "C" {
            void __asan_poison_memory_region(void const volatile *addr, size_t size); // NOLINT(bugprone-reserved-identifier)
            void __asan_unpoison_memory_region(void const volatile *addr, size_t size); // NOLINT(bugprone-reserved-identifier)
        }
    }

    // TODO: use it in heap implementation (see https://github.com/google/sanitizers/wiki/AddressSanitizerManualPoisoning)

    inline void asan_poison(hmem_region region) {
        // darn these macros!
#if defined(__has_feature)
#if __has_feature(address_sanitizer) || defined(__SANITIZE_ADDRESS__)
        __asan_poison_memory_region(region.begin(), region.size());
#endif
#elif defined(__SANITIZE_ADDRESS__)
        __asan_poison_memory_region(region.begin(), region.size());
#endif
    }

    inline void asan_unpoison(hmem_region region) {
        // darn these macros!
#if defined(__has_feature)
#if __has_feature(address_sanitizer) || defined(__SANITIZE_ADDRESS__)
        __asan_unpoison_memory_region(region.begin(), region.size());
#endif
#elif defined(__SANITIZE_ADDRESS__)
        __asan_unpoison_memory_region(region.begin(), region.size());
#endif
    }
}
