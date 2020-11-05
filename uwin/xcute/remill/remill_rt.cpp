//
// Created by dcnick3 on 10/25/20.
//

#include "xcute/remill/remill_state.h"
#include "xcute/remill/remill_rt.h"

#include <exception>

namespace uwin {
    namespace xcute {
        namespace remill {



            extern "C" Memory *sub_401000(State *st, uint32_t pc, Memory *mem);

            extern "C" Memory *uwin_remill_dispatch(State *st, uint32_t pc, Memory *mem) {
                if (pc & 0x80000000) {
                    ::std::terminate();
                } else {
                    switch (pc) {
                        case 0x401000:
                            return sub_401000(st, pc, mem);
                        default:
                            ::std::terminate();
                    }
                }
            }
        }
    }
}
