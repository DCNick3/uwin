<%
import json

from winerror import errors

def cstr(s):
    return json.dumps(s) # well, all JSON strings are valid C string literals
%>
#include "win32/error_code.h"

#include <exception>

namespace uwin::win32 {
    const char* error_code_to_slug(error_code code) {
        switch (static_cast<long>(code)) {
        % for code, slug, desc in errors:
            case ${code}: return ${cstr(slug)};
        % endfor
            default: std::terminate();
        }
    }

    const char* error_code_to_desc(error_code code) {
        switch (static_cast<long>(code)) {
        % for code, slug, desc in errors:
            case ${code}: return ${cstr(desc)};
        % endfor
            default: std::terminate();
        }
    }
}
