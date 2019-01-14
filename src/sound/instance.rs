use super::{Player, PlayState, get_player};

pub struct SoundInstance {
    index: u32,
    generation: u32
}

impl SoundInstance {
    pub fn resume(&self) {
        get_player().resume(self);
    }

    pub fn pause(&self) {
        get_player().pause(self);
    }

    pub fn stop(&self) {
        get_player().stop(self);
    }

    pub fn state(&self) -> PlayState {
        get_player().state(self)
    }
}
