//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "xcute/remill/remill_state.h"

namespace uwin {
    namespace xcute {
        namespace remill {
            extern "C" Memory *uwin_remill_dispatch(State *st, uint32_t pc, Memory *mem);
        }
    }
}
