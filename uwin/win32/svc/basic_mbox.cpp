//
// Created by dcnick3 on 12/25/20.
//

#include "win32/svc/basic_mbox.h"

#include "util/except.h"
#include "log/log.h"

namespace uwin::win32::svc {

    mbox::RESULT
    basic_mbox::show(std::string text, std::string caption, mbox::TYPE type) {
        sanitize_type(type);

        if (type != mbox::TYPE::TYPE_OK)
            throw util::not_implemented_error("Message box other then OK");

        log::info("basic_mbox::show({}, {}, {:x})", text, caption, static_cast<std::uint32_t>(type));

        return RESULT::OK;
    }

}