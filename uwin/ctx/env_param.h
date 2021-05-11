//
// Created by dcnick3 on 5/12/21.
//

#pragma once

namespace uwin::ctx {
    struct env_param {
        // TODO: make sure that boost::di will not inject it automagically
        str::native command_line;
        std::vector<str::narrow> environment;
        std::uint32_t ansi_codepage;
        std::uint32_t oem_codepage;
    };
}
