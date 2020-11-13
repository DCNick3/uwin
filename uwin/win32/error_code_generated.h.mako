<%
from winerror import errors
%>
#pragma once

#include <cstdint>

namespace uwin {
    namespace win32 {
        enum class error_code : std::uint32_t {
            % for code, slug, desc in errors:
                ${slug} = ${code},
            % endfor
        };
    }
}
