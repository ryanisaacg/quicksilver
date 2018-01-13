#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The current state of a button
pub enum ButtonState {
    /// The button was activated this frame
    Pressed,
    /// The button is active but was not activated this frame
    Held,
    /// The button was released this frame
    Released,
    /// The button is not active but was not released this frame
    NotPressed,
}

impl ButtonState {
    pub(crate) fn update(&self, new: ButtonState) -> ButtonState {
        match (self.is_down(), new.is_down()) {
            (false, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Released,
            _ => self.clone()
        }
    }

    /// Determine if the button is either Pressed or Held
    pub fn is_down(&self) -> bool {
        match *self {
            ButtonState::Pressed => true,
            ButtonState::Held => true,
            ButtonState::Released => false,
            ButtonState::NotPressed => false,
        }
    }

    /// Convert the button from a temporary state to a permanent state
    ///
    /// Pressed states become Held, Released states become NotPressed
    pub fn clear_temporary(&self) -> ButtonState {
        if self.is_down() {
            ButtonState::Held
        } else {
            ButtonState::NotPressed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_temporary() {
        for button in [ButtonState::Pressed, ButtonState::Held, ButtonState::Released, ButtonState::NotPressed].iter() {
            assert_eq!(button.is_down(), button.clear_temporary().is_down());
        }
    }
}

