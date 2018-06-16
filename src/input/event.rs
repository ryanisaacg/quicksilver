#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use input::{ButtonState, GamepadAxis, GamepadButton, Key, MouseButton};
use geom::Vector;
#[cfg(not(target_arch="wasm32"))]
use graphics::Window;
#[cfg(not(target_arch="wasm32"))]
use glutin::{EventsLoop, Event::{WindowEvent}};

/// An input event
#[derive(Copy, Clone, Debug)]
pub enum Event {
    /// The application has been closed
    Closed,
    /// The application has gained focus
    Focused,
    /// The application has lost focus
    Unfocused,
    /// A key has changed its button state
    Key(Key, ButtonState),
    /// The mouse has been moved to a position
    MouseMoved(Vector),
    /// The mouse has entered the window
    MouseEntered,
    /// The mouse has exited the window
    MouseExited,
    /// The mouse wheel has been scrolled by a vector
    MouseWheel(Vector),
    /// A mouse button has changed its button state
    MouseButton(MouseButton, ButtonState),
    /// A gamepad axis has changed its state
    GamepadAxis(i32, GamepadAxis, f32),
    /// A gamepad button has changed its state
    GamepadButton(i32, GamepadButton, ButtonState),
    /// A gamepad has been connected
    GamepadConnected(i32),
    /// A gamepad has been disconnected
    GamepadDisconnected(i32)
}

#[cfg(not(target_arch="wasm32"))]
const LINES_TO_PIXELS: f32 = 15.0;

#[cfg(not(target_arch="wasm32"))]
pub(crate) struct EventProvider {
    events_loop: EventsLoop
}

#[cfg(not(target_arch="wasm32"))]
impl EventProvider {
    pub(crate) fn new(events_loop: EventsLoop) -> EventProvider {
        EventProvider { 
            events_loop
        }
    }

    pub(crate) fn generate_events(&mut self, window: &mut Window, events: &mut Vec<Event>) -> bool {
        let mut running = true;
        self.events_loop.poll_events(|event| match event {
            WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => {
                    running = false;
                    events.push(Event::Closed);
                }
                glutin::WindowEvent::KeyboardInput { input: event, .. } => {
                    if let Some(keycode) = event.virtual_keycode {
                        let state = match event.state {
                            glutin::ElementState::Pressed => ButtonState::Pressed,
                            glutin::ElementState::Released => ButtonState::Released
                        };
                        let key = ::input::KEY_LIST[keycode as usize];
                        events.push(Event::Key(key, state));
                    }
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position;
                    let position = (Vector::new(x as f32, y as f32) - window.screen_offset()) / window.scale_factor;
                    let position = window.project() * position;
                    events.push(Event::MouseMoved(position));
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    let value = match state {
                        glutin::ElementState::Pressed => ButtonState::Pressed,
                        glutin::ElementState::Released => ButtonState::Released,
                    };
                    let index = match button {
                        glutin::MouseButton::Left => MouseButton::Left,
                        glutin::MouseButton::Right => MouseButton::Right,
                        glutin::MouseButton::Middle => MouseButton::Middle,
                        // Other mouse buttons just mean we should move on to the next glutin event
                        _ => { return; },
                    };
                    events.push(Event::MouseButton(index, value));
                }
                glutin::WindowEvent::MouseWheel { delta, .. } => {
                    let (x, y) = match delta {
                        glutin::MouseScrollDelta::LineDelta(x, y) => (x * LINES_TO_PIXELS, y * -LINES_TO_PIXELS),
                        glutin::MouseScrollDelta::PixelDelta(x, y) => (x, y)
                    };
                    let vector = Vector::new(x, y);
                    events.push(Event::MouseMoved(vector));
                }
                glutin::WindowEvent::Resized(new_width, new_height) => {
                    // Glutin reports a resize to 0, 0 when minimizing the window
                    if new_width != 0 && new_height != 0 {
                        window.adjust_size(Vector::new(new_width as f32, new_height as f32));
                    }
                },
                _ => ()
            },
            _ => ()
        });
        running
    }
}

