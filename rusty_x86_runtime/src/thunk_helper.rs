use crate::ExtendedContext;
use core_abi::stdcall::StdCalleeHelper;
use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::PtrRepr;
use rusty_x86::types::CpuContext;
use tracing::subscriber::Interest;
use tracing::{Callsite, Metadata};

pub(crate) struct MyCallsite {
    metadata: Metadata<'static>,
}

impl MyCallsite {
    pub const fn new(identifier: tracing::callsite::Identifier) -> Self {
        Self {
            metadata: Metadata::new(
                "",
                "thunk",
                tracing::Level::TRACE,
                None,
                None,
                None,
                tracing::field::FieldSet::new(&["message"], identifier),
                tracing::metadata::Kind::EVENT,
            ),
        }
    }

    pub const fn new_span(
        identifier: tracing::callsite::Identifier,
        span_name: &'static str,
    ) -> Self {
        Self {
            metadata: Metadata::new(
                span_name,
                "thunk",
                tracing::Level::TRACE,
                None,
                None,
                None,
                tracing::field::FieldSet::new(&[], identifier),
                tracing::metadata::Kind::EVENT,
            ),
        }
    }
}

impl Callsite for MyCallsite {
    fn set_interest(&self, _interest: Interest) {
        // поебать
    }

    fn metadata(&self) -> &Metadata<'_> {
        &self.metadata
    }
}

pub(crate) fn thunk_helper(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    span_callsite: &'static MyCallsite,
    body: fn(
        StdCalleeHelper<ExtendedContext, CpuContext, FlatMemoryCtx>,
        bool, // trace_enabled
        &'static MyCallsite,
    ) -> PtrRepr,
) -> PtrRepr {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call = StdCalleeHelper::new(context, memory);

        let span = tracing::Span::new(
            span_callsite.metadata(),
            &span_callsite.metadata().fields().value_set(&[]),
        );

        // let span = tracing::trace_span!("name"); // TODO
        let _enter = span.enter();
        // tracing::trace!("ret_addr = {:#010x}", call.return_address());

        let trace_event_enabled = tracing::event_enabled!(tracing::Level::TRACE);

        static CALLSITE: MyCallsite = MyCallsite::new(tracing::callsite::Identifier(&CALLSITE));

        let fields = CALLSITE.metadata().fields();

        if trace_event_enabled {
            tracing::event::Event::dispatch(
                CALLSITE.metadata(),
                &fields.value_set(&[(
                    &unsafe { fields.iter().next().unwrap_unchecked() },
                    Some(&format_args!("ret_addr = {:#010x}", call.return_address())
                        as &dyn tracing::Value),
                )]),
            );
        }

        body(call, trace_event_enabled, &CALLSITE)
    }));
    match result {
        Ok(ret) => ret,
        Err(_) => {
            eprintln!("Caught a panic in native code. Whoops, aborting..");
            std::process::abort();
        }
    }
}
