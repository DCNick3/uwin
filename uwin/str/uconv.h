//
// Created by dcnick3 on 4/26/21.
//

#pragma once

#include <string>
#include <type_traits>
#include <span>

#include "str/str.h"

namespace uwin::str {
    class uconv_error : public std::runtime_error {
    public:
        explicit uconv_error(const std::string &arg) : runtime_error(arg) {}
    };

    template<typename From, typename To>
    class converter {
    public:
        // performs a conversion
        // modifies spans for seemsless continuation of conversion (subspans them to the unused area)
        // this may buffer some amount of data inside if the output was not big enough (can be checked by checking the size of output after the conversion)
        virtual void convert(std::span<const typename From::char_t>& input,
                          std::span<typename To::char_t>& output,
                          bool flush) = 0;

        // should return an ultra-conservative check. not allowed to return less than would be actually required
        virtual std::size_t approximate_output_size(std::size_t input_size) = 0;

        virtual ~converter() = default;
    };

    std::unique_ptr<converter<wide, native>> cvt_wide_to_native();
    std::unique_ptr<converter<native, wide>> cvt_native_to_wide();

    std::unique_ptr<converter<narrow, wide>> cvt_narrow_to_wide(std::uint32_t codepage);
    std::unique_ptr<converter<narrow, native>> cvt_narrow_to_native(std::uint32_t codepage);

    std::unique_ptr<converter<native, narrow>> cvt_native_to_narrow(std::uint32_t codepage);
    std::unique_ptr<converter<wide, narrow>> cvt_wide_to_narrow(std::uint32_t codepage);

    native wide_to_native(wide_view src);
    wide native_to_wide(native_view src);

    wide narrow_to_wide(std::uint32_t codepage, narrow_view src);
    native narrow_to_native(std::uint32_t codepage, narrow_view src);

    narrow native_to_narrow(std::uint32_t codepage, native_view src);
    narrow wide_to_narrow(std::uint32_t codepage, wide_view src);
}