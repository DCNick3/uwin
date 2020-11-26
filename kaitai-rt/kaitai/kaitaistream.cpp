#pragma clang diagnostic push
#pragma ide diagnostic ignored "Simplify"
#include <kaitai/kaitaistream.h>
#include <kaitai/kaitaispanin.h>
#include <kaitai/kaitaistrin.h>

#include <bit>

#include <iostream>
#include <vector>
#include <stdexcept>
#include <climits>

static_assert(std::endian::native == std::endian::little || std::endian::native == std::endian::big,
              "Only big and little endian is supported");

kaitai::kstream::kstream(std::unique_ptr<kin> io) {
    m_io = std::move(io);
    init();
}

void kaitai::kstream::init() {
    align_to_byte();
}

void kaitai::kstream::close() {
    //  m_io->close();
}

// ========================================================================
// Stream positioning
// ========================================================================

bool kaitai::kstream::is_eof() const {
    if (m_bits_left > 0) {
        return false;
    }
   return m_io->is_eof();
}

void kaitai::kstream::seek(uint64_t pos) {
    m_io->seek(pos);
}

uint64_t kaitai::kstream::pos() {
    return m_io->pos();
}

uint64_t kaitai::kstream::size() {
    return m_io->size();
}

// ========================================================================
// Integer numbers
// ========================================================================

// ------------------------------------------------------------------------
// Signed
// ------------------------------------------------------------------------

int8_t kaitai::kstream::read_s1() {
    std::int8_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t*>(&t), 1));
    return t;
}


template <typename T>
T swap_endian(T u)
{
    static_assert (CHAR_BIT == 8, "CHAR_BIT != 8");

    union
    {
        T u;
        unsigned char u8[sizeof(T)];
    } source{}, dest{};

    source.u = u;

    for (size_t k = 0; k < sizeof(T); k++)
        dest.u8[k] = source.u8[sizeof(T) - k - 1];

    return dest.u;
}

// ........................................................................
// Big-endian
// ........................................................................

int16_t kaitai::kstream::read_s2be() {
    int16_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 2));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

int32_t kaitai::kstream::read_s4be() {
    int32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

int64_t kaitai::kstream::read_s8be() {
    int64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

// ........................................................................
// Little-endian
// ........................................................................

int16_t kaitai::kstream::read_s2le() {
    int16_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 2));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

int32_t kaitai::kstream::read_s4le() {
    int32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

int64_t kaitai::kstream::read_s8le() {
    int64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

// ------------------------------------------------------------------------
// Unsigned
// ------------------------------------------------------------------------

uint8_t kaitai::kstream::read_u1() {
    std::uint8_t t;
    m_io->read(std::span(&t, 1));
    return t;
}

// ........................................................................
// Big-endian
// ........................................................................

uint16_t kaitai::kstream::read_u2be() {
    uint16_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 2));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

uint32_t kaitai::kstream::read_u4be() {
    uint32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

uint64_t kaitai::kstream::read_u8be() {
    uint64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return t;
}

// ........................................................................
// Little-endian
// ........................................................................

uint16_t kaitai::kstream::read_u2le() {
    uint16_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 2));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

uint32_t kaitai::kstream::read_u4le() {
    uint32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

uint64_t kaitai::kstream::read_u8le() {
    uint64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return t;
}

// ========================================================================
// Floating point numbers
// ========================================================================

// ........................................................................
// Big-endian
// ........................................................................

float kaitai::kstream::read_f4be() {
    uint32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return reinterpret_cast<float&>(t);
}

double kaitai::kstream::read_f8be() {
    uint64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::little)
        t = swap_endian(t);
    return reinterpret_cast<double&>(t);
}

// ........................................................................
// Little-endian
// ........................................................................

float kaitai::kstream::read_f4le() {
    uint32_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 4));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return reinterpret_cast<float&>(t);
}

double kaitai::kstream::read_f8le() {
    uint64_t t;
    m_io->read(std::span(reinterpret_cast<std::uint8_t *>(&t), 8));
    if (std::endian::native == std::endian::big)
        t = swap_endian(t);
    return reinterpret_cast<double&>(t);
}

// ========================================================================
// Unaligned bit values
// ========================================================================

