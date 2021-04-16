#pragma once

#include "ctx/process.h"
#include "ctx/thread.h"
#include "xcute/remill/remill_rt.h"
#include "win32/ldr/module.h"
#include "win32/svc/locale.h"
#include "win32/dll/vararg_ctx.h"

namespace uwin::win32::dll {
    class native_module : public ldr::module {
        mem::tmem_region _memory_region;

    public:
        explicit native_module(mem::mgr::target_mem_mgr &target_mem_mgr, svc::locale &locale, std::string name)
                : _mem_mgr(target_mem_mgr), _locale(locale), _name(std::move(name)),
                    // TODO: put some useful info into this region so that it looks more like genuine PE image
                    _memory_region(target_mem_mgr.reserve_dynamic(mem::mgr::consts::page_size)){
        }

        [[nodiscard]] inline types::hmodule handle() const override {
            return _memory_region.begin().as_const().as<types::module_tag>();
        }

        [[nodiscard]] const std::string &name() const override { return _name; }

    protected:
        mem::mgr::target_mem_mgr &_mem_mgr;
        svc::locale& _locale;
        ctx::thread* _current_thread{};
        std::string _name;

        inline std::uint32_t get_esp_u32(xcute::remill::State& state, mem::taddr::tsvalue esp_offset) const {
            return _mem_mgr.deref(
                    mem::tptr<std::uint32_t>(
                            state.gpr.rsp.dword + esp_offset
                    ));
        }

        inline std::int32_t get_esp_s32(xcute::remill::State& state, mem::taddr::tsvalue esp_offset) const {
            return _mem_mgr.deref(
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

        static inline void stdcall_set_result_u16(xcute::remill::State&  state, std::uint16_t value) {
            stdcall_set_result_u32(state, value);
        }

        inline void stdcall_ret(xcute::remill::State& state, std::int32_t argument_number) const {
            auto new_eip = get_esp_u32(state, 0);
            state.gpr.rip.dword = new_eip;
            state.gpr.rsp.dword += 4 + 4 * argument_number;
        }

        inline vararg_ctx cdecl_get_vararg_ctx(xcute::remill::State& state, std::int32_t non_vararg_arg_count) const {
            auto tptr = state.gpr.rsp.dword + 4 + non_vararg_arg_count * 4;
            return vararg_ctx(_mem_mgr.ptr(mem::tcptr<std::uint32_t>(tptr)));
        }

        inline std::uint32_t get_cdecl_u32(xcute::remill::State& state, std::int32_t index) const {
            return get_esp_u32(state, index * 4 + 4); // skip ret address
        }

        inline std::uint32_t get_cdecl_s32(xcute::remill::State&  state, std::int32_t index) const {
            return get_esp_s32(state, index * 4 + 4); // skip ret address
        }

        static inline void cdecl_set_result_u32(xcute::remill::State&  state, std::uint32_t value) {
            state.gpr.rax.dword = value;
        }

        static inline void cdecl_set_result_s32(xcute::remill::State& state, std::int32_t value) {
            stdcall_set_result_u32(state, *reinterpret_cast<std::uint32_t *>(&value));
        }

        inline void cdecl_ret(xcute::remill::State& state, std::int32_t argument_number) const {
            auto new_eip = get_esp_u32(state, 0);
            state.gpr.rip.dword = new_eip;
            state.gpr.rsp.dword += 4;
        }

        [[nodiscard]] inline std::string_view tstr(mem::tcptr<char> ptr) const {
            return _mem_mgr.str(ptr);
        }

        [[nodiscard]] inline std::string nstr(mem::tcptr<char> ptr) const {
            return _locale.ascii_to_native(tstr(ptr));
        }

        template<typename TRes>
        [[nodiscard]] TRes inline handle_error_ex(TRes error_result, auto fun) {
            auto& thread_ctx = *_current_thread;
            thread_ctx.set_last_error(error_code::ERROR_SUCCESS);

            auto res = error_result;
            try {
                res = fun(res);
            } catch (error const& e) {
                thread_ctx.set_last_error(e.code());
            }
            return res;
        }

        template<typename Fun, typename R = typename std::invoke_result_t<Fun>>
        [[nodiscard]] auto inline handle_error(R error_result, Fun fun) {
            return handle_error_ex(error_result, [&](R& err) mutable { return fun(); });
        }

        virtual ~native_module() = default;
    };
}