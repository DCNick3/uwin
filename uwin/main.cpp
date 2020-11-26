#include <fmt/core.h>

#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "util/file.h"
#include "win32/error_code.h"
#include "xcute/remill/remill_rt.h"
#include "ctx/process.h"
#include "ctx/dll.h"
#include "ctx/ldr.h"
#include "win32/ldr/module_loader.h"
#include "log/log.h"


using namespace uwin;

int main(int argc, char** argv) {

    log::info("Starting...");

    auto mapper = mem::create_host_mem_mapper();

    ctx::process process_ctx;

    // Who needs proper dependency managements anyways
    // (Jokes aside, using Dependency Injection would be cool, but, IMO, quite difficult to implement,
    // considering all weird places where you need those dependencies)
    process_ctx._mem_mgr = std::make_unique<mem::mgr::target_mem_mgr>(mapper);
    process_ctx._dll = std::make_unique<ctx::dll>(process_ctx);
    process_ctx._ldr = std::make_unique<ctx::ldr>(*process_ctx._dll);

    assert(argc >= 2);
    const char *exe_path = argv[1];
    auto exe_data = util::read_file(exe_path);

    auto& module = win32::ldr::module_loader::load(*process_ctx._ldr,
            *process_ctx._mem_mgr, mem::taddr(0x00400000), "MAIN",
            {exe_data.data(), exe_data.size()});


    mem::mgr::target_mem_mgr &mgr = *process_ctx._mem_mgr;

    auto base_addr = mgr.get_region_base();

    uwin::xcute::remill::StateEx state{};

    state.process_ctx = &process_ctx;

    auto stack_region = mgr.reserve_dynamic(0x10000);

    mgr.commit(stack_region, mem::mgr::tprot::rw);

    mgr.deref((stack_region.end() - 20).as<std::uint32_t>()) = 0x81337227;

    state.base.gpr.rsp.dword = (stack_region.end() - 20).value();

    xcute::remill::uwin_remill_dispatch(&state.base, module.entrypoint().value(), (uwin::xcute::remill::Memory *) base_addr);

    /*} catch (const std::exception &exc) {
        fmt::print("{} caught:\n    {}", util::get_nice_current_exception_type_name(), exc.what());
    }*/

    return 0;
}
