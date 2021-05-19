//
// Created by dcnick3 on 12/4/20.
//

#include "win32/dll/KERNEL32/impl.h"

#include "util/except.h"
#include "util/visit.h"

namespace uwin::win32::dll {

    enum class ALLOCATION_TYPE : std::uint32_t {
        COMMIT      = 0x00001000,
        RESERVE     = 0x00002000,
        FREE        = 0x00010000,
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
        if ((flProtect & 0xffU) != flProtect)
            throw util::not_implemented_error("Specified flProtect");
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

    static std::uint32_t convert_protection(mem::mgr::tprot prot) {
        PROTECTION res;
        switch (prot) {
            case mem::mgr::tprot::none: res = PROTECTION::NOACCESS; break;
            case mem::mgr::tprot::r:    res = PROTECTION::READONLY; break;
            case mem::mgr::tprot::x:    res = PROTECTION::EXECUTE; break;
            case mem::mgr::tprot::rw:   res = PROTECTION::READWRITE; break;
            case mem::mgr::tprot::rx:   res = PROTECTION::EXECUTE_READ; break;
            case mem::mgr::tprot::rwx:  res = PROTECTION::EXECUTE_READWRITE; break;
            default:
                util::runtime_error("Unmappable tprot: {}", prot);
        }

        return static_cast<std::uint32_t>(res);
    }

    mem::tptr<void>
    KERNEL32_impl::VirtualAlloc(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flAllocationType,
                                std::uint32_t flProtect) {
        return handle_error({0}, [&]() {
            auto allocation_type = static_cast<ALLOCATION_TYPE>(flAllocationType);

            if (allocation_type % ALLOCATION_TYPE::TOP_DOWN ||
                allocation_type % ALLOCATION_TYPE::PHYSICAL ||
                allocation_type % ALLOCATION_TYPE::WRITE_WATCH ||
                allocation_type % ALLOCATION_TYPE::RESET)
                throw util::not_implemented_error("Specified flAllocationType");

            // like in win95
            if ((allocation_type & (ALLOCATION_TYPE::COMMIT | ALLOCATION_TYPE::RESERVE)) != allocation_type) {
                throw error(error_code::ERROR_INVALID_PARAMETER,
                            "The flAllocationType flag combination is not supported");
            }

            auto region = mem::tmem_region(lpAddress.as_taddr(), dwSize);

            // lpAddress = 0 implies MEM_RESERVE
            if (allocation_type % ALLOCATION_TYPE::RESERVE || region.begin() == 0) {
                if (region.begin() == 0) {
                    region = _mem_mgr.reserve_dynamic(region.size());
                } else {
                    region = _mem_mgr.reserve_fixed(region);
                }
            }

            if (allocation_type % ALLOCATION_TYPE::COMMIT) {
                if (region.begin() == 0)
                    throw win32::error(win32::error_code::ERROR_INVALID_ADDRESS);
                _mem_mgr.commit(region, convert_protection(flProtect));
            }

            return region.begin().as<void>();
        });
    }

    bool KERNEL32_impl::VirtualFree(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t dwFreeType) {
        return KERNEL32_iface::VirtualFree(lpAddress, dwSize, dwFreeType);
    }

    bool
    KERNEL32_impl::VirtualProtect(uwin::mem::tptr<void> lpAddress, std::uint32_t dwSize, std::uint32_t flNewProtect,
                                  uwin::mem::tptr<uint32_t> lpflOldProtect) {
        return KERNEL32_iface::VirtualProtect(lpAddress, dwSize, flNewProtect, lpflOldProtect);
    }

    uint32_t KERNEL32_impl::VirtualQuery(uwin::mem::tcptr<void> lpAddress,
                                         uwin::mem::tptr<uwin::win32::types::MEMORY_BASIC_INFORMATION> lpBuffer,
                                         std::uint32_t dwLength) {
        return handle_error(0, [&]() {

            // logic from NT series; win95 just returns 0 without setting an error on length mismatch
            if (dwLength < sizeof(types::MEMORY_BASIC_INFORMATION))
                throw error(error_code::ERROR_BAD_LENGTH);

            auto &res = _mem_mgr.deref(lpBuffer);

            // other stuff - win95-specific
            res.Type = 0x20000; // hard-coded in win95

            // this is not how it works in windows; TODO
            // seems to depend on which protection you passed when the region was reserved
            // NO DARN IDEA why would you need it, but, well, windows...
            // https://titanwolf.org/Network/Articles/Article?AID=4421f514-9bb8-4983-9181-af3f8b626fd7
            // this ^ seems to say that When AllocationProtect is set to PAGE_NOACCESS, it indicates that the area is initially reserved, rather than directly submitted
            // so maybe makes sense when MEM_RESERVE | MEM_COMMIT is passed...
            res.AllocationProtect = static_cast<std::uint32_t>(PROTECTION::NOACCESS);

            auto query_result = _mem_mgr.query(lpAddress.as_taddr().as_non_const());

            util::visit(query_result, [&](mem::mgr::query_results::free &free) {
                res.State = static_cast<std::uint32_t>(ALLOCATION_TYPE::FREE);
                res.BaseAddress = free.region.begin().value();
                res.RegionSize = free.region.size();
                res.AllocationBase = 0;
                res.AllocationProtect = 0;
                // win95 sets AllocationProtect to zero and does not touch Protect for free region
            }, [&](mem::mgr::query_results::reserved &reserved) {
                res.State = static_cast<std::uint32_t>(ALLOCATION_TYPE::RESERVE);
                res.BaseAddress = reserved.region.begin().value();
                res.RegionSize = reserved.region.size();
                res.AllocationBase = reserved.base_region.begin().value();
                res.Protect = static_cast<std::uint32_t>(PROTECTION::NOACCESS);
            }, [&](mem::mgr::query_results::committed &committed) {
                res.State = static_cast<std::uint32_t>(ALLOCATION_TYPE::COMMIT);
                res.BaseAddress = committed.region.begin().value();
                res.RegionSize = committed.region.size();
                res.AllocationBase = committed.base_region.begin().value();
                res.Protect = convert_protection(committed.protection);
            });

            return sizeof(types::MEMORY_BASIC_INFORMATION);
        });
    }

    bool KERNEL32_impl::IsBadReadPtr(uwin::mem::tcptr<void> lp, std::uint32_t ucb) {
        // TODO: implement it, might be quite useful for additional parameter-checking
        return false; // trust me, it's okay =)
    }

    bool KERNEL32_impl::IsBadWritePtr(uwin::mem::tptr<void> lp, std::uint32_t ucb) {
        return false; // trust me, it's okay =)
    }
}