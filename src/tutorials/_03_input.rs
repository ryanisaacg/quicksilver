//! Now we can draw all manner of colorful geometry, but that's not enough for an interesting
//! application.
//!
//! If we wanted to add keyboard support to our previous example, so that the user can use the left
//! and right arrow keys to move the square back ond forth, it would look like this:
//! ```no_run
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Rectangle, Vector},
//!     graphics::{Background, Color},
//!     input::Key, // We need the Key enum
//!     lifecycle::{State, Window, run}
//! };
//!
//! struct Screen {
//!     position: Vector // We need to store the position as state
//! }
//!
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen {
//!             position: Vector::new(50, 50)
//!         })
//!     }
//!
//!     fn update(&mut self, window: &mut Window) -> Result<()> {
//!         if window.keyboard()[Key::Right].is_down() {
//!             self.position.x += 2.5;
//!         }
//!         if window.keyboard()[Key::Left].is_down() {
//!             self.position.x -= 2.5;
//!         }
//!         Ok(())
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         window.draw(&Rectangle::new(self.position, (100, 200)), Background::Col(Color::RED));
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! Now we have very basic keyboard input controls. Every frame that the right arrow is held down,
//! the box will move 2.5 pixels to the right, and the same for left.
//!
//! The input API generally follows this principal: an input source is indexed by a button enum,
//! and returns a `ButtonState` enum. A button state can be `Pressed`, `Held`, `Released` or
//! `NotPressed`, and a convenience method `is_down` checks if the button is either pressed or
//! held.
//!
//! If we wanted to give the user more freedom, and allow them to use the mouse buttons or gamepad triggers instead of the arrow
//! keys, we could do that fairly easily:
//! ```no_run
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Rectangle, Vector},
//!     graphics::{Background, Color},
//!     input::{GamepadButton, Key, MouseButton}, // We need the mouse and gamepad buttons
//!     lifecycle::{State, Window, run}
//! };
//!
//! struct Screen {
//!     position: Vector // We need to store the position as state
//! }
//!
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen {
//!             position: Vector::new(50, 50)
//!         })
//!     }
//!
//!     fn update(&mut self, window: &mut Window) -> Result<()> {
//!         if window.keyboard()[Key::Right].is_down() ||
//!             window.mouse()[MouseButton::Right].is_down() ||
//!             window.gamepads().iter().any(|pad| pad[GamepadButton::TriggerRight].is_down())
//!             {
//!             self.position.x += 2.5;
//!         }
//!         if window.keyboard()[Key::Left].is_down() ||
//!             window.mouse()[MouseButton::Left].is_down() ||
//!             window.gamepads().iter().any(|pad| pad[GamepadButton::TriggerLeft].is_down())
//!             {
//!             self.position.x -= 2.5;
//!         }
//!         Ok(())
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         window.draw(&Rectangle::new(self.position, (100, 200)), Background::Col(Color::RED));
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! Unlike mice and keyboards, which generally are one-per-system, a machine may have many gamepads
//! connected. More advanced applications may wish to assign specific gamepads to specific
//! functions or specific users, but for our case checking against any gamepad does just fine.
//! The input API generally follows this principal: an input source is indexed by a button enum,
//! and returns a `ButtonState` enum. A button state can be `Pressed`, `Held`, `Released` or
//! `NotPressed`, and a convenience method `is_down` checks if the button is either pressed or
//! held.
//!
//! If we want to only apply an effect once per input submission, we have two options. One is to
//! check if the button state is exactly `Pressed`: that is, the button was not pressed the last
//! update, but now is. The other is to implement the `event` method of State, and listen for a
//! keypress event. To compare, here is an implementation that checks for `Pressed` for up and uses
//! an event for down:
//! ```no_run
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Rectangle, Vector},
//!     graphics::{Background, Color},
//!     input::{ButtonState, GamepadButton, Key, MouseButton}, // We need to match ButtonState
//!     lifecycle::{Event, State, Window, run} // We need to match against Event
//! };
//!
//! struct Screen {
//!     position: Vector // We need to store the position as state
//! }
//!
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen {
//!             position: Vector::new(50, 50)
//!         })
//!     }
//!
//!     fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
//!         if let Event::Key(Key::Down, ButtonState::Pressed) = event {
//!             self.position.y += 10.0;
//!         }
//!         Ok(())
//!     }
//!
//!     fn update(&mut self, window: &mut Window) -> Result<()> {
//!         if window.keyboard()[Key::Right].is_down() ||
//!             window.mouse()[MouseButton::Right].is_down() ||
//!             window.gamepads().iter().any(|pad| pad[GamepadButton::TriggerRight].is_down())
//!             {
//!             self.position.x += 2.5;
//!         }
//!         if window.keyboard()[Key::Left].is_down() ||
//!             window.mouse()[MouseButton::Left].is_down() ||
//!             window.gamepads().iter().any(|pad| pad[GamepadButton::TriggerLeft].is_down())
//!             {
//!             self.position.x -= 2.5;
//!         }
//!         if window.keyboard()[Key::Up] == ButtonState::Pressed {
//!             self.position.y -= 10.0;
//!         }
//!         Ok(())
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         window.draw(&Rectangle::new(self.position, (100, 200)), Background::Col(Color::RED));
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
