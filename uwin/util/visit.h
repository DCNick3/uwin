//
// Created by dcnick3 on 4/19/21.
//

#pragma once

#include <variant>

namespace uwin::util {
    // see https://bitbashing.io/std-visit.html
    template<class... Ts> struct overloaded : Ts... { using Ts::operator()...; };
    template<class... Ts> overloaded(Ts...) -> overloaded<Ts...>;

    template<typename T, typename... Ts>
    auto visit(T var, Ts&&... args) {
        return std::visit(overloaded {
            std::forward<Ts>(args)...
        }, var);
    }
}
