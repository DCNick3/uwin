//
// Created by dcnick3 on 11/30/20.
//

#pragma once

namespace uwin::util {
    struct nomove {
        nomove() = default;
        nomove(nomove&&) = delete;
        nomove& operator=(nomove&&) = delete;
    };
}