//
// Created by dcnick3 on 12/24/20.
//

#pragma once

#include "util/enumu.h"
#include "win32/error.h"

#include <string>

namespace uwin::win32::svc {
    class mbox {
    public:
        enum class RESULT : std::int32_t {
            ERROR = 0, // not returned, exception is thrown instead

            OK = 1,
            CANCEL = 2,
            ABORT = 3,
            RETRY = 4,
            IGNORE = 5,
            YES = 6,
            NO = 7,
        };

        enum class TYPE : std::uint32_t {
            TYPE_OK = 0x0,
            TYPE_OKCANCEL = 0x1,
            TYPE_ABORTRETRYIGNORE = 0x2,
            TYPE_YESNOCANCEL = 0x3,
            TYPE_YESNO = 0x4,
            TYPE_RETRYCANCEL = 0x5,
            TYPE_MASK = 0xf,

            ICON_NONE = 0x00,
            ICON_HAND = 0x10,
            ICON_QUESTION = 0x20,
            ICON_EXCLAMATION = 0x30,
            ICON_ASTERISK = 0x40,
            ICON_MASK = 0xf0,

            DEFBUTTON1 = 0x000,
            DEFBUTTON2 = 0x100,
            DEFBUTTON3 = 0x200,
            DEFBUTTON_MASK = 0xf00,

            MODAL_APP = 0x0000,
            MODAL_SYSTEM = 0x1000,
            MODAL_TASK = 0x2000,
            MODAL_MASK = 0xf000,

            OPT_NOFOCUS = 0x08000,
            OPT_SETFOREGROUND = 0x10000,
        };

        // How do we handle windows, again?
        // For now I will not put the hWnd parameter here, as windows implementation (will be)is a stub anyways
        virtual RESULT show(std::string text, std::string caption, TYPE type) = 0;

    protected:
        static inline void sanitize_type(TYPE type_composite);
    };
}

namespace uwin {
    template<>
    struct EnableBitMaskOperators<win32::svc::mbox::TYPE> {
        static const bool enable = true;
    };
}

namespace uwin::win32::svc {
    void mbox::sanitize_type(TYPE type_composite) {
        auto type = type_composite & TYPE::TYPE_MASK;
        auto icon = type_composite & TYPE::ICON_MASK;
        auto defbutton = type_composite & TYPE::DEFBUTTON_MASK;
        auto modality = type_composite & TYPE::MODAL_MASK;
        //auto opt_nofocus = type_composite % TYPE::OPT_NOFOCUS;
        //auto opt_setforeground = type_composite % TYPE::OPT_SETFOREGROUND;
        if (!(type == TYPE::TYPE_OK ||
                type == TYPE::TYPE_OKCANCEL ||
                type == TYPE::TYPE_ABORTRETRYIGNORE ||
                type == TYPE::TYPE_YESNOCANCEL ||
                type == TYPE::TYPE_YESNO ||
                type == TYPE::TYPE_RETRYCANCEL))
            throw win32::error(win32::error_code::ERROR_INVALID_PARAMETER);
        if (!(icon == TYPE::ICON_NONE ||
                icon == TYPE::ICON_HAND ||
                icon == TYPE::ICON_QUESTION ||
                icon == TYPE::ICON_EXCLAMATION ||
                icon == TYPE::ICON_ASTERISK))
            throw win32::error(win32::error_code::ERROR_INVALID_PARAMETER);
        if (!(defbutton == TYPE::DEFBUTTON1 ||
                defbutton == TYPE::DEFBUTTON2 ||
                defbutton == TYPE::DEFBUTTON3))
            throw win32::error(win32::error_code::ERROR_INVALID_PARAMETER);
        if (!(modality == TYPE::MODAL_APP ||
                modality == TYPE::MODAL_SYSTEM ||
                modality == TYPE::MODAL_TASK))
            throw win32::error(win32::error_code::ERROR_INVALID_PARAMETER);
    }
}