#pragma once

#include <kaitai/kaitaistream.h>

namespace kaitai {

class kstruct {
public:
    explicit kstruct(kstream *_io) { m__io = _io; }
    virtual ~kstruct() = default;
protected:
    kstream *m__io;
public:
    kstream *_io() { return m__io; }
};

}
