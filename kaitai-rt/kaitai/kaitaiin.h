//
// Created by dcnick3 on 11/25/20.
//

#pragma once

#include <cstdint>
#include <span>

namespace kaitai {
    class kin {
    public:
        virtual void close() = 0;
        /** @name Stream positioning */
        //@{
        /**
         * Check if stream pointer is at the end of stream. Note that the semantics
         * are different from traditional STL semantics: one does *not* need to do a
         * read (which will fail) after the actual end of the stream to trigger EOF
         * flag, which can be accessed after that read. It is sufficient to just be
         * at the end of the stream for this method to return true.
         * \return "true" if we are located at the end of the stream.
         */
        [[nodiscard]] virtual bool is_eof() const = 0;

        /**
         * Set stream pointer to designated position.
         * \param pos new position (offset in bytes from the beginning of the stream)
         */
        virtual void seek(uint64_t pos) = 0;

        /**
         * Get current position of a stream pointer.
         * \return pointer position, number of bytes from the beginning of the stream
         */
        virtual uint64_t pos() = 0;

        /**
         * Get total size of the stream in bytes.
         * \return size of the stream in bytes
         */
        virtual uint64_t size() = 0;

        virtual void read(std::span<std::uint8_t> target) = 0;
    };
}