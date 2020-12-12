//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "xcute/remill/remill_state.h"
#include "mem/mgr/target_mem_mgr.h"

namespace uwin::xcute::remill {
    typedef Memory* (*block_fun)(State& st, uint32_t pc, Memory *mem);
    typedef Memory* (*ex_block_fun)(StateEx& st, uint32_t pc, Memory *mem);

    Memory* enter_target_code(mem::mgr::target_mem_mgr& mem_mgr, StateEx& remill_state);
    Memory* leave_target_code(ex_block_fun target, State& st, uint32_t pc, Memory *mem);

    extern "C" Memory *uwin_xcute_remill_dispatch(State& st, uint32_t pc, Memory *mem);
    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem);
    extern "C" Memory *uwin_xcute_remill_async_hyper_call(State& st, uint32_t pc, Memory *mem);
}
