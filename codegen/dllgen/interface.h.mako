
#pragma once

#include "mem/tptr.h"
#include "win32/error.h"
#include "win32/types/types.h"
#include "win32/dll/base.h"

#include "xcute/remill/remill_state.h"
#include "xcute/remill/remill_rt.h"

#include <cstdint>

namespace uwin {
    namespace win32 {
        namespace dll {
            class ${dll_name}_iface : public base {
            public:
                explicit inline ${dll_name}_iface(ctx::process &process_ctx) : base(process_ctx) {}
            % for i, fun in funs:
                virtual ${repr(fun)};
            % endfor

            % for i, fun in funs:
                void ${fun.name}_raw_call(uwin::xcute::remill::State *st);
            % endfor

            % for i, fun in funs:
                static uwin::xcute::remill::Memory *${fun.name}_remill_entry(uwin::xcute::remill::State *st,
                    std::uint32_t pc, uwin::xcute::remill::Memory* memory);
            % endfor

                virtual mem::taddr::tvalue resolve(std::string& name) const;
            };
        }
    }
}
