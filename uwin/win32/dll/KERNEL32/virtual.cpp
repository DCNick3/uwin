//
// Created by dcnick3 on 12/4/20.
//

#include "impl.h"

#include "util/except.h"

namespace uwin::win32::dll {

    enum class ALLOCATION_TYPE : std::uint32_t {
        COMMIT      = 0x00001000,
        RESERVE     = 0x00002000,
        RESET       = 0x00080000,
        TOP_DOWN    = 0x00100000,
        WRITE_WATCH = 0x00200000,
        PHYSICAL    = 0x00400000,
    };

    enum class PROTECTION : std::uint32_t {
        NOACCESS            = 0x01,
        READONLY            = 0x02,
        READWRITE           = 0x04,
        EXECUTE             = 0x10,
        EXECUTE_READ        = 0x20,
        EXECUTE_READWRITE   = 0x40,
        GUARD               = 0x100,
        NOCACHE             = 0x200,
    };
}

namespace uwin {
    template<>
    struct EnableBitMaskOperators<win32::dll::ALLOCATION_TYPE> {
        static const bool enable = true;
    };
    template<>
    struct EnableBitMaskOperators<win32::dll::PROTECTION> {
        static const bool enable = true;
    };
}
namespace uwin::win32::dll {

    static mem::mgr::tprot convert_protection(std::uint32_t flProtect) {
        auto allocation_type = static_cast<PROTECTION>(flProtect);
        switch (allocation_type) {
            case PROTECTION::NOACCESS:              return mem::mgr::tprot::none;
            case PROTECTION::READONLY:              return mem::mgr::tprot::r;
            case PROTECTION::READWRITE:             return mem::mgr::tprot::rw;
            case PROTECTION::EXECUTE:               return mem::mgr::tprot::x;
            case PROTECTION::EXECUTE_READ:          return mem::mgr::tprot::rx;
            case PROTECTION::EXECUTE_READWRITE:     return mem::mgr::tprot::rwx;
            case PROTECTION::GUARD:
            case PROTECTION::NOCACHE:
            default:
                throw util::not_implemented_error("Specified flProtect");
        }
    }

    mem::tptr<void>
    KERNEL32_impl::VirtualAlloc(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flAllocationType,
                                std::uint32_t flProtect) {
        auto allocation_type = static_cast<ALLOCATION_TYPE>(flAllocationType);

        if (allocation_type % ALLOCATION_TYPE::TOP_DOWN ||
           allocation_type % ALLOCATION_TYPE::PHYSICAL ||
           allocation_type % ALLOCATION_TYPE::WRITE_WATCH ||
           allocation_type % ALLOCATION_TYPE::RESET)
            throw util::not_implemented_error("Specified flAllocationType");

        auto region = mem::tmem_region(lpAddress.as_taddr(), dwSize);

        if (allocation_type % ALLOCATION_TYPE::RESERVE) {
            if (region.begin() == 0) {
                region = _process_ctx._mem_mgr->reserve_dynamic(region.size());
            } else {
                region = _process_ctx._mem_mgr->reserve_fixed(region);
            }
        }

        if (allocation_type % ALLOCATION_TYPE::COMMIT) {
            if (region.begin() == 0)
                throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS);
            _process_ctx._mem_mgr->commit(region, convert_protection(flProtect));
        }

        return region.begin().as<void>();
    }

    bool KERNEL32_impl::VirtualFree(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t dwFreeType) {
        return KERNEL32_iface::VirtualFree(lpAddress, dwSize, dwFreeType);
    }

    bool
    KERNEL32_impl::VirtualProtect(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flNewProtect,
                                  uwin::mem::tptr<uint32_t> lpflOldProtect) {
        return KERNEL32_iface::VirtualProtect(lpAddress, dwSize, flNewProtect, lpflOldProtect);
    }

    bool KERNEL32_impl::IsBadReadPtr(uwin::mem::tcptr<void> lp, std::uint32_t ucb) {
        return false; // trust me, it's okay =)
    }

    bool KERNEL32_impl::IsBadWritePtr(uwin::mem::tptr<void> lp, std::uint32_t ucb) {
        return false; // trust me, it's okay =)
    }
}