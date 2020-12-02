//
// Created by dcnick3 on 10/25/20.
//

#include "xcute/remill/remill_state.h"
#include "xcute/remill/remill_rt.h"
#include "win32/dll/dispatcher.h"

#include <exception>

namespace uwin::xcute::remill {

    extern "C" Memory *uwin_xcute_remill_dispatch_recompiled(State& st, uint32_t pc, Memory *mem);

    extern "C" Memory *uwin_xcute_remill_dispatch(State& st, uint32_t pc, Memory *mem) {
        if (pc & 0x80000000) {
            return win32::dll::dispatch(st, pc, mem);
        } else {
            return uwin_xcute_remill_dispatch_recompiled(st, pc, mem);
        }
    }

    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem) {
        std::terminate();
    }
}
