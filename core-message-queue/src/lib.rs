use core_mem::ptr::{PtrDiffRepr, PtrRepr};

pub struct MouseMessage {
    pub keys: (), /*TODO*/
    pub point: (i16, i16),
}

impl MouseMessage {
    pub fn w_param(&self) -> PtrRepr {
        // key modifiers are not implemented yet
        0
    }

    pub fn l_param(&self) -> PtrDiffRepr {
        let (x, y) = self.point;
        let res = (x as u16 as PtrRepr) | ((y as u16 as PtrRepr) << 16);
        res as PtrDiffRepr
    }
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