void kaitai::kstream::align_to_byte() {
    m_bits_left = 0;
    m_bits = 0;
}

uint64_t kaitai::kstream::read_bits_int_be(int n) {
    int bits_needed = n - m_bits_left;
    if (bits_needed > 0) {
        // 1 bit  => 1 byte
        // 8 bits => 1 byte
        // 9 bits => 2 bytes
        int bytes_needed = ((bits_needed - 1) / 8) + 1;
        if (bytes_needed > 8)
            throw std::runtime_error("read_bits_int: more than 8 bytes requested");
        std::uint8_t buf[8];
        m_io->read(std::span(buf, bytes_needed));
        for (int i = 0; i < bytes_needed; i++) {
            uint8_t b = buf[i];
            m_bits <<= 8;
            m_bits |= b;
            m_bits_left += 8;
        }
    }

    // raw mask with required number of 1s, starting from lowest bit
    uint64_t mask = get_mask_ones(n);
    // shift mask to align with highest bits available in @bits
    int shift_bits = m_bits_left - n;
    mask <<= shift_bits;
    // derive reading result
    uint64_t res = (m_bits & mask) >> shift_bits;
    // clear top bits that we've just read => AND with 1s
    m_bits_left -= n;
    mask = get_mask_ones(m_bits_left);
    m_bits &= mask;

    return res;
}

// Deprecated, use read_bits_int_be() instead.
uint64_t kaitai::kstream::read_bits_int(int n) {
    return read_bits_int_be(n);
}

uint64_t kaitai::kstream::read_bits_int_le(int n) {
    int bits_needed = n - m_bits_left;
    if (bits_needed > 0) {
        // 1 bit  => 1 byte
        // 8 bits => 1 byte
        // 9 bits => 2 bytes
        int bytes_needed = ((bits_needed - 1) / 8) + 1;
        if (bytes_needed > 8)
            throw std::runtime_error("read_bits_int_le: more than 8 bytes requested");
        std::uint8_t buf[8];
        m_io->read(std::span(buf, bytes_needed));
        for (int i = 0; i < bytes_needed; i++) {
            uint8_t b = buf[i];
            m_bits |= (static_cast<uint64_t>(b) << m_bits_left);
            m_bits_left += 8;
        }
    }

    // raw mask with required number of 1s, starting from lowest bit
    uint64_t mask = get_mask_ones(n);
    // derive reading result
    uint64_t res = m_bits & mask;
    // remove bottom bits that we've just read by shifting
    m_bits >>= n;
    m_bits_left -= n;

    return res;
}

uint64_t kaitai::kstream::get_mask_ones(int n) {
    if (n == 64) {
        return 0xFFFFFFFFFFFFFFFF;
    } else {
        return ((uint64_t) 1 << n) - 1;
    }
}

// ========================================================================
// Byte arrays
// ========================================================================

std::string kaitai::kstream::read_bytes(std::streamsize len) {
    std::vector<std::uint8_t> result(len);

    // NOTE: streamsize type is signed, negative values are only *supposed* to not be used.
    // http://en.cppreference.com/w/cpp/io/streamsize
    if (len < 0) {
        throw std::runtime_error("read_bytes: requested a negative amount");
    }

    if (len > 0) {
        m_io->read(std::span(&result[0], len));
    }

    return std::string(result.begin(), result.end());
}

std::string kaitai::kstream::read_bytes_full() {
    std::iostream::pos_type p1 = m_io->pos();
    std::iostream::pos_type p2 = m_io->size();
    size_t len = p2 - p1;

    // Note: this requires a std::string to be backed with a
    // contiguous buffer. Officially, it's a only requirement since
    // C++11 (C++98 and C++03 didn't have this requirement), but all
    // major implementations had contiguous buffers anyway.
    std::string result(len, ' ');
    m_io->read(std::span(reinterpret_cast<std::uint8_t*>(&result[0]), len));

    return result;
}

std::string kaitai::kstream::read_bytes_term(char term, bool include, bool consume, bool eos_error) {
    std::string result;
    while (true) {
        if (m_io->is_eof()) {
            if (eos_error) {
                throw std::runtime_error("read_bytes_term: encountered EOF");
            }
            break;
        }

        std::uint8_t c;
        m_io->read(std::span(&c, 1));
        if (c != term) {
            result.push_back(term);
        } else {
            if (include)
                result.push_back(term);
            if (!consume)
                m_io->seek(m_io->pos() - 1);
            break;
        }
    }
    return result;
}

