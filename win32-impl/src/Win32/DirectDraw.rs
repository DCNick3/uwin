use crate::ProcessContext;
use core_gfx::GfxContext;
use core_heap::{Heap, HeapOptions};
use core_mem::ptr::{MutPtr, PtrRepr};
use std::sync::{Arc, Mutex};
use tracing::trace;
use win32::core::{IUnknown, GUID, HRESULT};
use win32::Win32::Foundation::S_OK;
use win32::Win32::Graphics::DirectDraw::{DirectDraw_Repr, IDirectDraw, IDirectDraw_Trait};
use win32_ddraw::DirectDraw;
use win32_windows::WindowsRegistry;

pub struct DirectDrawApi {
    pub process_ctx: ProcessContext,
    pub direct_draw_vtable: PtrRepr, // Would __really__ want to have some generalized API for handling those
    pub direct_draw_surface_vtable: PtrRepr,
    pub windows_registry: Arc<Mutex<WindowsRegistry>>,
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

        let mut process_heap = self.process_ctx.process_heap.lock().unwrap();

        let dd_heap = Heap::new(
            self.process_ctx.memory_manager.clone(),
            HeapOptions::default(),
        )
        .expect("Creating heap for DirectDraw object");
        let gfx_context = GfxContext::new(Arc::new(Mutex::new(dd_heap)));

        let cls = Arc::new(DirectDraw::new(
            self.process_ctx.memory_ctx,
            self.process_ctx.process_heap.clone(),
            self.windows_registry.clone(),
            gfx_context,
            self.direct_draw_surface_vtable,
        ));
        let cls = cls as Arc<dyn IDirectDraw_Trait>;

        // TODO: this core is really a boilerplate and should not be necessary at all
        // should be handled by codegen at the win32-bindgen
        // not sure what to do with heap allocation though
        let res = process_heap
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

        S_OK
    }
}
