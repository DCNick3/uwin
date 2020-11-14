
#include "win32/dll/dispatcher.h"
#include "ctx/dll.h"

#include <exception>

namespace uwin::win32::dll {
    xcute::remill::Memory* dispatch(uwin::xcute::remill::State *st, std::uint32_t pc, uwin::xcute::remill::Memory* memory) {
        switch (pc) {
        % for key, dll, fun in funs:
            case ${hex(key)}: return ${dll}_iface::${fun}_remill_entry(st, pc, memory);
        % endfor
            default:
                std::terminate();
        }
    }
}