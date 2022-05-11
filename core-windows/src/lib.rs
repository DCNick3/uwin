use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use tracing::trace;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

// TODO: add conditions & imports for other platforms
use winit::platform::unix::EventLoopExtUnix;

use core_mem::ptr::PtrRepr;
use core_message_queue::{Message, MessagePayload, MouseMessage, Sender};
use winit::window::WindowId;

#[derive(Debug)]
enum MyEvent {
    CreateWindow(WindowCreation),
}

struct WindowContextImpl {
    pub hwnd: PtrRepr,
    pub message_queue: Sender,
    pub winit_window: winit::window::Window,
}

struct WindowsContextImpl {
    event_loop: EventLoop<MyEvent>,
    windows: HashMap<WindowId, WindowContextImpl>,
}

impl WindowsContextImpl {
    fn new() -> Self {
        WindowsContextImpl {
            event_loop: EventLoop::new_any_thread(),
            windows: HashMap::new(),
        }
    }

    fn proxy(&self) -> EventLoopProxy<MyEvent> {
        self.event_loop.create_proxy()
    }

    #[allow(clippy::single_match)]
    fn window_event_loop(mut self) {
        self.event_loop.run(move |event, target, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { window_id, event } => {
                    trace!("WindowEvent: {:?}", event);

                    if let WindowEvent::CloseRequested = event {
                        let window = self.windows.remove(&window_id).unwrap();

                        // just stop!
                        // TODO: this is not how windows handles this stuff =)
                        // we should send a WM_CLOSE and wait for DefWindowProc to destroy the window
                        window
                            .message_queue
                            .send(Message {
                                hwnd: window.hwnd,
                                payload: MessagePayload::Quit { status: 0 },
                            })
                            .unwrap();
                    } else if let WindowEvent::Destroyed = event {
                        // ignore it, the window was dropped already
                    } else {
                        let window = self.windows.get(&window_id).unwrap();

                        match event {
                            WindowEvent::CursorMoved { position, .. } => {
                                let position = position.cast::<i16>();

                                window
                                    .message_queue
                                    .send(Message {
                                        hwnd: window.hwnd,
                                        payload: MessagePayload::MouseMove(MouseMessage {
                                            point: position.into(),
                                            keys: (),
                                        }),
                                    })
                                    .unwrap();
                            }
                            // WindowEvent::CursorEntered { .. } => {}
                            // WindowEvent::CursorLeft { .. } => {}
                            // WindowEvent::MouseWheel { .. } => {}
                            // WindowEvent::MouseInput { .. } => {}
                            _ => {}
                        }
                    }
                }
                Event::UserEvent(event) => {
                    trace!("UserEvent: {:?}", event);
                    match event {
                        MyEvent::CreateWindow(WindowCreation {
                            hwnd,
                            message_queue,
                            size,
                        }) => {
                            let winit_window = winit::window::WindowBuilder::new()
                                .with_inner_size(PhysicalSize::new(size.0, size.1))
                                .with_resizable(false)
                                .build(target)
                                .expect("Could not create a window");

                            let window_id = winit_window.id();

                            let window = WindowContextImpl {
                                hwnd,
                                message_queue,
                                winit_window,
                            };

                            self.windows.insert(window_id, window);
                        }
                    }
                }
                _ => (),
            }
        });
    }
}

#[derive(Debug)]
pub struct WindowCreation {
    pub hwnd: PtrRepr,
    pub message_queue: Sender,
    pub size: (u32, u32),
}

pub struct WindowsContext {
    event_thread: JoinHandle<()>,
    proxy: EventLoopProxy<MyEvent>,
}

impl WindowsContext {
    pub fn new() -> Self {
        let (proxy_send, proxy_recv) = crossbeam_channel::bounded(1);
        let event_thread = thread::spawn(move || {
            let context_impl = WindowsContextImpl::new();

            let proxy = context_impl.proxy();
            proxy_send.send(proxy).unwrap();

            context_impl.window_event_loop()
        });

        let proxy = proxy_recv.recv().unwrap();

        Self {
            event_thread,
            proxy,
        }
    }

    pub fn create_window(&self, creation: WindowCreation) {
        self.proxy
            .send_event(MyEvent::CreateWindow(creation))
            .unwrap()
    }
}

impl Default for WindowsContext {
    fn default() -> Self {
        WindowsContext::new()
    }
}
