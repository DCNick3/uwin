import re
import sys

filenames = sys.argv[1:]

CPP = re.compile(".*\.cpp")
H = re.compile(".*\.h")

REPLACEMENTS = [
    (H, re.compile(r"#if KAITAI_STRUCT_VERSION < 9000L\n"
                   r"#error \"Incompatible Kaitai Struct C\+\+/STL API: version 0\.9 or later is required\"\n"
                   r"#endif"), 'static_assert(kaitai::version >= 9000L, "Incompatible Kaitai Struct C++/STL API: version 0.9 or later is required");'),
    (re.compile(r"output/microsoft_pe\.cpp"), re.compile(r"_root\(\)->pe\(\)->sections\(\)->at\(\(section_number\(\) - 1\)\)"), "_root()->pe()->sections()->at((section_number() - 1)).get()")
]

for filename in filenames:
    with open(filename, 'r') as f:
        data = f.read()
    with open(filename, 'w') as f:
        for namerex, regex, replacement in REPLACEMENTS:
            if namerex.match(filename):
                if not regex.search(data):
                    raise RuntimeError(str(regex) + " was not matched on file " + filename)

                data = regex.sub(replacement, data)
        f.write(data)
