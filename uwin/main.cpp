#include <fmt/core.h>
#include <boost/di.hpp>
#include <boost/di/extension/injections/extensible_injector.hpp>
#include <boost/di/extension/scopes/shared.hpp>

#include "xcute/except.h"
#include "mem/mgr/target_mem_mgr.h"
#include "util/except.h"
#include "util/file.h"
#include "win32/error_code.h"
#include "xcute/remill/remill_rt.h"
#include "ctx/process.h"
#include "ctx/dll.h"
#include "ctx/thread.h"
#include "win32/ldr/module_loader.h"
#include "win32/svc/basic_mbox.h"
#include "win32/svc/dummy_locale.h"
#include "win32/dll/stub_exception.h"
#include "log/log.h"
#include "str/uconv.h"

#include "win32/dll/KERNEL32/impl.h"
#include "win32/dll/USER32/impl.h"
#include "win32/dll/stub_exception.h"

#include <map>

using namespace uwin;
namespace di = boost::di;
namespace detail {
    class my_scope {
    public:
        template<class, class T>
        class scope {
        public:
            scope() noexcept = default;

            template<class T_, class>
            using is_referable = typename di::wrappers::shared<my_scope, T>::template is_referable<T_>;

            template<class, class, class TProvider>
            static auto try_create(const TProvider &provider)
            -> decltype(di::wrappers::shared<my_scope, T>{std::shared_ptr<T>{provider.get()}});
            /**
             * Deduce scope version
             */
            template<class, class, class TProvider>
            di::wrappers::shared <my_scope, T> create(const TProvider &provider) &&{
                auto &object = provider.cfg().template data<T>();
                if (!object) {
                    object = std::shared_ptr<T>{provider.get()};
                }
                return di::wrappers::shared<my_scope, T> {std::static_pointer_cast<T>(object)};
            }
        };
    };
}  // namespace detail

static constexpr detail::my_scope my_scope{};

class my_config : public di::config {
public:
    template <class T>
    struct type {
        static void id() {}
    };
    template <class T>
    static auto type_id() {
        return reinterpret_cast<std::size_t>(&type<T>::id);
    }

    template <class T>
    struct scope_traits {
        //using type = typename di::config::scope_traits<T>::type;
        //static_assert(!std::is_pointer_v<T>, "NO");
        // if we forbid creating objects by default - it doesn't compile on gcc, but compiles with clang (TODO: look into this)
        using type = decltype(my_scope);
    };

    template <class T>
    struct scope_traits<T&> {
        using type = decltype(my_scope);
    };

    template <class T>
    struct scope_traits<T&&> {
        using type = decltype(my_scope);
    };

    template <class T>
    std::shared_ptr<void>* try_data() {
        if (upstream_ != nullptr) {
            auto *up = upstream_->template try_data<T>();
            if (up != nullptr)
                return up;
        }
        std::unique_lock lock(mutex_);
        auto it = data_.find(type_id<T>());
        if (it == data_.end())
            return nullptr;
        return &it->second;
    }

    template <class T>
    auto& data() {
        auto* data = try_data<T>();
        if (data != nullptr)
            return *data;
        std::unique_lock lock(mutex_);
        return data_[type_id<T>()];
    }

    void set_upstream(my_config& upstream) {
        upstream_ = &upstream;
    }

    my_config() = default;
    ~my_config() = default;

private:
    std::mutex mutex_;
    my_config* upstream_{};
    std::map<std::size_t, std::shared_ptr<void>> data_{};
};

int main(int argc, char** argv) {

#ifdef NDEBUG
    try
#endif
    {
        log::info("Starting...");


        auto t = mem::taddr(100);
        auto tc = mem::tcaddr(100);

        auto mapper = mem::create_host_mem_mapper();

        ctx::env_param env_param{"C:\\EXECUTABLE.EXE", {}};

        // Here a lot of objects are created implicitly as singletons.
        // This really reduces amount of boilerplate code. All hail dependency injection
        auto inj = di::make_injector<my_config>(
                // bind some core objects
                di::bind<mem::base_mem_mapper>.to(mapper),
                di::bind<ctx::env_param>.to(env_param),

                di::bind<ctx::process_heap>,
                di::bind<ctx::process>,

                // bind dll implementations
                di::bind<win32::dll::KERNEL32_iface>.to<win32::dll::KERNEL32_impl>(),
                di::bind<win32::dll::USER32_iface>.to<win32::dll::USER32_impl>(),

                // bind win32 svc implementations
                di::bind<win32::svc::mbox>.to<win32::svc::basic_mbox>(),
                di::bind<win32::svc::locale>.to<win32::svc::dummy_locale>()
        );

        auto &process_ctx = inj.create<ctx::process &>();

        {
            // TODO: how do we split thread-owned and process-owned objects?
            auto t_inj = di::make_injector<my_config>(
                    di::extension::make_extensible(inj)
            );
            t_inj.cfg().set_upstream(inj.cfg());

            auto t_inj2 = di::make_injector<my_config>(
                    di::extension::make_extensible(inj)
            );
            t_inj2.cfg().set_upstream(inj.cfg());

            auto &thread_ctx = t_inj.create<ctx::thread &>();
            auto &thread_ctx2 = t_inj2.create<ctx::thread &>();

            //assert(&thread_ctx != &thread_ctx2);
            //assert(&thread_ctx._process == &thread_ctx2._process);

            auto &module_table = inj.create<win32::ldr::module_table &>();

            // Who needs proper dependency managements anyways
            // (Jokes aside, using Dependency Injection would be cool, but, IMO, quite difficult to implement,
            // considering all weird places where you need those dependencies)
            // UPD: Now I use it... Oh well
//            process_ctx._mem_mgr = std::make_unique<mem::mgr::target_mem_mgr>(mapper);
//            process_ctx._dll = std::make_unique<ctx::dll>(process_ctx);
//            process_ctx._ldr = std::make_unique<ctx::ldr>(*process_ctx._dll);
//            process_ctx._handlelike_allocator = std::make_unique<ht::handlelike_allocator>();
//            process_ctx._handle_table = std::make_unique<ht::handletable>(*process_ctx._handlelike_allocator);
//            process_ctx._process_heap = ht::handle_holder(*process_ctx._handle_table,
//            process_ctx._handle_table->emplace<heap::heap>(*process_ctx._mem_mgr, 0, 0));
//            process_ctx._command_line = process_ctx.alloc_str("C:\\EXECUTABLE.EXE");
//            process_ctx._current_thread = std::make_unique<ctx::thread>();
//            process_ctx._environment = std::vector<std::uint8_t>({0, 0});

            assert(argc >= 2);
            const char *exe_path = argv[1];
            auto exe_data = util::read_file(exe_path);

            auto &module = win32::ldr::module_loader::load(module_table,
                                                           process_ctx._mem_mgr, mem::taddr(0x00400000), "MAIN",
                                                           {exe_data.data(), exe_data.size()});


            mem::mgr::target_mem_mgr &mgr = process_ctx._mem_mgr;

            auto base_addr = mgr.get_region_base();

            uwin::xcute::remill::StateEx state{};

            state.process_ctx = &process_ctx;
            state.thread_ctx = &thread_ctx;

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
            } catch (xcute::process_exit &e) {
                log::info("Target process terminated with exit code {}", e._exit_code);
            }
        }

    }
#ifdef NDEBUG
    catch (const std::exception &exc) {
        log::error("{} caught:\n    {}", util::get_nice_current_exception_type_name(), exc.what());
    } catch (...) {
        log::error("Non-std::exception ({}) caught!", util::get_nice_current_exception_type_name());
    }
#endif

    return 0;
}
