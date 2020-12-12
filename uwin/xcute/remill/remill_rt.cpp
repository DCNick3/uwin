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
            return leave_target_code(win32::dll::dispatch, st, pc, mem);
        } else {
            return uwin_xcute_remill_dispatch_recompiled(st, pc, mem);
        }
    }

    extern "C" Memory *uwin_xcute_remill_error(State& st, uint32_t pc, Memory *mem) {
        std::terminate();
    }

    extern "C" Memory *uwin_xcute_remill_async_hyper_call(State& st, uint32_t pc, Memory *mem) {
        std::terminate();
    }

    Memory* enter_target_code(mem::mgr::target_mem_mgr &mem_mgr, StateEx &remill_state) {
        auto& frame = remill_state.frame_stack.emplace();
        if (setjmp(frame.jmp_buf) > 0) {
            if (frame.jmp_reason)
                std::rethrow_exception(frame.jmp_reason);
            else
                // WTF?
                std::terminate();
        }

        auto base_addr = mem_mgr.get_region_base();
        auto pc = remill_state.gpr.rip.dword;
        auto res = uwin_xcute_remill_dispatch(remill_state, pc, reinterpret_cast<Memory*>(base_addr));
        // TODO: what if return happens, but we should not return?
        // May be the case for obfuscated CFG or when ret is not used for return
        // should probably put some magical return address on stack (on upper level, where the arguments are marshaled)
        assert(&frame == &remill_state.frame_stack.top());
        remill_state.frame_stack.pop();
        return res;
    }

    Memory* leave_target_code(ex_block_fun target, State &st, uint32_t pc, Memory *mem) {
        auto& state = static_cast<StateEx&>(st);

        auto& frame = state.frame_stack.top();
        try {
            return target(state, pc, mem);
        } catch (...) {
            frame.jmp_reason = std::current_exception();
            std::longjmp(frame.jmp_buf, 1); // NOLINT(cert-err52-cpp)
        }
    }
}
