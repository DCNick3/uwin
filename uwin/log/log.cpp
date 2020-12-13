//
// Created by dcnick3 on 11/26/20.
//

#include <iostream>
#include <chrono>

#include <fmt/chrono.h>

#include "log/log.h"

namespace uwin::log {

    void log(level level, std::string_view tag, std::string const& message, source_location location) {
        std::string_view level_text;
        switch (level) {
            case level::trace:
                level_text = "TRC";
                break;
            case level::debug:
                level_text = "DBG";
                break;
            case level::info:
                level_text = "INF";
                break;
            case level::warning:
                level_text = "WRN";
                break;
            case level::error:
                level_text = "ERR";
                break;
            case level::fatal:
                level_text = "FTL";
                break;
            default:
                std::terminate();
        }

        std::time_t t = std::time(nullptr);

        auto prefix = fmt::format("{0:%Y-%m-%d %H:%M:%S}  {1} {2:>{4}}:{3:<{5}} | ",
                                  *std::localtime(&t),
                                  level_text, location.file_name(), location.line(),
                                  40, 4);

        std::cerr << fmt::format("{0} {1}\n", prefix, message);

        if (level == level::fatal)
            std::terminate();
    }
}