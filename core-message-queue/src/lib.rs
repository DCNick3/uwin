use core_mem::ptr::PtrRepr;

pub struct MouseMessage {
    pub keys: (), /*TODO*/
    pub point: (i16, i16),
}

pub enum MessagePayload {
    // Create {} // This passes a pointer to CREATESTRUCT and only makes sense when passed directly, w/o putting it in the queue
    MouseMove(MouseMessage),
    LButtonDown(MouseMessage),
    RButtonDown(MouseMessage),
    LButtonUp(MouseMessage),
    RButtonUp(MouseMessage),

    Quit { status: u32 },
}

pub struct Message {
    pub hwnd: PtrRepr,
    pub payload: MessagePayload,
}

pub type Sender = crossbeam_channel::Sender<Message>;
pub type Receiver = crossbeam_channel::Receiver<Message>;
