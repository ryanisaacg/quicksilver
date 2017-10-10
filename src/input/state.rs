#[derive(Clone, Copy)]
pub enum State {
    Pressed,
    Held,
    Released,
    NotPressed
}

impl State {
    pub fn is_down(&self) -> bool {
        match *self {
            State::Pressed => true,
            State::Held => true,
            State::Released => false,
            State::NotPressed => false,
        }
    }
}
