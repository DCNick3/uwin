//
// Created by dcnick3 on 5/19/21.
//

#pragma once

#include <unicode/unistr.h>
#include <unicode/ucnv.h>
#include <unicode/errorcode.h>

#include "util/except.h"
#include "str/uconv.h"


namespace uwin::str {
    namespace {
        struct ucnv_deleter {
            void operator()(UConverter *p) const {
                ucnv_close(p);
            }
        };

        using ucnv_ptr = std::unique_ptr<UConverter, ucnv_deleter>;

        ucnv_ptr open_converter(std::int32_t ccsid) {
            icu::ErrorCode status;
            auto result = ucnv_openCCSID(ccsid, UCNV_IBM, status);
            if (status.isFailure())
                throw uconv_error(status.errorName());
            return std::unique_ptr<UConverter, ucnv_deleter>(result);
        }

        ucnv_ptr open_converter(const char *name) {
            icu::ErrorCode status;
            auto result = ucnv_open(name, status);
            if (status.isFailure())
                throw uconv_error(status.errorName());
            return std::unique_ptr<UConverter, ucnv_deleter>(result);
        }

        template<typename To>
        class cvt_from_wide_impl : public converter<wide, To> {
        public:
            ucnv_ptr _cnv;

            explicit cvt_from_wide_impl(ucnv_ptr cnv) : _cnv(std::move(cnv)) {
            }

            static_assert(std::is_same_v<typename To::char_t, char>);

            void
            convert(std::span<const wide::char_t> &input, std::span<typename To::char_t> &output, bool flush) override {
                auto source = &*input.begin();
                auto source_end = &*input.end();
                auto target = &*output.begin();
                auto target_end = &*output.end();

                icu::ErrorCode status;

                ucnv_fromUnicode(_cnv.get(), &target, target_end, &source, source_end, nullptr,
                                 static_cast<UBool>(flush), status);

                input = input.subspan(source - &*input.begin());
                output = output.subspan(target - &*output.begin());

                if (status.isFailure() && status.get() != U_BUFFER_OVERFLOW_ERROR)
                    throw uconv_error(status.errorName());
            }

            size_t approximate_output_size(std::size_t input_size) override {
                return input_size * ucnv_getMaxCharSize(_cnv.get());
            }
        };

        template<typename From>
        class cvt_to_wide_impl : public converter<From, wide> {
        public:
            ucnv_ptr _cnv;

            explicit cvt_to_wide_impl(ucnv_ptr cnv) : _cnv(std::move(cnv)) {
            }

            static_assert(std::is_same_v<typename From::char_t, char>);

            void convert(std::span<const typename From::char_t> &input, std::span<wide::char_t> &output,
                         bool flush) override {
                auto source = &*input.begin();
                auto source_end = &*input.end();
                auto target = &*output.begin();
                auto target_end = &*output.end();

                icu::ErrorCode status;

                ucnv_toUnicode(_cnv.get(), &target, target_end, &source, source_end, nullptr, static_cast<UBool>(flush),
                               status);

                input = input.subspan(source - &*input.begin());
                output = output.subspan(target - &*output.begin());

                if (status.isFailure() && status.get() != U_BUFFER_OVERFLOW_ERROR)
                    throw uconv_error(status.errorName());
            }

            size_t approximate_output_size(std::size_t input_size) override {
                return input_size / ucnv_getMinCharSize(_cnv.get());
            }
        };

        template<typename From, typename Intermediate, typename To>
        class cvt_composition : public converter<From, To> {
        public:
            std::unique_ptr<converter < From, Intermediate>> _step_one;
            std::unique_ptr<converter < Intermediate, To>> _step_two;
            std::array<typename Intermediate::char_t, 16> _intermediate_buffer{};
            std::span<const typename Intermediate::char_t> _intermediate_not_consumed_buffer{};

            explicit cvt_composition(std::unique_ptr<converter<From, Intermediate>> step_one,
                std::unique_ptr<converter < Intermediate, To>> step_two)
            : _step_one(std::move(step_one)),
                _step_two(std::move(step_two)) {
            }

            void
            convert(std::span<const typename From::char_t> &input, std::span<typename To::char_t> &output,
                    bool flush) override {

                while (!_intermediate_not_consumed_buffer.empty() && !output.empty()) {
                    _step_two->convert(_intermediate_not_consumed_buffer, output, false);
                }

                while (!input.empty() && !output.empty()) {
                    std::span<typename Intermediate::char_t> int_span_1(_intermediate_buffer);
                    _step_one->convert(input, int_span_1, flush);

                    _intermediate_not_consumed_buffer = std::span(_intermediate_buffer.data(), _intermediate_buffer.size() - int_span_1.size());
                    _step_two->convert(_intermediate_not_consumed_buffer, output, flush);
                }

                //throw util::not_implemented_error("cvt_composition");
            }

            size_t approximate_output_size(std::size_t input_size) override {
                return _step_two->approximate_output_size(_step_one->approximate_output_size(input_size));
            }
        };

        template<typename From, typename Intermediate, typename To>
        auto make_unique_cvt_composition(std::unique_ptr<converter<From, Intermediate>> step_one,
                    std::unique_ptr<converter<Intermediate, To>> step_two) {
            return std::make_unique<cvt_composition<From, Intermediate, To>>(std::move(step_one), std::move(step_two));
        }

        // TODO: are encodings in the icu REALLY the same as MS?
        // Maybe I should use MS data?...
        // Overall, it seems that doing something similar to what what wine is doing with their make_unicode script might be a good idea
        // But it's too much to implement, I think; kludges will be enough for now
        static std::int32_t codepage_to_ccsid(std::uint32_t cp) {
            switch (cp) {
                case 1251:
                    return 5347; // use euro-extended page
                case 1252:
                    return 5348; // use euro-extended page
                case 932:
                    return 943;
                default:
                    throw util::not_implemented_error(fmt::format("Code page {}", cp));
            }
        }

        template<typename From, typename To>
        To string_uconv_with_cvt(converter<From, To>& cvt, typename From::view src_view) {
            typename To::raw_string_t target_str;
            // initially allocate the buffer
            target_str.resize(cvt.approximate_output_size(src_view.size()));

            std::span src(src_view.data(), src_view.size());
            std::span dst(target_str.data(), target_str.size());

            cvt.convert(src, dst, true);

            if (!src.empty())
                throw std::runtime_error("cvt.approximate_output_size lied to us!");

            target_str.resize(target_str.size() - dst.size());

            return To(std::move(target_str));
        }
    }
}
