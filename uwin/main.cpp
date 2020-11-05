#include <fmt/core.h>

#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "win32/error_code.h"
#include "xcute/remill/remill_rt.h"


using namespace uwin;


int main() {
    try {
        auto mapper = mem::create_host_mem_mapper();
        mem::mgr::target_mem_mgr mgr(mapper);

        auto base_addr = mgr.get_region_base();

        uwin::xcute::remill::State state{};

        // reserve the executable image region
        auto image_region = mgr.reserve_fixed(mem::tmem_region(0x00400000, 0x41ff));
        mgr.commit(mem::tmem_region(0x404000, 0x1000), mem::mgr::tprot::rw);

        mgr.deref(mem::tptr<std::uint32_t>(0x404030)) = 0x81337228;

        auto stack_region = mgr.reserve_dynamic(0x10000);

        mgr.deref((stack_region.end() - 20).as<std::uint32_t>()) = 0x81337227;

        mgr.commit(stack_region, mem::mgr::tprot::rw);

        state.gpr.rsp.dword = (stack_region.end() - 20).value();


        xcute::remill::uwin_remill_dispatch(&state, 0x401000, (uwin::xcute::remill::Memory*)base_addr);

    } catch (const std::exception &exc) {
        fmt::print("{} caught:\n    {}", util::get_nice_current_exception_type_name(), exc.what());
    }

    return 0;
}
