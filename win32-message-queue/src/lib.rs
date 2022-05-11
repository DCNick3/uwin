use core_message_queue::{Message, MessagePayload, Receiver, Sender};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::thread::ThreadId;
use win32::core::prelude::PtrDiffRepr;
use win32::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use win32::Win32::UI::WindowsAndMessaging::{MSG, WM_QUIT};

fn convert_message(message: Message) -> MSG {
    let (message_type, w_param, l_param) = match message.payload {
        MessagePayload::Quit { status } => (WM_QUIT, status as _, 0),
        _ => todo!(),
    };

    MSG {
        hwnd: HWND(message_type as PtrDiffRepr),
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
}

impl MessageQueue {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self { sender, receiver }
    }

    pub fn get_sender(&self) -> Sender {
        self.sender.clone()
    }

    pub fn get_message(&self) -> MSG {
        let message = self.receiver.recv().unwrap();
        convert_message(message)
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        MessageQueue::new()
    }
}

pub struct MessageQueueRegistry {
    queues: HashMap<ThreadId, MessageQueue>,
}

impl MessageQueueRegistry {
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
        }
    }

    pub fn insert(&mut self, thread_id: ThreadId) -> &MessageQueue {
        let mq = MessageQueue::new();

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

impl Default for MessageQueueRegistry {
    fn default() -> Self {
        MessageQueueRegistry::new()
    }
}
