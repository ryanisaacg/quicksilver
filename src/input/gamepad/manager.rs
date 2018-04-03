#[cfg(not(any(target_arch="wasm32", target_os="macos")))] 
use gilrs::Gilrs;
use input::Gamepad;

pub(crate) struct GamepadManager {
    #[cfg(not(any(target_arch="wasm32", target_os="macos")))]
    gilrs: Gilrs,
    gamepads: Vec<Gamepad>,
    old: Vec<Gamepad>
}

impl GamepadManager {
    pub(crate) fn new() -> GamepadManager {
        GamepadManager {
            #[cfg(not(any(target_arch="wasm32", target_os="macos")))]
            gilrs: Gilrs::new().unwrap(),
            gamepads: Vec::new(),
            old: Vec::new()
        }
    }

    pub(crate) fn update(&mut self) {
        self.old.clear();
        self.old.append(&mut self.gamepads);

        self.update_platform();
        self.gamepads.sort();
        
        //Integrate old controller state into the new controllers
        let mut i = 0;
        let mut j = 0;

        while i < self.old.len() && j < self.gamepads.len() {
            if self.old[i].id() == self.gamepads[j].id() {
                let gamepad = self.old[i].clone();
                self.gamepads[j].set_previous(gamepad);
                i += 1;
                j += 1;
            } else if self.old[i].id() > self.gamepads[j].id() {
                j += 1;
            } else {
                i += 1;
            }
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn update_platform(&mut self) {
        #[cfg(not(target_os = "macos"))] {
            while let Some(ev) = self.gilrs.next_event() {
                self.gilrs.update(&ev);
            }

            self.gamepads.extend(
                self.gilrs.gamepads().map(|data| Gamepad::new(data)));
        }
    }

    #[cfg(target_arch="wasm32")]
    pub(crate) fn update_platform(&mut self) {
        use ffi::wasm;

        unsafe { 
            wasm::gamepads_update();
            for i in 0..wasm::gamepads_length() {
                self.gamepads.push(Gamepad::new(wasm::gamepads_id(i)));
            }
        }
    }

    pub(crate) fn list(&self) -> &Vec<Gamepad> {
        &self.gamepads
    }
}
