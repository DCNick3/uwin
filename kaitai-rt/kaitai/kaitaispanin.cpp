//
// Created by dcnick3 on 11/25/20.
//

#include <kaitai/kaitaispanin.h>

#include <cstring>

kaitai::kspanin::kspanin(std::span<const std::uint8_t> data)
        : _data(data), _left(data) {}

void kaitai::kspanin::close() {}

bool kaitai::kspanin::is_eof() const {
    return _left.empty();
}

void kaitai::kspanin::seek(uint64_t pos) {
    _left = _data.subspan(pos);
}

uint64_t kaitai::kspanin::pos() {
    return _left.begin() - _data.begin();
}

uint64_t kaitai::kspanin::size() {
    return _data.size();
}

void kaitai::kspanin::read(std::span<std::uint8_t> target) {
    auto read = _left.subspan(0, target.size());
    memcpy(&*target.begin(), &*read.begin(), read.size_bytes());
    _left = _left.subspan(target.size());
}
