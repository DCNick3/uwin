//
// Created by dcnick3 on 4/26/21.
//

#pragma once

#include <string>

namespace uwin::str {
    namespace detail {

        template<typename C, typename TAG>
        class generic_str : std::basic_string<C>
        {
            using S = std::basic_string<C>;
            using V = std::basic_string_view<C>;
        public:
            class view : V
            {
            public:
                inline explicit view(V s) : V(s) {}
                [[nodiscard]] inline V raw_view() const { return *this; }

                using V::size;
                using V::data;
            };

            inline explicit generic_str(typename S::const_pointer s) : S(s) {}
            inline explicit generic_str(S s) : S(s) {}
            inline explicit generic_str(V s) : S(std::move(s)) {}

            using S::size;
            using S::data;

            inline bool operator==(generic_str const& o) {
                return raw_view() == o.raw_view();
            }
            inline bool operator==(S const& o) {
                return raw_view() == o;
            }
            inline bool operator==(V o) {
                return raw_view() == o;
            }
            inline bool operator==(C const* o) {
                return raw_view() == o;
            }

            [[nodiscard]] inline V raw_view() const { return *this; }
            [[nodiscard]] inline S const& raw_str() const { return *this; }

            [[nodiscard]] inline operator view() const { // NOLINT(google-explicit-constructor)
                return view(raw_view());
            }
        };

        struct narrow_tag {};
        struct native_tag {};
        struct wide_tag {};
    }

    using narrow = detail::generic_str<char, detail::narrow_tag>;
    using narrow_view = narrow::view;

    using native = detail::generic_str<char, detail::native_tag>;
    using native_view = native::view;

    using wide = detail::generic_str<char16_t, detail::wide_tag>;
    using wide_view = wide::view;

}