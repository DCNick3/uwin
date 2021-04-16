//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "xcute/remill/remill_state.h"
#include "mem/mgr/target_mem_mgr.h"

namespace uwin::xcute::remill {
    typedef Memory* (*block_fun)(State& st, uint32_t pc, Memory *mem);
    typedef Memory* (*ex_block_fun)(StateEx& st, uint32_t pc, Memory *mem);

    // this pair of functions takes care of passing the exception through recompiled functions
    //  enter_target_code works like a call into the target
    //  leave_target_code works like a call into the host (handled transparently for native dlls)
    Memory* enter_target_code(mem::mgr::target_mem_mgr& mem_mgr, StateEx& remill_state);
    Memory* leave_target_code(ex_block_fun target, State& st, uint32_t pc, Memory *mem);

    extern "C" Memory *uwin_xcute_remill_dispatch(State& st, uint32_t pc, Memory *mem);
    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem);
    extern "C" Memory *uwin_xcute_remill_async_hyper_call(State& st, uint32_t pc, Memory *mem);
    extern "C" [[noreturn]] void uwin_xcute_remill_abort(const char* reason);
    extern "C" [[noreturn]] Memory *uwin_xcute_remill_sync_hyper_call(State &st, uint32_t pc, Memory *mem);
}
