#pragma once

#include <memory>
#include <cctype>

<%!
import json

def cstr(s):
    return json.dumps(s)
%>

% for dll_name in dll_names:
#include "win32/dll/${dll_name}_iface.h"
#include "win32/dll/${dll_name}/impl.h"
% endfor
#include "ctx/process.h"


namespace uwin::ctx {
    class dll {
        % for dll_name in dll_names:
            std::unique_ptr<win32::dll::${dll_name}_iface> _dll_${dll_name};
        % endfor
    public:
        explicit inline dll(ctx::process &process_ctx)
            :
        % for i, dll_name in enumerate(dll_names):
            ${',' if i != 0 else ''}_dll_${dll_name}(new win32::dll::${dll_name}_impl(process_ctx))
        % endfor
        {}

        % for dll_name in dll_names:
        inline win32::dll::${dll_name}_iface& get_${dll_name}() { return *_dll_${dll_name}; }
        % endfor

        inline win32::dll::base* try_resolve(const std::string& name) {
            if (false) {}
            % for dll_name in dll_names:
            else if (name == ${cstr(dll_name)})
                return &get_${dll_name}();
            % endfor
            else
                return nullptr;
        }
    };
}