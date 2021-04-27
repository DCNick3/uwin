//
// Created by dcnick3 on 12/1/20.
//

#include "win32/dll/KERNEL32/impl.h"
#include "util/enumu.h"
#include "util/except.h"
#include "heap/heap.h"

namespace uwin::win32::dll {

    enum class OPTIONS : std::uint32_t {
        HEAP_REALLOC_IN_PLACE_ONLY = 0x00000010,
        HEAP_ZERO_MEMORY = 0x00000008,
        HEAP_GENERATE_EXCEPTIONS = 0x00000004,
        HEAP_NO_SERIALIZE = 0x00000001,
    };
}

namespace uwin {
    template<>
    struct EnableBitMaskOperators<win32::dll::OPTIONS> {
        static const bool enable = true;
    };
}

namespace uwin::win32::dll {
    ht::handle<ht::kobj>
    KERNEL32_impl::HeapCreate(std::uint32_t flOptions, std::uint32_t dwInitialSize, std::uint32_t dwMaximumSize) {
        return handle_error(ht::handle<ht::kobj>(0), [&]() {
            auto options = static_cast<OPTIONS>(flOptions);

            // TODO: check invalid flags?

            auto generate_exceptions = options % OPTIONS::HEAP_GENERATE_EXCEPTIONS;
            auto no_serialize = options % OPTIONS::HEAP_NO_SERIALIZE;

            if (generate_exceptions)
                throw util::not_implemented_error("HEAP_GENERATE_EXCEPTIONS");

            // in win95 (and probably NT too) heap handle is not a handle, but a heap pointer
            // TODO: do we care?..

            return _handletable.emplace<heap::heap>(_mem_mgr,
                                                    util::align_up(dwInitialSize, mem::mgr::consts::page_size),
                                                    util::align_up(dwMaximumSize, mem::mgr::consts::page_size)).as_kobj();
        });
    }

    bool KERNEL32_impl::HeapDestroy(uwin::ht::handle<uwin::ht::kobj> hHeap) {
        return handle_error(false, [&]() {
            _handletable.close(hHeap);
            return true;
        });
    }

    mem::tptr<void>
    KERNEL32_impl::HeapAlloc(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags, std::uint32_t dwBytes) {
        return handle_error({0}, [&]() {
            auto options = static_cast<OPTIONS>(dwFlags);

            // TODO: check invalid flags?

            auto generate_exceptions = options % OPTIONS::HEAP_GENERATE_EXCEPTIONS;
            auto no_serialize = options % OPTIONS::HEAP_NO_SERIALIZE;
            auto zero_memory = options % OPTIONS::HEAP_ZERO_MEMORY;

            if (generate_exceptions)
                throw util::not_implemented_error("HEAP_GENERATE_EXCEPTIONS");

            auto heap = _handletable.get(hHeap.cast<heap::heap>());

            auto res = heap->alloc(dwBytes);

            if (zero_memory)
                memset(_mem_mgr.ptr(res), 0, heap->size(res));

            return res.as<void>();
        });
    }

    mem::tptr<void> KERNEL32_impl::HeapReAlloc(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags,
                                               uwin::mem::tptr<void> lpMem, std::uint32_t dwBytes) {
        return KERNEL32_iface::HeapReAlloc(hHeap, dwFlags, lpMem, dwBytes);
    }

    bool KERNEL32_impl::HeapFree(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags,
                                 uwin::mem::tptr<void> lpMem) {
        return KERNEL32_iface::HeapFree(hHeap, dwFlags, lpMem);
    }

    uint32_t KERNEL32_impl::HeapSize(uwin::ht::handle<uwin::ht::kobj> hHeap, std::uint32_t dwFlags,
                                     uwin::mem::tcptr<void> lpMem) {
        return KERNEL32_iface::HeapSize(hHeap, dwFlags, lpMem);
    }
}
