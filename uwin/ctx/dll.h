#pragma one

#include <memory>

#include "win32/dll/TEST_iface.h"
#include "ctx/process.h"


namespace uwin {
    namespace ctx {
        class dll {
            std::unique_ptr<win32::dll::TEST_iface> _dll_TEST;
        public:
            explicit inline dll(ctx::process &process_ctx)
                    :
                    _dll_TEST(new win32::dll::TEST_iface(process_ctx))
            {}

            inline win32::dll::TEST_iface& get_TEST() { return *_dll_TEST; }
        };
    }
}