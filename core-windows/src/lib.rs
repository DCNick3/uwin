use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use tracing::trace;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceId, ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};

use core_message_queue::{Message, MessagePayload, MouseMessage, Sender};
use winit::window::Window;

pub use winit::window::WindowId;

#[derive(Debug)]
enum MyEvent {
    CreateWindow {
        rq: WindowCreation,
        response_chan: crossbeam_channel::Sender<WindowId>,
    },
    GetWindow {
        window_id: WindowId,
        response_chan: crossbeam_channel::Sender<Option<Arc<Window>>>,
    },
}

struct WindowContextImpl {
    pub message_queue: Sender,
    #[allow(unused)]
    pub winit_window: Arc<Window>,
}

struct WindowsContextImpl {
    event_loop: EventLoop<MyEvent>,
    windows: HashMap<WindowId, WindowContextImpl>,
    mouse_positions: HashMap<DeviceId, PhysicalPosition<f64>>,
}

impl WindowsContextImpl {
    fn new() -> Self {
        use winit::platform::x11::EventLoopBuilderExtX11;
        WindowsContextImpl {
            event_loop: EventLoopBuilder::with_user_event()
                .with_any_thread(true)
                .build(),
            windows: HashMap::new(),
            mouse_positions: HashMap::new(),
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
                                window_id: Some(window_id),
                                payload: MessagePayload::Quit { status: 0 },
                            })
                            .unwrap();
                    } else if let WindowEvent::Destroyed = event {
                        // ignore it, the window was dropped already
                    } else {
                        let window = self.windows.get(&window_id).unwrap();

                        match event {
                            WindowEvent::CursorMoved {
                                position,
                                device_id,
                                ..
                            } => {
                                self.mouse_positions.insert(device_id, position);

                                let position = position.cast::<i16>();

                                window
                                    .message_queue
                                    .send(Message {
                                        window_id: Some(window_id),
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
                            WindowEvent::MouseInput {
                                button,
                                device_id,
                                state,
                                ..
                            } => {
                                let position = self
                                    .mouse_positions
                                    .get(&device_id)
                                    .cloned()
                                    .unwrap_or_default();
                                let position = position.cast::<i16>();

                                let mouse_message = MouseMessage {
                                    point: position.into(),
                                    keys: (),
                                };

                                use ElementState::*;
                                use MessagePayload::{
                                    LButtonDown, LButtonUp, RButtonDown, RButtonUp,
                                };
                                use MouseButton::*;

                                let payload = match (button, state) {
                                    (Left, Pressed) => Some(LButtonDown(mouse_message)),
                                    (Left, Released) => Some(LButtonUp(mouse_message)),
                                    (Right, Pressed) => Some(RButtonDown(mouse_message)),
                                    (Right, Released) => Some(RButtonUp(mouse_message)),
                                    (Middle | Other(_), _) => {
                                        // ignore
                                        None
                                    }
                                };

                                if let Some(payload) = payload {
                                    window
                                        .message_queue
                                        .send(Message {
                                            window_id: Some(window_id),
                                            payload,
                                        })
                                        .unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::UserEvent(event) => {
                    trace!("UserEvent: {:?}", event);
                    match event {
                        MyEvent::CreateWindow {
                            rq:
                                WindowCreation {
                                    message_queue,
                                    size,
                                },
                            response_chan,
                        } => {
                            let winit_window = Arc::new(
                                winit::window::WindowBuilder::new()
                                    .with_inner_size(PhysicalSize::new(size.0, size.1))
                                    .with_resizable(false)
                                    .build(target)
                                    .expect("Could not create a window"),
                            );

                            let window_id = winit_window.id();

                            response_chan.send(window_id).unwrap();

                            let window = WindowContextImpl {
                                message_queue,
                                winit_window,
                            };

                            self.windows.insert(window_id, window);
                        }
                        MyEvent::GetWindow {
                            window_id,
                            response_chan,
                        } => {
                            // let's do the full search here, it's not that much objects anyway

                            let window =
                                self.windows.get(&window_id).map(|w| w.winit_window.clone());

                            response_chan
                                .send(window)
                                .expect("Sending the response to GetWindow request");
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
    pub message_queue: Sender,
    pub size: (u32, u32),
}

pub struct WindowsContext {
    #[allow(unused)]
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

    pub fn create_window(&self, creation: WindowCreation) -> WindowId {
        let (response_chan, response_recv) = crossbeam_channel::bounded(1);

        self.proxy
            .send_event(MyEvent::CreateWindow {
                rq: creation,
                response_chan,
            })
            .unwrap();

        response_recv.recv().unwrap()
    }

    pub fn get_window(
        &self,
        window_id: WindowId,
    ) -> Option<Arc<impl HasRawWindowHandle + HasRawDisplayHandle>> {
        let (response_chan, response_recv) = crossbeam_channel::bounded(1);

        self.proxy
            .send_event(MyEvent::GetWindow {
                window_id,
                response_chan,
            })
            .unwrap();

        response_recv.recv().unwrap()
    }
}

impl Default for WindowsContext {
    fn default() -> Self {
        WindowsContext::new()
    }
}
