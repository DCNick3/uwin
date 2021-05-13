
#include <string>
#include <iostream>

#include "win32/error_code.h"

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " [error_code]\n";
        return 1;
    }

    uwin::win32::error_code code = static_cast<uwin::win32::error_code>(std::stoi(argv[1]));

    auto known = uwin::win32::error_code_known(code);
    if (known) {
        auto desc = uwin::win32::error_code_to_desc(code);
        auto slug = uwin::win32::error_code_to_slug(code);

        std::cout << slug << ": " << desc << std::endl;
    } else {
        std::cerr << "Unknown error code\n";
        return 1;
    }
    return 0;
}