std::string kaitai::kstream::ensure_fixed_contents(const std::string& expected) {
    std::string actual = read_bytes(expected.length());

    if (actual != expected) {
        // NOTE: I think printing it outright is not best idea, it could contain non-ascii charactes like backspace and beeps and whatnot. It would be better to print hexlified version, and also to redirect it to stderr.
        throw std::runtime_error("ensure_fixed_contents: actual data does not match expected data");
    }

    return actual;
}

std::string kaitai::kstream::bytes_strip_right(std::string src, char pad_byte) {
    std::size_t new_len = src.length();

    while (new_len > 0 && src[new_len - 1] == pad_byte)
        new_len--;

    return src.substr(0, new_len);
}

std::string kaitai::kstream::bytes_terminate(std::string src, char term, bool include) {
    std::size_t new_len = 0;
    std::size_t max_len = src.length();

    while (new_len < max_len && src[new_len] != term)
        new_len++;

    if (include && new_len < max_len)
        new_len++;

    return src.substr(0, new_len);
}

// ========================================================================
// Byte array processing
// ========================================================================

std::string kaitai::kstream::process_xor_one(std::string data, uint8_t key) {
    size_t len = data.length();
    std::string result(len, ' ');

    for (size_t i = 0; i < len; i++)
        result[i] = (unsigned)data[i] ^ (unsigned)key;

    return result;
}

std::string kaitai::kstream::process_xor_many(std::string data, std::string key) {
    size_t len = data.length();
    size_t kl = key.length();
    std::string result(len, ' ');

    size_t ki = 0;
    for (size_t i = 0; i < len; i++) {
        result[i] = (unsigned)data[i] ^ (unsigned)key[ki];
        ki++;
        if (ki >= kl)
            ki = 0;
    }

    return result;
}

std::string kaitai::kstream::process_rotate_left(std::string data, int amount) {
    size_t len = data.length();
    std::string result(len, ' ');

    for (size_t i = 0; i < len; i++) {
        uint8_t bits = data[i];
        result[i] = (unsigned)(bits << (unsigned)amount) | (unsigned)(bits >> (unsigned)(8 - amount));
    }

    return result;
}

// ========================================================================
// Misc utility methods
// ========================================================================

int kaitai::kstream::mod(int a, int b) {
    if (b <= 0)
        throw std::invalid_argument("mod: divisor b <= 0");
    int r = a % b;
    if (r < 0)
        r += b;
    return r;
}

std::string kaitai::kstream::to_string(int val) {
    return std::to_string(val);
}

#include <algorithm>
std::string kaitai::kstream::reverse(std::string val) {
    std::reverse(val.begin(), val.end());

    return val;
}

uint8_t kaitai::kstream::byte_array_min(const std::string& val) {
    uint8_t min = 0xff; // UINT8_MAX
    std::string::const_iterator end = val.end();
    for (std::string::const_iterator it = val.begin(); it != end; ++it) {
        auto cur = static_cast<uint8_t>(*it);
        if (cur < min) {
            min = cur;
        }
    }
    return min;
}

uint8_t kaitai::kstream::byte_array_max(const std::string& val) {
    uint8_t max = 0; // UINT8_MIN
    std::string::const_iterator end = val.end();
    for (std::string::const_iterator it = val.begin(); it != end; ++it) {
        auto cur = static_cast<uint8_t>(*it);
        if (cur > max) {
            max = cur;
        }
    }
    return max;
}

// ========================================================================
// Other internal methods
// ========================================================================

std::string kaitai::kstream::bytes_to_str(std::string src, [[maybe_unused]] const std::string& src_enc) {
    return src;
}

kaitai::kstream::kstream(std::span<const std::uint8_t> data)
    : kaitai::kstream(std::make_unique<kaitai::kspanin>(data)) {
}

kaitai::kstream::kstream(const std::string& data)
    : kaitai::kstream(std::make_unique<kaitai::kstrin>(data)) {
}

#pragma clang diagnostic pop