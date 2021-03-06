#pragma once

#include "xcute/remill/remill_state.h"

namespace uwin::win32::dll {
    xcute::remill::Memory *
    dispatch_native(uwin::xcute::remill::StateEx& st, std::uint32_t pc, uwin::xcute::remill::Memory *memory);
}