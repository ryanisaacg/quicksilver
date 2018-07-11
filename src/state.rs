use {Result, graphics::{Window, WindowBuilder}, input::Event};
#[cfg(target_arch = "wasm32")]
use { geom::Vector,
     input::{ButtonState, MouseButton, KEY_LIST, LINES_TO_PIXELS},
     std::{cell::{RefCell, RefMut}, collections::HashMap, rc::Rc},
     stdweb::{Value, unstable::TryInto,
              web::{document, window, IEventTarget,  IWindowOrWorker,
                    event::{BlurEvent, ConcreteEvent, FocusEvent, GamepadConnectedEvent,
                            GamepadDisconnectedEvent, IGamepadEvent, IKeyboardEvent,
                            IMouseEvent, KeyDownEvent, KeyUpEvent,
                            MouseButton as WebMouseButton, MouseDownEvent, MouseMoveEvent,
                            MouseOutEvent, MouseOverEvent, MouseUpEvent}}}};

/// The structure responsible for managing the game loop state
pub trait State: 'static {
    /// Create the state given the window and canvas
    fn new() -> Result<Self>
    where
        Self: Sized;
    /// Tick the State forward one frame
    ///
    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS.
    ///
    /// By default it does nothing
    fn update(&mut self, &mut Window) -> Result<()> {
        Ok(())
    }
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, &Event, &mut Window) -> Result<()> {
        Ok(())
    }
    /// Draw the state to the screen
    ///
    /// Will happen as often as possible, only limited by vysnc
    ///
    /// By default it draws a black screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        use graphics::Color;
        window.clear(Color::BLACK)?;
        window.present()?;
        Ok(())
    }
}

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
pub fn run<T: State>(window: WindowBuilder) -> Result<()> {
    run_impl::<T>(window)
}

struct Application<T: State> {
    state: T,
    window: Window,
    event_buffer: Vec<Event>,
}

impl<T: State> Application<T> {
    fn update(&mut self) -> Result<()> {
        self.state.update(&mut self.window)
    }

    fn draw(&mut self) -> Result<()> {
        self.state.draw(&mut self.window)
    }

    fn process_events(&mut self) -> Result<()> {
        self.window.update_gamepads(&mut self.event_buffer);
        for i in 0..self.event_buffer.len() {
            self.window.process_event(&self.event_buffer[i]);
            self.state.event(&self.event_buffer[i], &mut self.window)?;
        }
        self.event_buffer.clear();
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_impl<T: State>(window: WindowBuilder) -> Result<()> {
    use input::EventProvider;
    let (window, events_loop) = window.build()?;
    let mut events = EventProvider::new(events_loop);
    let event_buffer = Vec::new();
    let state = T::new()?;
    let mut app = Application {
        window,
        state,
        event_buffer,
    };
    use std::time::Duration;
    #[cfg(feature = "sounds")]
    {
        use sound::Sound;
        Sound::initialize();
    }
    let mut timer = ::Timer::new();
    let mut running = true;
    while running {
        running = events.generate_events(&mut app.window, &mut app.event_buffer);
        app.process_events()?;
        timer.tick(|| -> Result<Duration> {
            app.update()?;
            app.window.clear_temporary_states();
            Ok(Duration::from_millis(16))
        })?;
        app.draw()?;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn run_impl<T: State>(builder: WindowBuilder) -> Result<()> {
    let app = Rc::new(RefCell::new(Application {
        window: builder.build()?,
        state: T::new()?,
        event_buffer: Vec::new(),
    }));

    let document = document();
    let window = window();
    let canvas = ::get_canvas()?;

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
        document.getElementById("canvas").onwheel = function(e) {
            @{wheel_handler}(e.deltaX, e.deltaY, e.deltaMode);
            e.preventDefault();
        }
    }

    handle_event(&document, &app, |mut app, _: BlurEvent| {
        app.event_buffer.push(Event::Unfocused)
    });
    handle_event(&document, &app, |mut app, _: FocusEvent| {
        app.event_buffer.push(Event::Focused)
    });

    handle_event(&canvas, &app, |mut app, _: MouseOutEvent| {
        app.event_buffer.push(Event::MouseExited)
    });
    handle_event(&canvas, &app, |mut app, _: MouseOverEvent| {
        app.event_buffer.push(Event::MouseEntered)
    });

    handle_event(&canvas, &app, |mut app, event: MouseMoveEvent| {
        let pointer = Vector::new(event.offset_x() as f32, event.offset_y() as f32);
        app.event_buffer.push(Event::MouseMoved(pointer));
    });
    handle_event(&canvas, &app, |mut app, event: MouseUpEvent| {
        let state = ButtonState::Released;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return,
        };
        app.event_buffer.push(Event::MouseButton(button, state));
    });
    handle_event(&canvas, &app, |mut app, event: MouseDownEvent| {
        let state = ButtonState::Pressed;
        let button = match event.button() {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Right => MouseButton::Right,
            _ => return,
        };
        app.event_buffer.push(Event::MouseButton(button, state));
    });

    let key_names = generate_key_names();
    handle_event(&document, &app, move |mut app, event: KeyDownEvent| {
        let state = ButtonState::Pressed;
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event_buffer.push(Event::Key(KEY_LIST[*keycode], state));
        }
    });
    let key_names = generate_key_names();
    handle_event(&document, &app, move |mut app, event: KeyUpEvent| {
        let state = ButtonState::Released;
        if let Some(keycode) = key_names.get(&event.code()) {
            app.event_buffer.push(Event::Key(KEY_LIST[*keycode], state));
        }
    });

    handle_event(
        &window,
        &app,
        move |mut app, event: GamepadConnectedEvent| {
            app.event_buffer
                .push(Event::GamepadConnected(event.gamepad().index() as i32));
        },
    );
    handle_event(
        &window,
        &app,
        move |mut app, event: GamepadDisconnectedEvent| {
            app.event_buffer
                .push(Event::GamepadDisconnected(event.gamepad().index() as i32));
        },
    );

    update(app.clone())?;
    draw(app.clone())
}

//TODO: Add error handling to subsequent calls to unwrap and draw
// Basically, there is no stack on the web because the state is a black box that the runtime calls
// This means the best way to do error handling is probably have either custom unwrap() that does
// not panic (because panicking on wasm is not great) or have the user pass in custom
// error-reporting

#[cfg(target_arch = "wasm32")]
fn update<T: State>(app: Rc<RefCell<Application<T>>>) -> Result<()> {
    app.borrow_mut().process_events()?;
    app.borrow_mut().update()?;
    window().set_timeout(move || update(app).unwrap(), 16);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn draw<T: State>(app: Rc<RefCell<Application<T>>>) -> Result<()> {
    app.borrow_mut().draw()?;
    window().request_animation_frame(move |_| draw(app).unwrap());
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
fn generate_key_names() -> HashMap<String, usize> {
    KEY_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| (String::from(*name), index))
        .collect()
}
