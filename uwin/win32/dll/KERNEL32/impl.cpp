//
// Created by dcnick3 on 11/29/20.
//

#include "impl.h"
#include "util/enumu.h"
#include "xcute/except.h"
#include "log/log.h"
#include "util/str.h"

namespace uwin::win32::dll {
    enum class LCMAP : std::uint32_t {
        LOWERCASE = 0x00000100,
        UPPERCASE = 0x00000200,
    };
}

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
        return uNumber;
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

    bool KERNEL32_impl::FreeEnvironmentStringsA(uwin::mem::tptr<char> arg0) {
        _process_heap->free(arg0.as_taddr());
        return true;
    }

    // quite a dumb locale implementation
    // I should consider using some library for this, as it's quite... complicated

    uint32_t KERNEL32_impl::GetACP() {
        return 1252; // Windows 3.1 Latin 1 (U.S., Western Europe)
    }

    bool KERNEL32_impl::GetCPInfo(std::uint32_t CodePage, uwin::mem::tptr<uwin::win32::types::CPINFO> lpCPInfo) {
        if (CodePage != 1252)
            throw util::not_implemented_error("Any codepage other than 1251");
        auto hptr = _mem_mgr.ptr(lpCPInfo);
        hptr->MaxCharSize = 1;
        hptr->DefaultChar[0] = '?';
        hptr->DefaultChar[1] = '\0';
        memset(hptr->LeadByte, 0, sizeof(hptr->LeadByte));
        return true;
    }

    bool
    KERNEL32_impl::GetStringTypeW(std::uint32_t dwInfoType, uwin::mem::tcptr<wchar_t> lpSrcStr, std::int32_t cchSrc,
                                  uwin::mem::tptr<uint16_t> lpCharType) {
        // We emulate Win95, so no unicode
        _current_thread->set_last_error(error_code::ERROR_CALL_NOT_IMPLEMENTED);
        return false;
    }

    static std::uint16_t ctype_1[] = {0x248, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x268, 0x228,
                                      0x228, 0x228, 0x228, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220,
                                      0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x220, 0x248,
                                      0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210,
                                      0x210, 0x210, 0x210, 0x210, 0x284, 0x284, 0x284, 0x284, 0x284, 0x284, 0x284,
                                      0x284, 0x284, 0x284, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x381,
                                      0x381, 0x381, 0x381, 0x381, 0x381, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301,
                                      0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301,
                                      0x301, 0x301, 0x301, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x382, 0x382,
                                      0x382, 0x382, 0x382, 0x382, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302,
                                      0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302,
                                      0x302, 0x302, 0x210, 0x210, 0x210, 0x210, 0x220, 0x200, 0x220, 0x210, 0x302,
                                      0x210, 0x210, 0x210, 0x210, 0x300, 0x210, 0x301, 0x210, 0x301, 0x220, 0x301,
                                      0x220, 0x220, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x210, 0x200, 0x200,
                                      0x302, 0x210, 0x302, 0x220, 0x302, 0x301, 0x248, 0x210, 0x210, 0x210, 0x210,
                                      0x210, 0x210, 0x210, 0x210, 0x210, 0x312, 0x210, 0x210, 0x230, 0x210, 0x210,
                                      0x210, 0x210, 0x214, 0x214, 0x210, 0x312, 0x210, 0x210, 0x210, 0x214, 0x312,
                                      0x210, 0x210, 0x210, 0x210, 0x210, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301,
                                      0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x301,
                                      0x301, 0x301, 0x301, 0x301, 0x301, 0x301, 0x210, 0x301, 0x301, 0x301, 0x301,
                                      0x301, 0x301, 0x301, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302,
                                      0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302, 0x302,
                                      0x302, 0x302, 0x302, 0x302, 0x302, 0x210, 0x302, 0x302, 0x302, 0x302, 0x302,
                                      0x302, 0x302, 0x302,};
    static std::uint8_t map_lo[] = {0x20, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
                                    0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
                                    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
                                    0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
                                    0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
                                    0x76, 0x77, 0x78, 0x79, 0x7a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63,
                                    0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71,
                                    0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f,
                                    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x9a, 0x8b, 0x9c, 0x8d,
                                    0x9e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
                                    0x9c, 0x9d, 0x9e, 0xff, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9,
                                    0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
                                    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5,
                                    0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3,
                                    0xf4, 0xf5, 0xf6, 0xd7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xdf, 0xe0, 0xe1,
                                    0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
                                    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd,
                                    0xfe, 0xff, };
    static std::uint8_t map_up[] = {0x20, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
                                    0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
                                    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
                                    0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
                                    0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55,
                                    0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x41, 0x42, 0x43,
                                    0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
                                    0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f,
                                    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d,
                                    0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x8a, 0x9b,
                                    0x8c, 0x9d, 0x8e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9,
                                    0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
                                    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5,
                                    0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3,
                                    0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xc0, 0xc1,
                                    0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
                                    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xf7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd,
                                    0xde, 0x9f, };


            bool KERNEL32_impl::GetStringTypeA(std::uint32_t Locale, std::uint32_t dwInfoType, uwin::mem::tcptr<char> lpSrcStr,
                                       std::int32_t cchSrc, uwin::mem::tptr<uint16_t> lpCharType) {
        // Well, this is not what Win95 does exactly, but msvc 6.0 runtime is ok with it
        if (Locale != 0)
            throw util::not_implemented_error("Locale != 0");
        if (dwInfoType != 1)
            throw util::not_implemented_error("dwInfoType != 1");

        for (std::uint32_t i = 0; i < cchSrc; i++) {
            // convert to unsigned here
            std::uint8_t c =  _mem_mgr.ptr(lpSrcStr)[i];
            _mem_mgr.ptr(lpCharType)[i] = ctype_1[c];
        }

        return true;
    }

    int32_t
    KERNEL32_impl::LCMapStringW(std::uint32_t Locale, std::uint32_t dwMapFlags, uwin::mem::tcptr<wchar_t> lpSrcStr,
                                std::int32_t cchSrc, uwin::mem::tptr<wchar_t> lpDestStr, std::int32_t cchDest) {
        // We emulate Win95, so no unicode
        _current_thread->set_last_error(error_code::ERROR_CALL_NOT_IMPLEMENTED);
        return false;
    }

    int32_t KERNEL32_impl::LCMapStringA(std::uint32_t Locale, std::uint32_t dwMapFlags, uwin::mem::tcptr<char> lpSrcStr,
                                        std::int32_t cchSrc, uwin::mem::tptr<char> lpDestStr, std::int32_t cchDest) {
        return handle_error(0, [&]() {
            if (Locale != 0)
                throw util::not_implemented_error("Locale != 0");
            if (cchSrc == 0)
                throw util::not_implemented_error("cchSrc == 0");

            auto mapping = static_cast<LCMAP>(dwMapFlags);

            if (cchDest != 0 && cchDest < cchSrc)
                throw error(error_code::ERROR_INSUFFICIENT_BUFFER);

            if (mapping == LCMAP::LOWERCASE) {
                if (cchDest != 0) {
                    for (std::uint32_t i = 0; i < cchSrc; i++) {
                        // convert to unsigned here
                        std::uint8_t c = _mem_mgr.ptr(lpSrcStr)[i];
                        _mem_mgr.ptr(lpDestStr)[i] = map_lo[c];
                    }
                }

                return cchSrc;
            } else if (mapping == LCMAP::UPPERCASE) {

                if (cchDest != 0) {
                    for (std::uint32_t i = 0; i < cchSrc; i++) {
                        // convert to unsigned here
                        std::uint8_t c = _mem_mgr.ptr(lpSrcStr)[i];
                        _mem_mgr.ptr(lpDestStr)[i] = map_up[c];
                    }
                }

                return cchSrc;
            } else {
                throw util::not_implemented_error("specified mapping");
            }
        });
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
}