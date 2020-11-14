#include <fmt/core.h>

#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "win32/error_code.h"
#include "xcute/remill/remill_rt.h"
#include "ctx/process.h"
#include "ctx/dll.h"


using namespace uwin;


int main() {
    try {
        auto mapper = mem::create_host_mem_mapper();

        ctx::process process_ctx;

        process_ctx._mem_mgr = std::make_unique<mem::mgr::target_mem_mgr>(mapper);
        process_ctx._dll = std::make_unique<ctx::dll>(process_ctx);

        mem::mgr::target_mem_mgr &mgr = *process_ctx._mem_mgr;

        auto base_addr = mgr.get_region_base();

        uwin::xcute::remill::StateEx state{};

        state.process_ctx = &process_ctx;

        // reserve the executable image region
        auto image_region = mgr.reserve_fixed(mem::tmem_region(0x00400000, 0x41ff));
        mgr.commit(mem::tmem_region(0x404000, 0x1000), mem::mgr::tprot::rw);

        // MessageBoxA reference
        mgr.deref(mem::tptr<std::uint32_t>(0x404030)) = 0x80000000;

        auto stack_region = mgr.reserve_dynamic(0x10000);

        mgr.commit(stack_region, mem::mgr::tprot::rw);

        mgr.deref((stack_region.end() - 20).as<std::uint32_t>()) = 0x81337227;

        state.base.gpr.rsp.dword = (stack_region.end() - 20).value();


        xcute::remill::uwin_remill_dispatch(&state.base, 0x401000, (uwin::xcute::remill::Memory *) base_addr);

    } catch (const std::exception &exc) {
        fmt::print("{} caught:\n    {}", util::get_nice_current_exception_type_name(), exc.what());
    }

    return 0;
}
