//
// Created by dcnick3 on 10/25/20.
//

#pragma once

// some macro magic
// this is quite ugly... What can be done with it?

// included here, so that it will not get included later, inside our namespace
#include <cstdlib>
#include <cstdint>
#include <limits>
#include <type_traits>

#define HAS_FEATURE_AVX 0
#define HAS_FEATURE_AVX512 0
#define ADDRESS_SIZE_BITS 32
namespace uwin {
    namespace xcute {
        namespace remill {

#include <remill/Arch/X86/Runtime/State.h>

        }
    }
}

#undef HAS_FEATURE_AVX
#undef HAS_FEATURE_AVX512
#undef ADDRESS_SIZE_BITS