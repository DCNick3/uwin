//
// Created by dcnick3 on 4/26/21.
//

#include <unicode/unistr.h>
#include <unicode/ucnv.h>
#include <unicode/errorcode.h>
#include <fmt/core.h>

#include "util/except.h"
#include "str/uconv.h"

namespace uwin::str {
    native wide_to_native(wide_view src) {
        icu::UnicodeString s(src.data(), (std::int32_t)src.size());
        std::string res;
        s.toUTF8String(res);

        return native(std::move(res));
    }

    wide native_to_wide(native_view src) {
        auto us = icu::UnicodeString::fromUTF8(src.raw_view());
        std::u16string res;

        res.resize(us.length());
        icu::ErrorCode status;
        us.extract(res.data(), us.length(), status);
        if (status.isFailure())
            throw icu_error(status.errorName());

        return wide(std::move(res));
    }

    // TODO: are encodings in the icu REALLY the same as MS?
    // Maybe I should use MS data?...
    // Overall, it seems that doing something similar to what what wine is doing with their make_unicode script might be a good idea
    // But it's too much to implement; kludges will be enough for now
    static std::int32_t codepage_to_ccsid(std::uint32_t cp) {
        switch (cp) {
            case 1251: return 5347; // use euro-extended page
            case 1252: return 5348; // use euro-extended page
            case 932:  return 943;
            default:
                throw util::not_implemented_error(fmt::format("Code page {}", cp));
        }
    }

    struct UConverterDeleter {
        void operator()(UConverter* p) const {
            ucnv_close(p);
        }
    };

    static std::unique_ptr<UConverter, UConverterDeleter> open_converter(std::int32_t ccsid) {
        icu::ErrorCode status;
        auto result = ucnv_openCCSID(ccsid, UCNV_IBM, status);
        if (status.isFailure())
            throw icu_error(status.errorName());
        return std::unique_ptr<UConverter, UConverterDeleter>(result);
    }

    wide narrow_to_wide(std::uint32_t codepage, narrow_view src_view) {
        auto conv = open_converter(codepage_to_ccsid(codepage));

        std::u16string target_str;
        target_str.resize(src_view.size() / ucnv_getMinCharSize(conv.get()));

        auto src = src_view.data();
        auto src_end = src_view.data() + src_view.size();

        std::size_t pos = 0;
        icu::ErrorCode status;
        while (true) {
            auto target = target_str.data();
            auto target_end = target_str.data() + target_str.size();
            ucnv_toUnicode(conv.get(), &target, target_end, &src, src_end, nullptr, true, status);
            pos = target - target_str.data();

            if (status == U_BUFFER_OVERFLOW_ERROR) {
                target_str.resize(target_str.size() * 2);
                continue;
            } else if (status.isFailure()) {
                throw icu_error(status.errorName());
            }
            break;
        }

        target_str.resize(pos);

        return wide(std::move(target_str));
    }

    native narrow_to_native(std::uint32_t codepage, narrow_view src) {
        return wide_to_native(narrow_to_wide(codepage, src));
    }


    narrow wide_to_narrow(std::uint32_t codepage, wide_view src_view) {
        auto conv = open_converter(codepage_to_ccsid(codepage));

        std::string target_str;
        target_str.resize(src_view.size());

        auto src = src_view.data();
        auto src_end = src_view.data() + src_view.size();

        std::size_t pos = 0;
        icu::ErrorCode status;
        while (true) {
            auto target = target_str.data();
            auto target_end = target_str.data() + target_str.size();
            ucnv_fromUnicode(conv.get(), &target, target_end, &src, src_end, nullptr, true, status);
            pos = target - target_str.data();

            if (status == U_BUFFER_OVERFLOW_ERROR) {
                target_str.resize(target_str.size() * 2);
                continue;
            } else if (status.isFailure()) {
                throw icu_error(status.errorName());
            }
            break;
        }

        target_str.resize(pos);

        return narrow(std::move(target_str));
    }

    narrow native_to_narrow(std::uint32_t codepage, native_view src) {
        return wide_to_narrow(codepage, native_to_wide(src));
    }
}