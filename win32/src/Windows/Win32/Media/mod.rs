#![doc = r" do not edit! File auto-generated with win32-bindgen"]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    unused_assignments,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub mod Audio;
pub mod Multimedia;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTASK(pub PtrDiffRepr);
impl HTASK {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HTASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTASK {}
impl ::core::hash::Hash for HTASK {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HTASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTASK").field(&self.0).finish()
    }
}
impl FromIntoMemory for HTASK {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub struct IReferenceClock(pub crate::core::IUnknown);
pub trait IReferenceClock_Trait: crate::core::IUnknown_Trait {
    fn GetTime(&self, p_time: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetTime")
    }
    fn AdviseTime(
        &self,
        base_time: i64,
        stream_time: i64,
        h_event: super::Foundation::HANDLE,
        pdw_advise_cookie: MutPtr<PtrRepr>,
    ) -> crate::core::HRESULT {
        todo!("AdviseTime")
    }
    fn AdvisePeriodic(
        &self,
        start_time: i64,
        period_time: i64,
        h_semaphore: super::Foundation::HANDLE,
        pdw_advise_cookie: MutPtr<PtrRepr>,
    ) -> crate::core::HRESULT {
        todo!("AdvisePeriodic")
    }
    fn Unadvise(&self, dw_advise_cookie: PtrRepr) -> crate::core::HRESULT {
        todo!("Unadvise")
    }
}
impl ::core::clone::Clone for IReferenceClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IReferenceClock {}
impl ::core::cmp::PartialEq for IReferenceClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock {}
impl ::core::fmt::Debug for IReferenceClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock").field(&self.0).finish()
    }
}
impl FromIntoMemory for IReferenceClock {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IReferenceClock {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x56a86897_0ad4_11ce_b03a_0020af0ba770);
}
pub struct IReferenceClock2(pub crate::core::IUnknown);
pub trait IReferenceClock2_Trait: IReferenceClock_Trait {}
impl ::core::clone::Clone for IReferenceClock2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IReferenceClock2 {}
impl ::core::cmp::PartialEq for IReferenceClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock2 {}
impl ::core::fmt::Debug for IReferenceClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock2").field(&self.0).finish()
    }
}
impl FromIntoMemory for IReferenceClock2 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IReferenceClock2 {
    type Super = IReferenceClock;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x36b73885_c2c8_11cf_8b46_00805f6cef60);
}
pub struct IReferenceClockTimerControl(pub crate::core::IUnknown);
pub trait IReferenceClockTimerControl_Trait: crate::core::IUnknown_Trait {
    fn SetDefaultTimerResolution(&self, timer_resolution: i64) -> crate::core::HRESULT {
        todo!("SetDefaultTimerResolution")
    }
    fn GetDefaultTimerResolution(&self, p_timer_resolution: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetDefaultTimerResolution")
    }
}
impl ::core::clone::Clone for IReferenceClockTimerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IReferenceClockTimerControl {}
impl ::core::cmp::PartialEq for IReferenceClockTimerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClockTimerControl {}
impl ::core::fmt::Debug for IReferenceClockTimerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClockTimerControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IReferenceClockTimerControl {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IReferenceClockTimerControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xebec459c_2eca_4d42_a8af_30df557614b8);
}
pub const JOYERR_BASE: u32 = 160u32;
pub type LPDRVCALLBACK = StdCallFnPtr<(Multimedia::HDRVR, u32, PtrRepr, PtrRepr, PtrRepr), ()>;
pub type LPTIMECALLBACK = StdCallFnPtr<(u32, u32, PtrRepr, PtrRepr, PtrRepr), ()>;
pub const MAXERRORLENGTH: u32 = 256u32;
pub const MAXPNAMELEN: u32 = 32u32;
pub const MCIERR_BASE: u32 = 256u32;
pub const MCI_CD_OFFSET: u32 = 1088u32;
pub const MCI_SEQ_OFFSET: u32 = 1216u32;
pub const MCI_STRING_OFFSET: u32 = 512u32;
pub const MCI_VD_OFFSET: u32 = 1024u32;
pub const MCI_WAVE_OFFSET: u32 = 1152u32;
pub const MIDIERR_BASE: u32 = 64u32;
pub const MIXERR_BASE: u32 = 1024u32;
pub const MMSYSERR_ALLOCATED: u32 = 4u32;
pub const MMSYSERR_BADDB: u32 = 14u32;
pub const MMSYSERR_BADDEVICEID: u32 = 2u32;
pub const MMSYSERR_BADERRNUM: u32 = 9u32;
pub const MMSYSERR_BASE: u32 = 0u32;
pub const MMSYSERR_DELETEERROR: u32 = 18u32;
pub const MMSYSERR_ERROR: u32 = 1u32;
pub const MMSYSERR_HANDLEBUSY: u32 = 12u32;
pub const MMSYSERR_INVALFLAG: u32 = 10u32;
pub const MMSYSERR_INVALHANDLE: u32 = 5u32;
pub const MMSYSERR_INVALIDALIAS: u32 = 13u32;
pub const MMSYSERR_INVALPARAM: u32 = 11u32;
pub const MMSYSERR_KEYNOTFOUND: u32 = 15u32;
pub const MMSYSERR_LASTERROR: u32 = 21u32;
pub const MMSYSERR_MOREDATA: u32 = 21u32;
pub const MMSYSERR_NODRIVER: u32 = 6u32;
pub const MMSYSERR_NODRIVERCB: u32 = 20u32;
pub const MMSYSERR_NOERROR: u32 = 0u32;
pub const MMSYSERR_NOMEM: u32 = 7u32;
pub const MMSYSERR_NOTENABLED: u32 = 3u32;
pub const MMSYSERR_NOTSUPPORTED: u32 = 8u32;
pub const MMSYSERR_READERROR: u32 = 16u32;
pub const MMSYSERR_VALNOTFOUND: u32 = 19u32;
pub const MMSYSERR_WRITEERROR: u32 = 17u32;
pub struct MMTIME {
    pub wType: u32,
    pub u: MMTIME_0,
}
impl ::core::marker::Copy for MMTIME {}
impl ::core::clone::Clone for MMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMTIME")
            .field("wType", &self.wType)
            .field("u", &self.u)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wType == other.wType && self.u == other.u
    }
}
impl ::core::cmp::Eq for MMTIME {}
impl FromIntoMemory for MMTIME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_wType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_u = <MMTIME_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            wType: f_wType,
            u: f_u,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.wType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.u, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MMTIME_0 {
    data: [u8; 4],
}
impl ::core::default::Default for MMTIME_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for MMTIME_0 {}
impl ::core::clone::Clone for MMTIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMTIME_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMTIME_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMTIME_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MMTIME_0 {}
impl FromIntoMemory for MMTIME_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct MMTIME_0_0 {
    pub songptrpos: u32,
}
impl ::core::marker::Copy for MMTIME_0_0 {}
impl ::core::clone::Clone for MMTIME_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMTIME_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMTIME_0_0")
            .field("songptrpos", &self.songptrpos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMTIME_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.songptrpos == other.songptrpos
    }
}
impl ::core::cmp::Eq for MMTIME_0_0 {}
impl FromIntoMemory for MMTIME_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_songptrpos = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            songptrpos: f_songptrpos,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.songptrpos, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct MMTIME_0_1 {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub frame: u8,
    pub fps: u8,
    pub dummy: u8,
    pub pad: [u8; 2],
}
impl ::core::marker::Copy for MMTIME_0_1 {}
impl ::core::clone::Clone for MMTIME_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMTIME_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMTIME_0_1")
            .field("hour", &self.hour)
            .field("min", &self.min)
            .field("sec", &self.sec)
            .field("frame", &self.frame)
            .field("fps", &self.fps)
            .field("dummy", &self.dummy)
            .field("pad", &self.pad)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMTIME_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour
            && self.min == other.min
            && self.sec == other.sec
            && self.frame == other.frame
            && self.fps == other.fps
            && self.dummy == other.dummy
            && self.pad == other.pad
    }
}
impl ::core::cmp::Eq for MMTIME_0_1 {}
impl FromIntoMemory for MMTIME_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_hour = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_min = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_sec = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_frame = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_fps = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_dummy = <u8 as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        let f_pad = <[u8; 2] as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            hour: f_hour,
            min: f_min,
            sec: f_sec,
            frame: f_frame,
            fps: f_fps,
            dummy: f_dummy,
            pad: f_pad,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.hour, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.min, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.sec, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.frame, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.fps, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.dummy, &mut into[5..5 + 1]);
        FromIntoMemory::into_bytes(self.pad, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const MM_ADLIB: u32 = 9u32;
pub const MM_DRVM_CLOSE: u32 = 977u32;
pub const MM_DRVM_DATA: u32 = 978u32;
pub const MM_DRVM_ERROR: u32 = 979u32;
pub const MM_DRVM_OPEN: u32 = 976u32;
pub const MM_JOY1BUTTONDOWN: u32 = 949u32;
pub const MM_JOY1BUTTONUP: u32 = 951u32;
pub const MM_JOY1MOVE: u32 = 928u32;
pub const MM_JOY1ZMOVE: u32 = 930u32;
pub const MM_JOY2BUTTONDOWN: u32 = 950u32;
pub const MM_JOY2BUTTONUP: u32 = 952u32;
pub const MM_JOY2MOVE: u32 = 929u32;
pub const MM_JOY2ZMOVE: u32 = 931u32;
pub const MM_MCINOTIFY: u32 = 953u32;
pub const MM_MCISIGNAL: u32 = 971u32;
pub const MM_MICROSOFT: u32 = 1u32;
pub const MM_MIDI_MAPPER: u32 = 1u32;
pub const MM_MIM_CLOSE: u32 = 962u32;
pub const MM_MIM_DATA: u32 = 963u32;
pub const MM_MIM_ERROR: u32 = 965u32;
pub const MM_MIM_LONGDATA: u32 = 964u32;
pub const MM_MIM_LONGERROR: u32 = 966u32;
pub const MM_MIM_MOREDATA: u32 = 972u32;
pub const MM_MIM_OPEN: u32 = 961u32;
pub const MM_MIXM_CONTROL_CHANGE: u32 = 977u32;
pub const MM_MIXM_LINE_CHANGE: u32 = 976u32;
pub const MM_MOM_CLOSE: u32 = 968u32;
pub const MM_MOM_DONE: u32 = 969u32;
pub const MM_MOM_OPEN: u32 = 967u32;
pub const MM_MOM_POSITIONCB: u32 = 970u32;
pub const MM_MPU401_MIDIIN: u32 = 11u32;
pub const MM_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_PC_JOYSTICK: u32 = 12u32;
pub const MM_SNDBLST_MIDIIN: u32 = 4u32;
pub const MM_SNDBLST_MIDIOUT: u32 = 3u32;
pub const MM_SNDBLST_SYNTH: u32 = 5u32;
pub const MM_SNDBLST_WAVEIN: u32 = 7u32;
pub const MM_SNDBLST_WAVEOUT: u32 = 6u32;
pub const MM_STREAM_CLOSE: u32 = 981u32;
pub const MM_STREAM_DONE: u32 = 982u32;
pub const MM_STREAM_ERROR: u32 = 983u32;
pub const MM_STREAM_OPEN: u32 = 980u32;
pub const MM_WAVE_MAPPER: u32 = 2u32;
pub const MM_WIM_CLOSE: u32 = 959u32;
pub const MM_WIM_DATA: u32 = 960u32;
pub const MM_WIM_OPEN: u32 = 958u32;
pub const MM_WOM_CLOSE: u32 = 956u32;
pub const MM_WOM_DONE: u32 = 957u32;
pub const MM_WOM_OPEN: u32 = 955u32;
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
impl ::core::marker::Copy for TIMECAPS {}
impl ::core::clone::Clone for TIMECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECAPS")
            .field("wPeriodMin", &self.wPeriodMin)
            .field("wPeriodMax", &self.wPeriodMax)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wPeriodMin == other.wPeriodMin && self.wPeriodMax == other.wPeriodMax
    }
}
impl ::core::cmp::Eq for TIMECAPS {}
impl FromIntoMemory for TIMECAPS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_wPeriodMin = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wPeriodMax = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            wPeriodMin: f_wPeriodMin,
            wPeriodMax: f_wPeriodMax,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.wPeriodMin, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMax, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TIMECODE {
    data: [u8; 8],
}
impl ::core::default::Default for TIMECODE {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for TIMECODE {}
impl ::core::clone::Clone for TIMECODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECODE")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMECODE {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for TIMECODE {}
impl FromIntoMemory for TIMECODE {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 8];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        8
    }
}
pub struct TIMECODE_0 {
    pub wFrameRate: u16,
    pub wFrameFract: u16,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for TIMECODE_0 {}
impl ::core::clone::Clone for TIMECODE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECODE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECODE_0")
            .field("wFrameRate", &self.wFrameRate)
            .field("wFrameFract", &self.wFrameFract)
            .field("dwFrames", &self.dwFrames)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMECODE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wFrameRate == other.wFrameRate
            && self.wFrameFract == other.wFrameFract
            && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for TIMECODE_0 {}
impl FromIntoMemory for TIMECODE_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_wFrameRate = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wFrameFract = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_dwFrames = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            wFrameRate: f_wFrameRate,
            wFrameFract: f_wFrameFract,
            dwFrames: f_dwFrames,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.wFrameRate, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wFrameFract, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.dwFrames, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TIMECODE_SAMPLE {
    pub qwTick: i64,
    pub timecode: TIMECODE,
    pub dwUser: u32,
    pub dwFlags: TIMECODE_SAMPLE_FLAGS,
}
impl ::core::marker::Copy for TIMECODE_SAMPLE {}
impl ::core::clone::Clone for TIMECODE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECODE_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECODE_SAMPLE")
            .field("qwTick", &self.qwTick)
            .field("timecode", &self.timecode)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMECODE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        self.qwTick == other.qwTick
            && self.timecode == other.timecode
            && self.dwUser == other.dwUser
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TIMECODE_SAMPLE {}
impl FromIntoMemory for TIMECODE_SAMPLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_qwTick = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_timecode = <TIMECODE as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_dwUser = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwFlags = <TIMECODE_SAMPLE_FLAGS as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            qwTick: f_qwTick,
            timecode: f_timecode,
            dwUser: f_dwUser,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.qwTick, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.timecode, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TIMECODE_SAMPLE_FLAGS(pub u32);
pub const ED_DEVCAP_TIMECODE_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(4121u32);
pub const ED_DEVCAP_ATN_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5047u32);
pub const ED_DEVCAP_RTC_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5050u32);
impl ::core::marker::Copy for TIMECODE_SAMPLE_FLAGS {}
impl ::core::clone::Clone for TIMECODE_SAMPLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TIMECODE_SAMPLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIMECODE_SAMPLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMECODE_SAMPLE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TIMECODE_SAMPLE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const TIMERR_BASE: u32 = 96u32;
pub const TIMERR_NOCANDO: u32 = 97u32;
pub const TIMERR_NOERROR: u32 = 0u32;
pub const TIMERR_STRUCT: u32 = 129u32;
pub const TIME_BYTES: u32 = 4u32;
pub const TIME_CALLBACK_EVENT_PULSE: u32 = 32u32;
pub const TIME_CALLBACK_EVENT_SET: u32 = 16u32;
pub const TIME_CALLBACK_FUNCTION: u32 = 0u32;
pub const TIME_KILL_SYNCHRONOUS: u32 = 256u32;
pub const TIME_MIDI: u32 = 16u32;
pub const TIME_MS: u32 = 1u32;
pub const TIME_ONESHOT: u32 = 0u32;
pub const TIME_PERIODIC: u32 = 1u32;
pub const TIME_SAMPLES: u32 = 2u32;
pub const TIME_SMPTE: u32 = 8u32;
pub const TIME_TICKS: u32 = 32u32;
pub const WAVERR_BASE: u32 = 32u32;
pub trait Api {
    fn timeBeginPeriod(&self, u_period: u32) -> u32 {
        todo!("timeBeginPeriod")
    }
    fn timeEndPeriod(&self, u_period: u32) -> u32 {
        todo!("timeEndPeriod")
    }
    fn timeGetDevCaps(&self, ptc: MutPtr<TIMECAPS>, cbtc: u32) -> u32 {
        todo!("timeGetDevCaps")
    }
    fn timeGetSystemTime(&self, pmmt: MutPtr<MMTIME>, cbmmt: u32) -> u32 {
        todo!("timeGetSystemTime")
    }
    fn timeGetTime(&self) -> u32 {
        todo!("timeGetTime")
    }
    fn timeKillEvent(&self, u_timer_id: u32) -> u32 {
        todo!("timeKillEvent")
    }
    fn timeSetEvent(
        &self,
        u_delay: u32,
        u_resolution: u32,
        fptc: LPTIMECALLBACK,
        dw_user: PtrRepr,
        fu_event: u32,
    ) -> u32 {
        todo!("timeSetEvent")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
