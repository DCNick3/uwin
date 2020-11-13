#pragma one

#include <memory>

% for dll_name in dll_names:
#include "win32/dll/${dll_name}_iface.h"
% endfor
#include "ctx/process.h"


namespace uwin {
    namespace ctx {
        class dll {
            % for dll_name in dll_names:
                std::unique_ptr<win32::dll::${dll_name}_iface> _dll_${dll_name};
            % endfor
        public:
            explicit inline dll(ctx::process &process_ctx)
                :
            % for i, dll_name in enumerate(dll_names):
                ${',' if i != 0 else ''}_dll_${dll_name}(new win32::dll::TEST_iface(process_ctx))
            % endfor
            {}

            % for dll_name in dll_names:
            inline win32::dll::${dll_name}_iface& get_${dll_name}() { return *_dll_${dll_name}; }
            % endfor
        };
    }
}