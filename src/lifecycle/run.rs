use crate::{
    Result,
    geom::Vector,
    lifecycle::{Application, State, Settings, Window},
};
#[cfg(not(target_arch = "wasm32"))]
use {
    crate::lifecycle::EventProvider,
    std::env::set_current_dir,
};
#[cfg(target_arch = "wasm32")]
use {
    crate::{
        input::{ButtonState, MouseButton, KEY_LIST, LINES_TO_PIXELS},
        lifecycle::Event,
    },
    std::{
        cell::{RefCell, RefMut},
        rc::Rc
    },
    stdweb::{
        Value, unstable::TryInto,
        web::{
            document, window, IEventTarget, IHtmlElement,  IWindowOrWorker,
            event::{
                BlurEvent, ConcreteEvent, FocusEvent, GamepadConnectedEvent, GamepadDisconnectedEvent,
                IGamepadEvent, IKeyboardEvent, IMouseEvent, KeyDownEvent, KeyUpEvent,
                MouseButton as WebMouseButton, PointerDownEvent, PointerMoveEvent, PointerOutEvent,
                PointerOverEvent, PointerUpEvent, ResizeEvent
            },
            html_element::InputElement
        }
    }
};

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
pub fn run<T: State>(title: &str, size: Vector, settings: Settings) {
    if let Err(error) = run_impl::<T>(title, size.into(), settings) {
        T::handle_error(error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_impl<T: State>(title: &str, size: Vector, settings: Settings) -> Result<()> {
    // A workaround for https://github.com/koute/cargo-web/issues/112
    if let Err(_) = set_current_dir("static") {
        eprintln!("Warning: no asset directory found. Please place all your assets inside a directory called 'static' so they can be loaded");
        eprintln!("Execution continuing, but any asset-not-found errors are likely due to the lack of a 'static' directory.")
    }
    let (window, events_loop) = Window::build(title, size, settings)?;
    let mut events = EventProvider::new(events_loop);
    #[cfg(feature = "sounds")]
    crate::sound::Sound::initialize();
    let mut app: Application<T> = Application::new(window)?;
    while app.window.is_running() {
        let stay_open = events.generate_events(&mut app.window, &mut app.event_buffer);
        if !stay_open {
            app.window.close();
        }
        app.update()?;
        app.draw()?;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn run_impl<T: State>(title: &str, size: Vector, settings: Settings) -> Result<()> {
    let (win, canvas, input) = Window::build(title, size, settings)?;

    let app: Rc<RefCell<Application<T>>> = Rc::new(RefCell::new(Application::new(win)?));

    let input_rc: Rc<InputElement> = Rc::new(input);

    let document = document();
    let window = window();

    let application = app.clone();
    let close_handler = move || application.borrow_mut().event_buffer.push(Event::Closed);
    js! {
        window.onclose = @{close_handler};
    }

    let application = app.clone();
    let wheel_handler = move |x: Value, y: Value, mode: Value| {
        let x: f64 = x.try_into().unwrap_or(0.0);
        let y: f64 = y.try_into().unwrap_or(0.0);
        let mode: u64 = mode.try_into().unwrap_or(0);
        application
            .borrow_mut()
            .event_buffer
            .push(Event::MouseWheel(
                Vector::new(x as f32, y as f32) * if mode != 0 { LINES_TO_PIXELS } else { 1.0 },
            ));
    };
    js! {
        @{&canvas}.onwheel = function(e) {
            @{wheel_handler}(e.deltaX, e.deltaY, e.deltaMode);
            e.preventDefault();
        }
    }

    let application = app.clone();
    let input = input_rc.clone();
    let document_composition_end_handler = move |event: Value| {
        let data: String = js! { return @{&event}.data }.try_into().unwrap();

        js! {
            @{&*input}.value = "";
            @{&event}.preventDefault();
        }

        match data.chars().next() {
            Some(ch) if ch.is_alphanumeric() => application.borrow_mut().event_buffer.push(Event::Typed(ch)),
            _ => (),
        }
    };

    js! {
        document.addEventListener("compositionend", @{document_composition_end_handler});
    }

    let input = input_rc.clone();
    handle_event(&document, &app, move |mut app, _: BlurEvent| {
        app.event_buffer.push(Event::Unfocused);
        input.blur();
    });

    let input = input_rc.clone();
    handle_event(&document, &app, move |mut app, _: FocusEvent| {
        app.event_buffer.push(Event::Focused);
        input.focus();
    });

    handle_event(&canvas, &app, |mut app, _: PointerOutEvent| {
        app.event_buffer.push(Event::MouseExited)
    });
    handle_event(&canvas, &app, |mut app, _: PointerOverEvent| {
        app.event_buffer.push(Event::MouseEntered)
    });

    handle_event(&canvas, &app, |mut app, event: PointerMoveEvent| {
        let pointer = Vector::new(event.offset_x() as f32, event.offset_y() as f32);
        app.event_buffer.push(Event::MouseMoved(pointer));
    });
    handle_event(&canvas, &app, |mut app, event: PointerUpEvent| {
        let state = ButtonState::Released;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return,
        };
        app.event_buffer.push(Event::MouseButton(button, state));
    });

    let input = input_rc.clone();
    handle_event(&canvas, &app, move |mut app, event: PointerDownEvent| {
        let state = ButtonState::Pressed;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return,
        };
        app.event_buffer.push(Event::MouseButton(button, state));
        input.focus();
    });

    let key_names = generate_key_names();
    handle_event(&document, &app, move |mut app, event: KeyDownEvent| {
        // If the key press is 'Backspace', we shouldn't send a typed event
        // However, if it is a single alphanumeric character, that should be a typed event
        // TODO: this is imperfect at best, a better way to decide what codes get sent to the
        // application would be desirable
        let string = event.key();
        let mut characters = string.chars();
        let first = characters.next();
        let second = characters.next();
        match (first, second) {
            (Some(ch), None) if ch.is_alphanumeric() => app.event_buffer.push(Event::Typed(ch)),
            _ => ()
        }
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event_buffer.push(Event::Key(KEY_LIST[*keycode], ButtonState::Pressed));
        }
    });
    let key_names = generate_key_names();
    handle_event(&document, &app, move |mut app, event: KeyUpEvent| {
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event_buffer.push(Event::Key(KEY_LIST[*keycode], ButtonState::Released));
        }
    });

    handle_event(&window, &app, move |mut app, event: GamepadConnectedEvent| {
        app.event_buffer.push(Event::GamepadConnected(event.gamepad().index() as i32));
    });
    handle_event(&window, &app, move |mut app, event: GamepadDisconnectedEvent| {
        app.event_buffer.push(Event::GamepadDisconnected(event.gamepad().index() as i32));
    });
    handle_event(&window, &app, move |mut app, _: ResizeEvent| {
        if app.window.get_fullscreen() {
            app.window.set_fullscreen(true);
        }
    });

    update(app.clone())?;
    draw(app.clone())
}

#[cfg(target_arch = "wasm32")]
fn update<T: State>(app: Rc<RefCell<Application<T>>>) -> Result<()> {
    app.borrow_mut().update()?;
    let duration = app.borrow_mut().window.update_rate();
    if app.borrow().window.is_running() {
        window().set_timeout(move || if let Err(error) = update(app) {
            T::handle_error(error)
        }, duration as u32);
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn draw<T: State>(app: Rc<RefCell<Application<T>>>) -> Result<()> {
    app.borrow_mut().draw()?;
    if app.borrow().window.is_running() {
        window().request_animation_frame(move |_| if let Err(error) = draw(app) {
            T::handle_error(error)
        });
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn handle_event<T, E, F>(
    target: &impl IEventTarget,
    application: &Rc<RefCell<Application<T>>>,
    mut handler: F,
) where
    T: State,
    E: ConcreteEvent,
    F: FnMut(RefMut<Application<T>>, E) + 'static,
{
    let application = application.clone();
    target.add_event_listener(move |event: E| {
        event.prevent_default();
        event.stop_propagation();
        event.cancel_bubble();
        handler(application.borrow_mut(), event);
    });
}

#[cfg(any(test, target_arch = "wasm32"))]
static KEY_NAMES: &[&str] = &[
    "Digit1",
    "Digit2",
    "Digit3",
    "Digit4",
    "Digit5",
    "Digit6",
    "Digit7",
    "Digit8",
    "Digit9",
    "Digit0",
    "KeyA",
    "KeyB",
    "KeyC",
    "KeyD",
    "KeyE",
    "KeyF",
    "KeyG",
    "KeyH",
    "KeyI",
    "KeyJ",
    "KeyK",
    "KeyL",
    "KeyM",
    "KeyN",
    "KeyO",
    "KeyP",
    "KeyQ",
    "KeyR",
    "KeyS",
    "KeyT",
    "KeyU",
    "KeyV",
    "KeyW",
    "KeyX",
    "KeyY",
    "KeyZ",
    "Escape",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
    "PrintScreen",
    "ScrollLock",
    "Pause",
    "Insert",
    "Home",
    "Delete",
    "End",
    "PageDown",
    "PageUp",
    "ArrowLeft",
    "ArrowUp",
    "ArrowRight",
    "ArrowDown",
    "Backspace",
    "Enter",
    "Space",
    "Compose",
    "Caret",
    "NumLock",
    "Numpad0",
    "Numpad1",
    "Numpad2",
    "Numpad3",
    "Numpad4",
    "Numpad5",
    "Numpad6",
    "Numpad7",
    "Numpad8",
    "Numpad9",
    "AbntC1",
    "AbntC2",
    "Add",
    "Quote",
    "Apps",
    "At",
    "Ax",
    "Backslash",
    "Calculator",
    "Capital",
    "Colon",
    "Comma",
    "Convert",
    "Decimal",
    "Divide",
    "Equal",
    "Backquote",
    "Kana",
    "Kanji",
    "AltLeft",
    "BracketLeft",
    "ControlLeft",
    "ShiftLeft",
    "MetaLeft",
    "Mail",
    "MediaSelect",
    "MediaStop",
    "Minus",
    "Multiply",
    "Mute",
    "LaunchMyComputer",
    "NavigateForward",
    "NavigateBackward",
    "NextTrack",
    "NoConvert",
    "NumpadComma",
    "NumpadEnter",
    "NumpadEquals",
    "OEM102",
    "Period",
    "PlayPause",
    "Power",
    "PrevTrack",
    "AltRight",
    "BracketRight",
    "ControlRight",
    "ShiftRight",
    "MetaRight",
    "Semicolon",
    "Slash",
    "Sleep",
    "Stop",
    "Subtract",
    "Sysrq",
    "Tab",
    "Underline",
    "Unlabeled",
    "AudioVolumeDown",
    "AudioVolumeUp",
    "Wake",
    "WebBack",
    "WebFavorites",
    "WebForward",
    "WebHome",
    "WebRefresh",
    "WebSearch",
    "WebStop",
    "Yen",
];

#[cfg(any(test, target_arch = "wasm32"))]
use std::collections::HashMap;

#[cfg(any(test, target_arch = "wasm32"))]
fn generate_key_names() -> HashMap<String, usize> {
    KEY_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| (String::from(*name), index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Key;

    #[test]
    fn web_key_constants_match() {
        let mapping = generate_key_names();
        assert_eq!(Key::Key1 as usize, mapping["Digit1"]);
        assert_eq!(Key::Key2 as usize, mapping["Digit2"]);
        assert_eq!(Key::Key3 as usize, mapping["Digit3"]);
        assert_eq!(Key::Key4 as usize, mapping["Digit4"]);
        assert_eq!(Key::Key5 as usize, mapping["Digit5"]);
        assert_eq!(Key::Key6 as usize, mapping["Digit6"]);
        assert_eq!(Key::Key7 as usize, mapping["Digit7"]);
        assert_eq!(Key::Key8 as usize, mapping["Digit8"]);
        assert_eq!(Key::Key9 as usize, mapping["Digit9"]);
        assert_eq!(Key::Key0 as usize, mapping["Digit0"]);
        assert_eq!(Key::A as usize, mapping["KeyA"]);
        assert_eq!(Key::B as usize, mapping["KeyB"]);
        assert_eq!(Key::C as usize, mapping["KeyC"]);
        assert_eq!(Key::D as usize, mapping["KeyD"]);
        assert_eq!(Key::E as usize, mapping["KeyE"]);
        assert_eq!(Key::F as usize, mapping["KeyF"]);
        assert_eq!(Key::G as usize, mapping["KeyG"]);
        assert_eq!(Key::H as usize, mapping["KeyH"]);
        assert_eq!(Key::I as usize, mapping["KeyI"]);
        assert_eq!(Key::J as usize, mapping["KeyJ"]);
        assert_eq!(Key::K as usize, mapping["KeyK"]);
        assert_eq!(Key::L as usize, mapping["KeyL"]);
        assert_eq!(Key::M as usize, mapping["KeyM"]);
        assert_eq!(Key::N as usize, mapping["KeyN"]);
        assert_eq!(Key::O as usize, mapping["KeyO"]);
        assert_eq!(Key::P as usize, mapping["KeyP"]);
        assert_eq!(Key::Q as usize, mapping["KeyQ"]);
        assert_eq!(Key::R as usize, mapping["KeyR"]);
        assert_eq!(Key::S as usize, mapping["KeyS"]);
        assert_eq!(Key::T as usize, mapping["KeyT"]);
        assert_eq!(Key::U as usize, mapping["KeyU"]);
        assert_eq!(Key::V as usize, mapping["KeyV"]);
        assert_eq!(Key::W as usize, mapping["KeyW"]);
        assert_eq!(Key::X as usize, mapping["KeyX"]);
        assert_eq!(Key::Y as usize, mapping["KeyY"]);
        assert_eq!(Key::Z as usize, mapping["KeyZ"]);
        assert_eq!(Key::Escape as usize, mapping["Escape"]);
        assert_eq!(Key::F1 as usize, mapping["F1"]);
        assert_eq!(Key::F2 as usize, mapping["F2"]);
        assert_eq!(Key::F3 as usize, mapping["F3"]);
        assert_eq!(Key::F4 as usize, mapping["F4"]);
        assert_eq!(Key::F5 as usize, mapping["F5"]);
        assert_eq!(Key::F6 as usize, mapping["F6"]);
        assert_eq!(Key::F7 as usize, mapping["F7"]);
        assert_eq!(Key::F8 as usize, mapping["F8"]);
        assert_eq!(Key::F9 as usize, mapping["F9"]);
        assert_eq!(Key::F10 as usize, mapping["F10"]);
        assert_eq!(Key::F11 as usize, mapping["F11"]);
        assert_eq!(Key::F12 as usize, mapping["F12"]);
        assert_eq!(Key::F13 as usize, mapping["F13"]);
        assert_eq!(Key::F14 as usize, mapping["F14"]);
        assert_eq!(Key::F15 as usize, mapping["F15"]);
        assert_eq!(Key::F16 as usize, mapping["F16"]);
        assert_eq!(Key::F17 as usize, mapping["F17"]);
        assert_eq!(Key::F18 as usize, mapping["F18"]);
        assert_eq!(Key::F19 as usize, mapping["F19"]);
        assert_eq!(Key::F20 as usize, mapping["F20"]);
        assert_eq!(Key::F21 as usize, mapping["F21"]);
        assert_eq!(Key::F22 as usize, mapping["F22"]);
        assert_eq!(Key::F23 as usize, mapping["F23"]);
        assert_eq!(Key::F24 as usize, mapping["F24"]);
        assert_eq!(Key::Snapshot as usize, mapping["PrintScreen"]);
        assert_eq!(Key::Scroll as usize, mapping["ScrollLock"]);
        assert_eq!(Key::Pause as usize, mapping["Pause"]);
        assert_eq!(Key::Insert as usize, mapping["Insert"]);
        assert_eq!(Key::Home as usize, mapping["Home"]);
        assert_eq!(Key::Delete as usize, mapping["Delete"]);
        assert_eq!(Key::End as usize, mapping["End"]);
        assert_eq!(Key::PageDown as usize, mapping["PageDown"]);
        assert_eq!(Key::PageUp as usize, mapping["PageUp"]);
        assert_eq!(Key::Left as usize, mapping["ArrowLeft"]);
        assert_eq!(Key::Up as usize, mapping["ArrowUp"]);
        assert_eq!(Key::Right as usize, mapping["ArrowRight"]);
        assert_eq!(Key::Down as usize, mapping["ArrowDown"]);
        assert_eq!(Key::Back as usize, mapping["Backspace"]);
        assert_eq!(Key::Return as usize, mapping["Enter"]);
        assert_eq!(Key::Space as usize, mapping["Space"]);
        assert_eq!(Key::Compose as usize, mapping["Compose"]);
        assert_eq!(Key::Caret as usize, mapping["Caret"]);
        assert_eq!(Key::Numlock as usize, mapping["NumLock"]);
        assert_eq!(Key::Numpad0 as usize, mapping["Numpad0"]);
        assert_eq!(Key::Numpad1 as usize, mapping["Numpad1"]);
        assert_eq!(Key::Numpad2 as usize, mapping["Numpad2"]);
        assert_eq!(Key::Numpad3 as usize, mapping["Numpad3"]);
        assert_eq!(Key::Numpad4 as usize, mapping["Numpad4"]);
        assert_eq!(Key::Numpad5 as usize, mapping["Numpad5"]);
        assert_eq!(Key::Numpad6 as usize, mapping["Numpad6"]);
        assert_eq!(Key::Numpad7 as usize, mapping["Numpad7"]);
        assert_eq!(Key::Numpad8 as usize, mapping["Numpad8"]);
        assert_eq!(Key::Numpad9 as usize, mapping["Numpad9"]);
        assert_eq!(Key::AbntC1 as usize, mapping["AbntC1"]);
        assert_eq!(Key::AbntC2 as usize, mapping["AbntC2"]);
        assert_eq!(Key::Add as usize, mapping["Add"]);
        assert_eq!(Key::Apostrophe as usize, mapping["Quote"]);
        assert_eq!(Key::Apps as usize, mapping["Apps"]);
        assert_eq!(Key::At as usize, mapping["At"]);
        assert_eq!(Key::Ax as usize, mapping["Ax"]);
        assert_eq!(Key::Backslash as usize, mapping["Backslash"]);
        assert_eq!(Key::Calculator as usize, mapping["Calculator"]);
        assert_eq!(Key::Capital as usize, mapping["Capital"]);
        assert_eq!(Key::Colon as usize, mapping["Colon"]);
        assert_eq!(Key::Comma as usize, mapping["Comma"]);
        assert_eq!(Key::Convert as usize, mapping["Convert"]);
        assert_eq!(Key::Decimal as usize, mapping["Decimal"]);
        assert_eq!(Key::Divide as usize, mapping["Divide"]);
        assert_eq!(Key::Equals as usize, mapping["Equal"]);
        assert_eq!(Key::Grave as usize, mapping["Backquote"]);
        assert_eq!(Key::Kana as usize, mapping["Kana"]);
        assert_eq!(Key::Kanji as usize, mapping["Kanji"]);
        assert_eq!(Key::LAlt as usize, mapping["AltLeft"]);
        assert_eq!(Key::LBracket as usize, mapping["BracketLeft"]);
        assert_eq!(Key::LControl as usize, mapping["ControlLeft"]);
        assert_eq!(Key::LShift as usize, mapping["ShiftLeft"]);
        assert_eq!(Key::LWin as usize, mapping["MetaLeft"]);
        assert_eq!(Key::Mail as usize, mapping["Mail"]);
        assert_eq!(Key::MediaSelect as usize, mapping["MediaSelect"]);
        assert_eq!(Key::MediaStop as usize, mapping["MediaStop"]);
        assert_eq!(Key::Minus as usize, mapping["Minus"]);
        assert_eq!(Key::Multiply as usize, mapping["Multiply"]);
        assert_eq!(Key::Mute as usize, mapping["Mute"]);
        assert_eq!(Key::MyComputer as usize, mapping["LaunchMyComputer"]);
        assert_eq!(Key::NavigateForward as usize, mapping["NavigateForward"]);
        assert_eq!(Key::NavigateBackward as usize, mapping["NavigateBackward"]);
        assert_eq!(Key::NextTrack as usize, mapping["NextTrack"]);
        assert_eq!(Key::NoConvert as usize, mapping["NoConvert"]);
        assert_eq!(Key::NumpadComma as usize, mapping["NumpadComma"]);
        assert_eq!(Key::NumpadEnter as usize, mapping["NumpadEnter"]);
        assert_eq!(Key::NumpadEquals as usize, mapping["NumpadEquals"]);
        assert_eq!(Key::OEM102 as usize, mapping["OEM102"]);
        assert_eq!(Key::Period as usize, mapping["Period"]);
        assert_eq!(Key::PlayPause as usize, mapping["PlayPause"]);
        assert_eq!(Key::Power as usize, mapping["Power"]);
        assert_eq!(Key::PrevTrack as usize, mapping["PrevTrack"]);
        assert_eq!(Key::RAlt as usize, mapping["AltRight"]);
        assert_eq!(Key::RBracket as usize, mapping["BracketRight"]);
        assert_eq!(Key::RControl as usize, mapping["ControlRight"]);
        assert_eq!(Key::RShift as usize, mapping["ShiftRight"]);
        assert_eq!(Key::RWin as usize, mapping["MetaRight"]);
        assert_eq!(Key::Semicolon as usize, mapping["Semicolon"]);
        assert_eq!(Key::Slash as usize, mapping["Slash"]);
        assert_eq!(Key::Sleep as usize, mapping["Sleep"]);
        assert_eq!(Key::Stop as usize, mapping["Stop"]);
        assert_eq!(Key::Subtract as usize, mapping["Subtract"]);
        assert_eq!(Key::Sysrq as usize, mapping["Sysrq"]);
        assert_eq!(Key::Tab as usize, mapping["Tab"]);
        assert_eq!(Key::Underline as usize, mapping["Underline"]);
        assert_eq!(Key::Unlabeled as usize, mapping["Unlabeled"]);
        assert_eq!(Key::VolumeDown as usize, mapping["AudioVolumeDown"]);
        assert_eq!(Key::VolumeUp as usize, mapping["AudioVolumeUp"]);
        assert_eq!(Key::Wake as usize, mapping["Wake"]);
        assert_eq!(Key::WebBack as usize, mapping["WebBack"]);
        assert_eq!(Key::WebFavorites as usize, mapping["WebFavorites"]);
        assert_eq!(Key::WebForward as usize, mapping["WebForward"]);
        assert_eq!(Key::WebHome as usize, mapping["WebHome"]);
        assert_eq!(Key::WebRefresh as usize, mapping["WebRefresh"]);
        assert_eq!(Key::WebSearch as usize, mapping["WebSearch"]);
        assert_eq!(Key::WebStop as usize, mapping["WebStop"]);
        assert_eq!(Key::Yen as usize, mapping["Yen"]);
    }
}
