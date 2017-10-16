#[derive(Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Held,
    Released,
    NotPressed
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
}
