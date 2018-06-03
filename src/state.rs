#[cfg(target_arch="wasm32")]
use geom::Vector;
use graphics::{Window, WindowBuilder};
use input::Event;
#[cfg(target_arch="wasm32")]
use std::{
    cell::RefCell,
    rc::Rc
};

/// The structure responsible for managing the game loop state
pub trait State {
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
pub fn run<T: 'static + State>(window: WindowBuilder) {
    run_impl::<T>(window)
}

#[doc(hidden)]
pub struct Application {
    state: Box<State>, 
    window: Window,
    event_buffer: Vec<Event>
}

impl Application {
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
fn run_impl<T: 'static + State>(window: WindowBuilder) {
    use input::EventProvider;
    let (window, events_loop) = window.build();
    let mut events = EventProvider::new(events_loop);
    let event_buffer = Vec::new();
    let state = Box::new(T::new());
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
fn run_impl<T: 'static + State>(window: WindowBuilder) {
    use stdweb::web::{
        document,
        event::{BlurEvent, FocusEvent},
        IEventTarget, 
    };
    let window = window.build();
    let app = Rc::new(RefCell::new(Application { 
        window,
        state: Box::new(T::new()),
        event_buffer: Vec::new()
    }));
    let document = document();
    let event_app = app.clone();
    document.add_event_listener(move |_: BlurEvent| event_app.borrow_mut().event(&Event::Unfocused));
}
