//
// Created by dcnick3 on 11/29/20.
//

#include "win32/dll/KERNEL32/impl.h"
#include "util/enumu.h"
#include "xcute/except.h"
#include "log/log.h"
#include "util/str.h"
#include "win32/kobj/event.h"

namespace uwin::win32::dll {

    uint32_t KERNEL32_impl::GetVersion() {
        return 0xc0000004; // As returned by windows 95
    }

    void KERNEL32_impl::GetStartupInfoA(uwin::mem::tptr<uwin::win32::types::STARTUPINFOA> lpStartupInfo) {
        auto res = _mem_mgr.ptr(lpStartupInfo);

        memset(res, 0, sizeof(*res));
        res->cb = sizeof(*res);
    }

    ht::handle<uwin::ht::kobj> KERNEL32_impl::GetStdHandle(std::uint32_t nStdHandle) {
        return ht::handle<uwin::ht::kobj>::invalid();
    }

    uint32_t KERNEL32_impl::SetHandleCount(std::uint32_t uNumber) {
        return std::min(uNumber, 0x100u);
    }

    mem::tptr<char> KERNEL32_impl::GetCommandLineA() {
        return _env._command_line;
    }

    mem::tptr<wchar_t> KERNEL32_impl::GetEnvironmentStringsW() {
        // We emulate Win95, so no unicode
        _current_thread->set_last_error(error_code::ERROR_CALL_NOT_IMPLEMENTED);
        return mem::tptr<wchar_t>(0);
    }

    mem::tptr<char> KERNEL32_impl::GetEnvironmentStrings() {
        return _process_heap->alloc_bytes(_env._environment).as<char>();
    }

    mem::tptr<char> KERNEL32_impl::GetEnvironmentStringsA() {
        return GetEnvironmentStrings();
    }

    bool KERNEL32_impl::FreeEnvironmentStringsA(uwin::mem::tptr<char> arg0) {
        _process_heap->free(arg0.as_taddr());
        return true;
    }

    uint32_t KERNEL32_impl::GetModuleFileNameA(uwin::win32::types::hmodule hModule, uwin::mem::tptr<char> lpFilename,
                                               std::uint32_t nSize) {
        // TODO: implement a proper module table
        // TODO: 8.3 paths?
        return handle_error_ex<uint32_t>(0, [&](uint32_t& result) {
            if (hModule != 0)
                throw util::not_implemented_error("hModule != 0");
            std::string self_module_file_name = "C:\\MAIN.EXE"; // Very dumb way to implement this
            auto output_buffer = _mem_mgr.ptr(lpFilename);

            auto last = std::min(std::uint32_t(self_module_file_name.size()), nSize - 1);
            for (std::uint32_t i = 0; i < last; i++) {
                output_buffer[i] = self_module_file_name[i];
            }
            output_buffer[last] = '\0';
            if (nSize - 1 < self_module_file_name.size())
                throw error(error_code::ERROR_INSUFFICIENT_BUFFER);
            return result;
        });
        //return last - 1;
    }

    std::string KERNEL32_impl::normalize_module_name(std::string_view unnormalized) {
        std::string module_name = util::ascii_to_upper(unnormalized);

        // the logic is carefully reconstructed according to win95's kernel32.dll
        auto extension_pos = module_name.find_last_of('.');
        if (extension_pos != std::string::npos) {
            if (extension_pos + 1 == module_name.size())
                module_name.erase(extension_pos);
        } else {
            module_name += ".DLL";
        }

        return module_name;
    }

    types::hmodule KERNEL32_impl::GetModuleHandleA(uwin::mem::tcptr<char> lpModuleName) {
        return handle_error(types::hmodule(0), [&]() {
            if (lpModuleName == 0) {
                // TODO: actually return a main module handle
                return types::hmodule(0);
            } else {
                auto module_name = normalize_module_name(tstr(lpModuleName));

                // TODO: allow to use full paths

                auto module = _module_table.try_get_module(module_name);

                if (module == nullptr)
                    throw error(error_code::ERROR_MOD_NOT_FOUND);

                return module->handle();
            }
        });
    }

    mem::tptr<void>
    KERNEL32_impl::GetProcAddress(uwin::win32::types::hmodule hModule, uwin::mem::tcptr<char> lpProcName) {
        return handle_error(mem::tptr<void>(0), [&]() {
            if (hModule == 0)
                // FIXME: assuming there are no exports in the main module (can't get object for main module)
                throw error(error_code::ERROR_PROC_NOT_FOUND);

            auto module = _module_table.try_get_module(hModule);
            if (module == nullptr)
                throw error(error_code::ERROR_INVALID_HANDLE);

            if ((lpProcName.value() & 0xffff0000) == 0)
                throw util::not_implemented_error("GetProcAddress by ordinal");

            auto name = std::string(tstr(lpProcName));

            auto result = module->try_resolve(name);

            if (result.value() == 0)
                throw error(error_code::ERROR_PROC_NOT_FOUND);

            return result.as<void>().as_non_const();
        });
    }

    void KERNEL32_impl::ExitProcess(std::uint32_t uExitCode) {
        throw xcute::process_exit(uExitCode);
    }

    uint32_t KERNEL32_impl::GetCurrentThreadId() {
        return 2; // TODO: threads
    }

    mem::tptr<void> KERNEL32_impl::SetUnhandledExceptionFilter(uwin::mem::tptr<void> lpTopLevelExceptionFilter) {
        // TODO: implement SEH unwinding (ignored for now)
        return {0};
    }

    ht::handle<uwin::ht::kobj>
    KERNEL32_impl::CreateEventA(uwin::mem::tptr<uwin::win32::types::SECURITY_ATTRIBUTES> lpEventAttributes,
                                bool bManualReset, bool bInitialState, uwin::mem::tcptr<char> lpName) {
        return handle_error(ht::handle<uwin::ht::kobj>::invalid(), [&]() {
            if (lpEventAttributes.value() != 0)
                throw util::not_implemented_error("Non-zero lpEventAttributes");
            if (lpName.value() != 0)
                throw util::not_implemented_error("Named events");

            return _handletable.emplace<kobj::event>().as_kobj();
        });
    }
}