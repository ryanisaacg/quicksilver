#[derive(Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Held,
    Released,
    NotPressed,
}

impl ButtonState {
    pub fn is_down(&self) -> bool {
        match *self {
            ButtonState::Pressed => true,
            ButtonState::Held => true,
            ButtonState::Released => false,
            ButtonState::NotPressed => false,
        }
    }

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

