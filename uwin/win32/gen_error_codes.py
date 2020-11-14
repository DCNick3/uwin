import os
import csv
import sys
import json

dir = os.path.dirname(os.path.realpath(__file__))
errors = []

with open(dir + "/winerror.csv") as f:
    reader = csv.reader(f)
    next(reader)  # skip header
    for code, slug, desc in reader:
        code = int(code, 10)
        errors.append((code, slug, desc))


def cstr(s):
    return json.dumps(s)  # well, all JSON strings are valid C string literals


os.makedirs(os.path.dirname(sys.argv[1]), exist_ok=True)
os.makedirs(os.path.dirname(sys.argv[2]), exist_ok=True)
sys.stdout = open(sys.argv[1], 'w')

print("""#pragma once

#include <cstdint>

namespace uwin {
    namespace win32 {
        enum class error_code : std::uint32_t {""")

for code, slug, desc in errors:
    print("            %s = %s," % (slug, code))

print("""        };
    }
}""")

sys.stdout = open(sys.argv[2], 'w')
print("""#include "win32/error_code.h"

#include <exception>

namespace uwin {
    namespace win32 {
        const char* error_code_to_slug(error_code code) {
            switch (static_cast<long>(code)) {""")

for code, slug, desc in errors:
    print("                case %s: return %s;" % (code, cstr(slug)))

print("""                default: std::terminate();
            }
        }
        const char* error_code_to_desc(error_code code) {
            switch (static_cast<long>(code)) {""")

for code, slug, desc in errors:
    print("                case %s: return %s;" % (code, cstr(desc)))

print("""                default: std::terminate();
            }
        }
    }
}""")
