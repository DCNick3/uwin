//
// Created by dcnick3 on 11/30/20.
//

#pragma once

namespace uwin::util {
    struct nocopy {
        nocopy() = default;
        nocopy(nocopy const&) = delete;
        nocopy& operator=(nocopy const&) = delete;
    };
}