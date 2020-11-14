//
// Created by dcnick3 on 10/27/20.
//

#pragma once

#include "mem/tptr.h"
#include "util/align.h"

#include <fmt/format.h>

#include <cstdlib>
#include <cstdint>
#include <cassert>

namespace uwin::mem {

    template<typename B, typename S>
    class generic_mem_region {
        B _base;
        S _size;
    public:
        generic_mem_region(B base, S size)
                : _base(base), _size(size) {}

        [[nodiscard]] inline S size() const { return _size; }

        [[nodiscard]] constexpr inline B begin() const { return _base; }

        [[nodiscard]] constexpr inline B end() const { return _base + _size; }

        // checks whether the passed region is fully contained within this region
        // (region \subset this)
        [[nodiscard]] constexpr inline bool does_contain(generic_mem_region<B, S> const &region) const {
            return begin() <= region.begin() && end() >= region.end();
        }

        // checks whether this region is fully contained within the passed region
        // (this \subset region)
        [[nodiscard]] constexpr inline bool is_contained(generic_mem_region<B, S> const &region) const {
            return region.does_contain(*this);
        }

        [[nodiscard]] constexpr inline bool intersects(generic_mem_region<B, S> const &region) const {
            return end() > region.begin() && region.end() > begin();
        }

        [[nodiscard]] constexpr inline generic_mem_region<B, S>
        relative_to(generic_mem_region<B, S> const &region) const {
            return generic_mem_region<B, S>(begin() - region.begin(), size());
        }

        template<typename T>
        constexpr inline generic_mem_region<B, S> align_greedy(T begin_align, T end_align) const {
            // What if not?
            assert(util::is_aligned(begin_align, end_align));
            auto bgn = util::align_down(begin(), begin_align);
            return generic_mem_region<B, S>(bgn, util::align_up(end(), end_align) - bgn);
        }

        template<typename T>
        constexpr inline generic_mem_region<B, S> align_greedy(T alignment) const {
            return align_greedy(alignment, alignment);
        }

        template<typename T>
        constexpr inline auto operator/(T o) const {
            return generic_mem_region(begin() / o, size() / o);
        }

        template<typename T1, typename T2>
        inline bool is_aligned(T1 base_align, T2 size_align) const {
            return util::is_aligned(begin(), base_align) && util::is_aligned(size(), size_align);
        }

        template<typename T1>
        inline bool is_aligned(T1 align) const {
            return is_aligned(align, align);
        }

        inline bool operator==(generic_mem_region<B, S> const &o) const {
            return _base == o._base && _size == o.size();
        }

        inline bool operator!=(generic_mem_region<B, S> const &o) const { return !operator==(o); }
    };

    using hmem_region = generic_mem_region<std::uint8_t *, std::size_t>;

    using tmem_region = generic_mem_region<taddr, taddr::tvalue>;
}


template<>
struct fmt::formatter<uwin::mem::hmem_region> : formatter<string_view> {
    // parse is inherited from formatter<string_view>.
    template<typename FormatContext>
    auto format(uwin::mem::hmem_region c, FormatContext &ctx) {
        std::string name = fmt::format("mem_region({0:p}, 0x{1:x})", fmt::ptr(c.begin()), c.size());
        return formatter<string_view>::format(name, ctx);
    }
};

template<>
struct fmt::formatter<uwin::mem::tmem_region> : formatter<string_view> {
    // parse is inherited from formatter<string_view>.
    template<typename FormatContext>
    auto format(uwin::mem::tmem_region c, FormatContext &ctx) {
        std::string name = fmt::format("mem_region(0x{0:x}, 0x{1:x})", c.begin(), c.size());
        return formatter<string_view>::format(name, ctx);
    }
};