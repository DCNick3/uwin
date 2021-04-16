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
% endfor
#include "win32/ldr/module_table.h"
#include "ctx/process.h"


namespace uwin::ctx {
    class dll {
        % for dll_name in dll_names:
        win32::dll::${dll_name}_iface& _dll_${dll_name};
        % endfor
    public:
        explicit inline dll(
            win32::ldr::module_table& module_table,
        % for i, dll_name in enumerate(dll_names):
            ${',' if i != 0 else ''}win32::dll::${dll_name}_iface& dll_${dll_name}
        % endfor
        )
            :
        % for i, dll_name in enumerate(dll_names):
            ${',' if i != 0 else ''}_dll_${dll_name}(dll_${dll_name})
        % endfor
        {
                % for i, dll_name in enumerate(dll_names):
                    module_table.register_native_module(dll_${dll_name});
                % endfor
        }

        % for dll_name in dll_names:
        inline win32::dll::${dll_name}_iface& get_${dll_name}() { return _dll_${dll_name}; }
        % endfor

        inline win32::dll::native_module* try_resolve(const std::string& name) {
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