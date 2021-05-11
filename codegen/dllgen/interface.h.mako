
#pragma once

#include "mem/tptr.h"
#include "win32/error.h"
#include "win32/types/types.h"
#include "win32/dll/native_module.h"
#include "ht/handle.h"
#include "ht/kobj.h"
#include "win32/uconv.h"

#include "xcute/remill/remill_state.h"
#include "xcute/remill/remill_rt.h"

#include <cstdint>

namespace uwin::win32::dll {
    class ${dll_name}_iface : public native_module {
    public:
        explicit inline ${dll_name}_iface(mem::mgr::target_mem_mgr &target_mem_mgr, uconv &uconv)
            : native_module(target_mem_mgr, uconv, "${dll_name}.DLL") {}
    % for i, fun in funs:
        virtual ${repr(fun)};
    % endfor

    % for i, fun in funs:
        void ${fun.name}_raw_call(uwin::xcute::remill::StateEx& st);
    % endfor

    % for i, fun in funs:
        static uwin::xcute::remill::Memory *${fun.name}_remill_entry(uwin::xcute::remill::StateEx& st,
            std::uint32_t pc, uwin::xcute::remill::Memory* memory);
    % endfor

        [[nodiscard]] virtual mem::tcaddr try_resolve(const std::string& symbol_name) const override;
    };
}
