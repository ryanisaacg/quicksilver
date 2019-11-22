use crate::{
    lifecycle::{Event, Window},
    Error, Result,
};

/// Things that have a default conversion from the event type
pub trait FromEvent {
    /// Convert an event to this type
    fn from(event: &Event) -> Self;
}

impl FromEvent for Event {
    fn from(event: &Event) -> Event {
        *event
    }
}

/// The structure responsible for managing the game loop state
pub trait State: 'static {
    /// Custom messages that this state understands how to process
    type Message: FromEvent;

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
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, _message: &Self::Message, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    /// Draw the state to the screen
    ///
    /// Will happen as often as possible, only limited by vysnc and the configured draw rate.
    ///
    /// By default it draws a black screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(crate::graphics::Color::BLACK)?;
        Ok(())
    }
    /// Log and report an error in some way
    ///
    /// There's no way to *recover from* the error at this stage, because error handling should take
    /// place at the error site. However, on the web especially, logging errors can be difficult,
    /// so this provides a way to log other than a panic.
    fn handle_error(error: Error) {
        #[cfg(target_arch = "wasm32")]
        {
            let message = format!("Unhandled error: {:?}", error);
            console!(error, message);
        }
        panic!("Unhandled error: {:?}", error);
    }
}
