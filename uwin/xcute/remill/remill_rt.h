//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "xcute/remill/remill_state.h"

namespace uwin::xcute::remill {
    extern "C" Memory *uwin_xcute_remill_dispatch(State& st, uint32_t pc, Memory *mem);
    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem);
}
