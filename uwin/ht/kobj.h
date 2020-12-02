//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "util/nomove.h"
#include "util/nocopy.h"
#include "ht/except.h"
#include "ht/handle.h"

#include <cstdlib>

namespace uwin::ht {
    struct kobj : util::nomove, util::nocopy {
        virtual ~kobj() = default;
    };
}