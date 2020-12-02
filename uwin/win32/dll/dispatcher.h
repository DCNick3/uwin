#pragma once

#include "xcute/remill/remill_state.h"

namespace uwin::win32::dll {
    xcute::remill::Memory *
    dispatch(uwin::xcute::remill::State& st, std::uint32_t pc, uwin::xcute::remill::Memory *memory);
}