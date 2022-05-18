use crate::ProcessContext;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::sync::Arc;
use tracing::trace;
use win32::core::{IUnknown, IUnknown_Trait, GUID, HRESULT};
use win32::Win32::Foundation::S_OK;
use win32::Win32::Graphics::DirectDraw::{DirectDraw_Repr, IDirectDraw, IDirectDraw_Trait};

pub struct DirectDrawApi {
    pub process_ctx: ProcessContext,
    pub direct_draw_vtable: PtrRepr, // Would __really__ want to have some generalized API for handling those
}

#[allow(non_snake_case)]
impl win32::Win32::Graphics::DirectDraw::Api for DirectDrawApi {
    fn DirectDrawCreate(
        &self,
        lp_guid: MutPtr<GUID>,
        lplp_dd: MutPtr<IDirectDraw>,
        p_unk_outer: IUnknown,
    ) -> HRESULT {
        let ctx = self.process_ctx.memory_ctx;

        assert_eq!(lp_guid, MutPtr::null());
        assert_eq!(p_unk_outer.raw_ptr(), 0);

        let mut heap = self.process_ctx.process_heap.lock().unwrap();

        let cls = Arc::new(DirectDrawCls {});
        let cls = cls as Arc<dyn IDirectDraw_Trait>;

        let res = heap
            .alloc_typed(
                ctx,
                DirectDraw_Repr {
                    vtable_IDirectDraw: self.direct_draw_vtable,
                    implementation: Arc::into_raw(cls),
                },
            )
            .expect("Allocating memory for the DirectDraw object");

        trace!("Allocated a DirectDraw object at {:#010x}", res.repr());

        // TODO: use some API to cast the class repr to an interface ptr
        // would probably be useful for QueryInterface implementations

        lplp_dd.write_with(ctx, IDirectDraw(IUnknown::from_raw_ptr(res.repr())));

        // todo!("Generate vtable for COM classes");

        S_OK
    }
}

struct DirectDrawCls {}

impl IUnknown_Trait for DirectDrawCls {}

impl IDirectDraw_Trait for DirectDrawCls {}
