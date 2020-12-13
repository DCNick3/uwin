//
// Created by dcnick3 on 11/26/20.
//

#pragma once

#include <string>
#include <experimental/source_location>
#include <utility>
#include <fmt/core.h>

namespace uwin::log {
    using source_location = std::experimental::source_location;

    enum class level {
        trace,
        debug,
        info,
        warning,
        error,
        fatal,
    };

    void log(level level, std::string_view tag, std::string const& message, source_location location = source_location::current());

    template <typename... Ts>
    struct trace {
        inline explicit trace(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::trace, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    trace(std::string_view, Ts&&...) -> trace<Ts...>;

    template <typename... Ts>
    struct debug {
        inline explicit debug(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::debug, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    debug(std::string_view, Ts&&...) -> debug<Ts...>;

    template <typename... Ts>
    struct info {
        inline explicit info(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::info, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    info(std::string_view, Ts&&...) -> info<Ts...>;

    template <typename... Ts>
    struct warning {
        inline explicit warning(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::warning, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    warning(std::string_view, Ts&&...) -> warning<Ts...>;

    template <typename... Ts>
    struct error {
        inline explicit error(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::error, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    error(std::string_view, Ts&&...) -> error<Ts...>;

    template <typename... Ts>
    struct fatal {
        inline explicit fatal(std::string_view fmt, Ts... args, source_location location = source_location::current()) {
            log(level::fatal, "", fmt::format(fmt, args...), location);
        }
    };
    template <typename... Ts>
    fatal(std::string_view, Ts&&...) -> fatal<Ts...>;

}