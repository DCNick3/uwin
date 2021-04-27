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
#include <stack>
#include <exception>

#include <csetjmp>

namespace uwin {
    namespace ctx {
        class process;
        class thread;
    }

    namespace xcute::remill {

// a hacky way to put all the definitions inside a namespace
#define HAS_FEATURE_AVX 0
#define HAS_FEATURE_AVX512 0
#define ADDRESS_SIZE_BITS 32
#include <remill/Arch/X86/Runtime/State.h>
#undef HAS_FEATURE_AVX
#undef HAS_FEATURE_AVX512
#undef ADDRESS_SIZE_BITS

        struct longjmp_frame {
            std::jmp_buf jmp_buf;
            std::exception_ptr jmp_reason;
        };

        struct StateEx : public State {
            ctx::process *process_ctx;
            ctx::thread *thread_ctx;
        };
    }
}