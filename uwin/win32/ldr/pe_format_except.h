//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include <stdexcept>
// TODO: migrate to std::source_location when it gets implemented
#include <experimental/source_location>

#include <fmt/core.h>

template <>
struct fmt::formatter<std::experimental::source_location> {
    static constexpr auto parse(format_parse_context& ctx) {
        // Return an iterator past the end of the parsed range:
        return ctx.begin();
    }

    // Formats the point p using the parsed format specification (presentation)
    // stored in this formatter.
    template <typename FormatContext>
    auto format(const std::experimental::source_location& p, FormatContext& ctx) {
        // auto format(const point &p, FormatContext &ctx) -> decltype(ctx.out()) // c++11
        // ctx.out() is an output iterator to write to.
        return format_to(
                ctx.out(),
                "{}:{}:{}", p.file_name(), p.line(), p.column());
    }
};

namespace uwin::win32::ldr {
    class pe_format_exception : public std::runtime_error {
    public:
        explicit pe_format_exception(std::experimental::source_location loc = std::experimental::source_location())
            : runtime_error(
                fmt::format("Attempt to load PE file with invalid format was made at {}", loc)) {}
    };
}
