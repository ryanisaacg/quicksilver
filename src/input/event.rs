#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(all(not(target_arch="wasm32"), not(target_os="macos"), feature="gamepads"))]
extern crate gilrs;

use input::{ButtonState, GamepadAxis, GamepadButton, Key, KEY_LIST, MouseButton};
use geom::Vector;
use graphics::Window;
#[cfg(not(target_arch="wasm32"))]
use glutin::{EventsLoop, Event::{WindowEvent}};

/// An input event
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
    GamepadAxis(u32, GamepadAxis, f32),
    /// A gamepad button has changed its state
    GamepadButton(u32, GamepadButton, ButtonState),
    /// A gamepad has been connected
    GamepadConnected(u32),
    /// A gamepad has been disconnected
    GamepadDisconnected(u32)
}

const LINES_TO_PIXELS: f32 = 15.0;

#[cfg(not(target_arch="wasm32"))]
pub(crate) struct EventProvider {
    events_loop: EventsLoop,
    #[cfg(all(not(target_os="macos"), feature="gamepads"))]
    gilrs: gilrs::Gilrs
}

#[cfg(not(target_arch="wasm32"))]
impl EventProvider {
    pub(crate) fn new(events_loop: EventsLoop) -> EventProvider {
        EventProvider { 
            events_loop, 
            #[cfg(all(not(target_os="macos"), feature="gamepads"))]
            gilrs: gilrs::Gilrs::new().unwrap()
        }
    }

    pub(crate) fn generate_events(&mut self, window: &mut Window, events: &mut Vec<Event>) -> bool {
        let mut running = true;
        //TODO: Make sure only novel events hit the user
        self.events_loop.poll_events(|event| match event {
            WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => {
                    running = false;
                    events.push(Event::Closed);
                }
                glutin::WindowEvent::KeyboardInput { input: event, .. } => {
                    if let Some(keycode) = event.virtual_keycode {
                        let state = match event.state {
                            glutin::ElementState::Pressed => ButtonState::Pressed,
                            glutin::ElementState::Released => ButtonState::Released
                        };
                        let key = KEY_LIST[keycode as usize];
                        events.push(Event::Key(key, state));
                    }
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position;
                    let position = (Vector::new(x as f32, y as f32) - window.screen_offset()) / window.scale_factor;
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
                    window.adjust_size(Vector::new(new_width as f32, new_height as f32));
                },
                _ => ()
            },
            _ => ()
        });
        #[cfg(all(not(target_os="macos"), feature="gamepads"))]
        while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
            use input::GAMEPAD_BUTTON_LIST;
            use gilrs::{Axis, Button, EventType};
            let id = id as u32;
            fn convert_button(button: Button) -> Option<GamepadButton> {
                Some(match button {
                    Button::South => GamepadButton::FaceDown,
                    Button::East => GamepadButton::FaceRight,
                    Button::North => GamepadButton::FaceUp,
                    Button::West => GamepadButton::FaceLeft,
                    Button::LeftTrigger => GamepadButton::ShoulderLeft,
                    Button::LeftTrigger2 => GamepadButton::TriggerLeft,
                    Button::RightTrigger => GamepadButton::ShoulderRight,
                    Button::RightTrigger2 => GamepadButton::TriggerRight,
                    Button::Select => GamepadButton::Select,
                    Button::Start => GamepadButton::Start,
                    Button::Mode => GamepadButton::Home,
                    Button::LeftThumb => GamepadButton::StickButtonLeft,
                    Button::RightThumb => GamepadButton::StickButtonRight,
                    Button::DPadUp => GamepadButton::DpadUp,
                    Button::DPadDown => GamepadButton::DpadDown,
                    Button::DPadLeft => GamepadButton::DpadLeft,
                    Button::DPadRight => GamepadButton::DpadRight,
                    _ => return None
                })
            }
            match event {
                EventType::ButtonPressed(button, _) => {
                    match convert_button(button) {
                        Some(button) => events.push(Event::GamepadButton(id, button, ButtonState::Pressed)),
                        None => continue
                    }
                },
                EventType::ButtonReleased(button, _) => { 
                    match convert_button(button) {
                        Some(button) => events.push(Event::GamepadButton(id, button, ButtonState::Released)),
                        None => continue
                    }
                },

                EventType::AxisChanged(axis, value, _) => events.push(Event::GamepadAxis(id, match axis {
                    Axis::LeftStickX => GamepadAxis::LeftStickX,
                    Axis::LeftStickY => GamepadAxis::LeftStickY,
                    Axis::RightStickX => GamepadAxis::RightStickX,
                    Axis::RightStickY => GamepadAxis::RightStickY,
                    _ => continue
                }, value)),
                EventType::Connected => events.push(Event::GamepadConnected(id)),
                EventType::Disconnected => events.push(Event::GamepadDisconnected(id)),
                _ => ()
            }
        }
        running
    }
}

