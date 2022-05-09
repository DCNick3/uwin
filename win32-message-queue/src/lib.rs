use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::ThreadId;
use win32::Win32::UI::WindowsAndMessaging::MSG;

pub struct MessageQueueRegistry {
    senders: HashMap<ThreadId, Sender<MSG>>,
    receivers: HashMap<ThreadId, Receiver<MSG>>,
}

impl MessageQueueRegistry {
    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
            receivers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, thread_id: ThreadId) -> (&Sender<MSG>, &Receiver<MSG>) {
        let (sender, receiver) = channel();

        let sender_entry = self.senders.entry(thread_id);
        let receiver_entry = self.receivers.entry(thread_id);

        assert!(matches!(sender_entry, Entry::Vacant(_)));
        assert!(matches!(receiver_entry, Entry::Vacant(_)));

        let sender = sender_entry.or_insert(sender);
        let receiver = receiver_entry.or_insert(receiver);

        (sender, receiver)
    }

    pub fn remove(&mut self, thread_id: ThreadId) -> bool {
        let rm_sender = self.senders.remove(&thread_id).is_some();
        let rm_receiver = self.receivers.remove(&thread_id).is_some();

        assert_eq!(rm_sender, rm_receiver);

        rm_receiver
    }

    pub fn get_sender(&self, thread_id: ThreadId) -> Option<Sender<MSG>> {
        let sender = self.senders.get(&thread_id)?;
        Some(sender.clone())
    }

    pub fn get_receiver(&self, thread_id: ThreadId) -> Option<&Receiver<MSG>> {
        self.receivers.get(&thread_id)
    }
}

impl Default for MessageQueueRegistry {
    fn default() -> Self {
        MessageQueueRegistry::new()
    }
}
