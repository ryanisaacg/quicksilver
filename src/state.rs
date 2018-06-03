#[cfg(target_arch="wasm32")]
use geom::Vector;
use graphics::{Window, WindowBuilder};
use input::Event;
#[cfg(target_arch="wasm32")]
use {
    input::{ButtonState, KEY_LIST, MouseButton},
    std::{
        cell::{RefCell, RefMut},
        collections::HashMap,
        rc::Rc
    },
    stdweb::web::{
        document,
        event::{BlurEvent, ConcreteEvent, FocusEvent, IKeyboardEvent, IMouseEvent, KeyDownEvent, KeyUpEvent, MouseButton as WebMouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent},
        IEventTarget, IParentNode,
    }
};

/// The structure responsible for managing the game loop state
pub trait State: 'static {
    /// Create the state given the window and canvas
    fn new() -> Self where Self: Sized;
    /// Tick the State forward one frame
    ///
    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS. 
    ///
    /// By default it does nothing
    fn update(&mut self, &mut Window) {}
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, &Event, &mut Window) {}
    /// Draw the state to the screen
    ///
    /// Will happen as often as possible, only limited by vysnc
    ///
    /// By default it draws a black screen
    fn draw(&mut self, window: &mut Window) {
        use graphics::Color;
        window.clear(Color::black());
        window.present();
    }
}

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
pub fn run<T: State>(window: WindowBuilder) {
    run_impl::<T>(window)
}

struct Application<T: State> {
    state: T, 
    window: Window,
    event_buffer: Vec<Event>
}

impl<T: State> Application<T> {
    fn update(&mut self) {
        self.state.update(&mut self.window);
    }

    fn draw(&mut self) {
        self.state.draw(&mut self.window);
    }

    #[cfg(target_arch="wasm32")]
    fn event(&mut self, event: &Event) {
        self.window.process_event(event);
        self.state.event(event, &mut self.window);
    }

    fn process_events(&mut self) {
        self.window.update_gamepads(&mut self.event_buffer);
        for i in 0..self.event_buffer.len() {
            self.window.process_event(&self.event_buffer[i]);
            self.state.event(&self.event_buffer[i], &mut self.window);
        }
        self.event_buffer.clear();
    }
}

#[cfg(not(target_arch="wasm32"))]
fn run_impl<T: State>(window: WindowBuilder) {
    use input::EventProvider;
    let (window, events_loop) = window.build();
    let mut events = EventProvider::new(events_loop);
    let event_buffer = Vec::new();
    let state = T::new();
    let mut app = Application { window, state, event_buffer };
    use std::time::Duration;
    #[cfg(feature="sounds")] {
        use sound::Sound;
        Sound::initialize();
    }
    let mut timer = ::Timer::new();
    let mut running = true;
    while running {
        running = events.generate_events(&mut app.window, &mut app.event_buffer);
        app.process_events();
        timer.tick(||  { 
            app.update(); 
            Duration::from_millis(16) 
        });
        app.draw();
        app.window.clear_temporary_states();
    }
}

#[cfg(target_arch="wasm32")]
fn run_impl<T: State>(window: WindowBuilder) {
    let window = window.build();
    let app = Rc::new(RefCell::new(Application { 
        window,
        state: T::new(),
        event_buffer: Vec::new()
    }));

    let document = document();
    let canvas = document.query_selector("#canvas").unwrap().unwrap();

    handle_event(&document, &app, |mut app, _: BlurEvent| app.event(&Event::Unfocused));
    handle_event(&document, &app, |mut app, _: FocusEvent| app.event(&Event::Focused));

    handle_event(&canvas, &app, |mut app, event: MouseMoveEvent| {
        let pointer = Vector::new(event.offset_x() as f32, event.offset_y() as f32);
        app.event(&Event::MouseMoved(pointer));
    });
    handle_event(&canvas, &app, |mut app, event: MouseUpEvent| {
        let state = ButtonState::Released;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return
        };
        app.event(&Event::MouseButton(button, state));
    });
    handle_event(&canvas, &app, |mut app, event: MouseDownEvent| {
        let state = ButtonState::Pressed;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return
        };
        app.event(&Event::MouseButton(button, state));
    });

    let key_names = generate_key_names();
    handle_event(&canvas, &app, move |mut app, event: KeyDownEvent| {
        let state = ButtonState::Pressed;
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event(&Event::Key(KEY_LIST[*keycode], state));
        }
    });
    let key_names = generate_key_names();
    handle_event(&canvas, &app, move |mut app, event: KeyUpEvent| {
        let state = ButtonState::Released;
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event(&Event::Key(KEY_LIST[*keycode], state));
        }
    });

}

#[cfg(target_arch="wasm32")]
fn handle_event<T, E, F>(target: &impl IEventTarget, application: &Rc<RefCell<Application<T>>>, mut handler: F) 
        where T: State, E: ConcreteEvent, F: FnMut(RefMut<Application<T>>, E) + 'static {
    let application = application.clone();
    target.add_event_listener(move |event: E| {
        event.prevent_default();
        handler(application.borrow_mut(), event);
    });
}

#[cfg(target_arch="wasm32")]
static KEY_NAMES: &[&str] = &["Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9", "Digit0", "KeyA", "KeyB", "KeyC", "KeyD", "KeyE", "KeyF", "KeyG", "KeyH", "KeyI", "KeyJ", "KeyK", "KeyL", "KeyM", 
    "KeyN", "KeyO", "KeyP", "KeyQ", "KeyR", "KeyS", "KeyT", "KeyU", "KeyV", "KeyW", "KeyX", "KeyY", "KeyZ", "Escape", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", 
    "F13", "F14", "F15", "PrintScreen", "ScrollLock", "Pause", "Insert", "Home", "Delete", "End", "PageDown", "PageUp", "ArrowLeft", "ArrowUp", "ArrowRight", 
    "ArrowDown", "Backspace", "Enter", "Space", "Compose", "Caret", "NumLock", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", 
    "Numpad6", "Numpad7", "Numpad8", "Numpad9", "AbntC1", "AbntC2", "Add", "Quote", "Apps", "At", "Ax", "Backslash", "Calculator", 
    "Capital", "Colon", "Comma", "Convert", "Decimal", "Divide", "Equal", "Backquote", "Kana", "Kanji", "AltLeft", "BracketLeft", "ControlLeft", 
    "LMenu", "ShiftLeft", "MetaLeft", "Mail", "MediaSelect", "MediaStop", "Minus", "Multiply", "Mute", "LaunchMyComputer", "NavigateForward", 
    "NavigateBackward", "NextTrack", "NoConvert", "NumpadComma", "NumpadEnter", "NumpadEquals", "OEM102", "Period", "PlayPause", 
    "Power", "PrevTrack", "AltRight", "BracketRight", "ControlRight", "RMenu", "ShiftRight", "MetaRight", "Semicolon", "Slash", "Sleep", "Stop", "Subtract", 
    "Sysrq", "Tab", "Underline", "Unlabeled", "AudioVolumeDown", "AudioVolumeUp", "Wake", "WebBack", "WebFavorites", "WebForward", "WebHome", 
    "WebRefresh", "WebSearch", "WebStop", "Yen"];

#[cfg(target_arch="wasm32")]
fn generate_key_names() -> HashMap<String, usize> {
    KEY_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| (String::from(*name), index))
        .collect()    
}