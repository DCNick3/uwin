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
            // if we have pc address in upper part of the address space - we are dispatching to the native dll implementation
            return leave_target_code(win32::dll::dispatch_native, st, pc, mem);
        } else {
            // else - to the recompiled code
            return uwin_xcute_remill_dispatch_recompiled(st, pc, mem);
        }
    }

    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem) {
        std::terminate();
    }

    extern "C" Memory *uwin_xcute_remill_async_hyper_call(State& st, uint32_t pc, Memory *mem) {
        std::terminate();
    }
    extern "C" [[noreturn]] void uwin_xcute_remill_abort(const char* reason) {
        std::terminate();
    }
    extern "C" [[noreturn]] Memory *uwin_xcute_remill_sync_hyper_call(State &st, uint32_t pc, Memory *mem) {
        std::terminate();
    }

    Memory* enter_target_code(mem::mgr::target_mem_mgr &mem_mgr, StateEx &remill_state) {
        // exception handling code was here, but it was abolished in favor of using clang-generated unwind tables
        // (now exception can safely pass through the lifted functions, saving the callstack)

        auto base_addr = mem_mgr.get_region_base();
        auto pc = remill_state.gpr.rip.dword;
        auto res = uwin_xcute_remill_dispatch(remill_state, pc, reinterpret_cast<Memory*>(base_addr));
        // TODO: what if return happens, but we should not return?
        // May be the case for obfuscated CFG or when ret is not used for return
        // should probably put some magical return address on stack (on upper level, where the arguments are marshaled)
        return res;
    }

    Memory* leave_target_code(ex_block_fun target, State &st, uint32_t pc, Memory *mem) {
        auto& state = static_cast<StateEx&>(st);
        return target(state, pc, mem);
    }
}
