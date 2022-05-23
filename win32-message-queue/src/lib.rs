use core_message_queue::{Message, MessagePayload, Receiver, Sender};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;
use win32::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use win32::Win32::UI::WindowsAndMessaging::{
    MSG, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP,
};
use win32_windows::WindowsRegistry;

fn convert_message(registry: &Mutex<WindowsRegistry>, message: Message) -> MSG {
    use MessagePayload::*;
    let (message_type, w_param, l_param) = match message.payload {
        Quit { status } => (WM_QUIT, status as _, 0),
        MouseMove(m) => (WM_MOUSEMOVE, m.w_param(), m.l_param()),
        LButtonDown(m) => (WM_LBUTTONDOWN, m.w_param(), m.l_param()),
        LButtonUp(m) => (WM_LBUTTONUP, m.w_param(), m.l_param()),
        RButtonDown(m) => (WM_RBUTTONDOWN, m.w_param(), m.l_param()),
        RButtonUp(m) => (WM_RBUTTONUP, m.w_param(), m.l_param()),
        _ => todo!(),
    };

    let registry = registry.lock().unwrap();

    MSG {
        hwnd: message
            .window_id
            .map(|window_id| registry.window_id_to_hwnd(window_id).unwrap())
            .unwrap_or(HWND(0) /* TODO: Is this right? */),
        message: message_type,
        wParam: WPARAM(w_param),
        lParam: LPARAM(l_param),
        time: 0,
        pt: POINT { x: 0, y: 0 },
    }
}

pub struct MessageQueue {
    receiver: Receiver,
    sender: Sender,
    window_registry: Arc<Mutex<WindowsRegistry>>,
}

impl MessageQueue {
    pub fn new(window_registry: Arc<Mutex<WindowsRegistry>>) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self {
            sender,
            receiver,
            window_registry,
        }
    }

    pub fn get_sender(&self) -> Sender {
        self.sender.clone()
    }

    pub fn get_message(&self) -> MSG {
        let message = self.receiver.recv().unwrap();
        convert_message(&self.window_registry, message)
    }
}

pub struct MessageQueueRegistry {
    queues: HashMap<ThreadId, MessageQueue>,
    windows_registry: Arc<Mutex<WindowsRegistry>>,
}

impl MessageQueueRegistry {
    pub fn new(windows_registry: Arc<Mutex<WindowsRegistry>>) -> Self {
        Self {
            queues: HashMap::new(),
            windows_registry,
        }
    }

    pub fn insert(&mut self, thread_id: ThreadId) -> &MessageQueue {
        let mq = MessageQueue::new(self.windows_registry.clone());

        let entry = self.queues.entry(thread_id);

        assert!(matches!(entry, Entry::Vacant(_)));

        let sender = entry.or_insert(mq);

        sender
    }

    pub fn remove(&mut self, thread_id: ThreadId) -> bool {
        self.queues.remove(&thread_id).is_some()
    }

    pub fn get_queue(&self, thread_id: ThreadId) -> Option<&MessageQueue> {
        self.queues.get(&thread_id)
    }
}
