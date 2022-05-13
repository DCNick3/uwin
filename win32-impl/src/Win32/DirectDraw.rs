use crate::ProcessContext;
use core_mem::ptr::MutPtr;
use win32::core::{IUnknown, GUID, HRESULT};
use win32::Win32::Graphics::DirectDraw::IDirectDraw;

pub struct DirectDraw {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::Graphics::DirectDraw::Api for DirectDraw {
    fn DirectDrawCreate(
        &self,
        lp_guid: MutPtr<GUID>,
        _lplp_dd: MutPtr<IDirectDraw>,
        p_unk_outer: IUnknown,
    ) -> HRESULT {
        let _ctx = self.process_ctx.memory_ctx;

        assert_eq!(lp_guid, MutPtr::null());
        assert_eq!(p_unk_outer.raw_ptr(), 0);

        todo!()
    }
}
