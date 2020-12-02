#pragma once

#include "ctx/process.h"
#include "xcute/remill/remill_rt.h"
#include "win32/ldr/linkable.h"

namespace uwin::win32::dll {
    class base : public ldr::linkable {
    public:
        explicit base(ctx::process &process_ctx) : _process_ctx(process_ctx) {}

    protected:
        uwin::ctx::process &_process_ctx;

        inline std::uint32_t get_esp_u32(xcute::remill::State& state, mem::taddr::tsvalue esp_offset) const {
            return _process_ctx.deref(
                    mem::tptr<std::uint32_t>(
                            state.gpr.rsp.dword + esp_offset
                    ));
        }

        inline std::int32_t get_esp_s32(xcute::remill::State& state, mem::taddr::tsvalue esp_offset) const {
            return _process_ctx.deref(
                    mem::tptr<std::int32_t>(
                            state.gpr.rsp.dword + esp_offset
                    ));
        }

        inline std::uint32_t get_stdcall_u32(xcute::remill::State& state, std::int32_t index) const {
            return get_esp_u32(state, index * 4 + 4); // skip ret address
        }

        inline std::uint32_t get_stdcall_s32(xcute::remill::State&  state, std::int32_t index) const {
            return get_esp_s32(state, index * 4 + 4); // skip ret address
        }

        static inline void stdcall_set_result_u32(xcute::remill::State&  state, std::uint32_t value) {
            state.gpr.rax.dword = value;
        }

        static inline void stdcall_set_result_s32(xcute::remill::State& state, std::int32_t value) {
            stdcall_set_result_u32(state, *reinterpret_cast<std::uint32_t *>(&value));
        }

        inline void stdcall_ret(xcute::remill::State& state, std::int32_t argument_number) const {
            auto new_eip = get_esp_u32(state, 0);
            state.gpr.rip.dword = new_eip;
            state.gpr.rsp.dword += 4 + 4 * argument_number;
        }

        [[nodiscard]] inline std::string_view str(mem::tcptr<char> tstr) const {
            return _process_ctx._mem_mgr->str(tstr);
        }

        virtual ~base() = default;
    };
}