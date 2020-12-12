#include <fmt/core.h>
#include <xcute/except.h>

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
    process_ctx._handlelike_allocator = std::make_unique<ht::handlelike_allocator>();
    process_ctx._handle_table = std::make_unique<ht::handletable>(*process_ctx._handlelike_allocator);
    process_ctx._process_heap = ht::handle_holder(*process_ctx._handle_table,
            process_ctx._handle_table->emplace<heap::heap>(*process_ctx._mem_mgr, 0, 0));
    process_ctx._command_line = process_ctx.alloc_str("C:\\EXECUTABLE.EXE");
    process_ctx._current_thread = std::make_unique<ctx::thread>();
    process_ctx._environment = std::vector<std::uint8_t>({0, 0});

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

    auto teb_region = mgr.reserve_dynamic(0x1000);
    mgr.commit(teb_region, mem::mgr::tprot::rw);


    state.addr.fs_base.dword = teb_region.begin().value();

    // those are set as per https://stackoverflow.com/questions/6028849/windows-initial-execution-context/6029691#6029691
    state.gpr.rax.dword = module.entrypoint().value();
    state.gpr.rbx.dword = teb_region.begin().value();

    state.gpr.rsp.dword = stack_region.end().value();
    state.gpr.rip.dword = module.entrypoint().value();

    try {
        xcute::remill::enter_target_code(mgr, state);
    } catch (xcute::process_exit& e) {
        log::info("Target process terminated with exit code {}", e._exit_code);
    }

    /*} catch (const std::exception &exc) {
        fmt::print("{} caught:\n    {}", util::get_nice_current_exception_type_name(), exc.what());
    }*/

    return 0;
}
