use graphics::Window;
use input::{Button, ButtonState};

/// A trait to see if a type can examine a mouse and keyboard and return a bool
pub trait InputCheckable: Sized {
    /// Check against the mouse and keyboard
    fn satisfied(&self, window: &Window) -> bool;

    /// Boolean and with another input check
    fn and<Other: InputCheckable>(self, other: Other) -> And<Self, Other> {
        And(self, other)
    }
   
    /// Boolean or with another input check
    fn or<Other: InputCheckable>(self, other: Other) -> Or<Self, Other> {
        Or(self, other)
    }
   
    /// Boolean xor with another input check
    fn xor<Other: InputCheckable>(self, other: Other) -> Xor<Self, Other> {
        Xor(self, other)
    }
   
    /// Boolean not with this input check
    fn not(self) -> Not<Self> {
        Not(self)
    }
}

//TODO: This part of the API needs to be better

impl InputCheckable for Button {
    fn satisfied(&self, window: &Window) -> bool {
        (*self, ButtonState::Pressed).or((*self, ButtonState::Held)).satisfied(window)
    }
}

impl<'a> InputCheckable for &'a Button {
    fn satisfied(&self, window: &Window) -> bool {
        (*self).satisfied(window)
    }
}

impl InputCheckable for (Button, ButtonState) {
    fn satisfied(&self, window: &Window) -> bool {
        let state = match self.0 {
            Button::GamepadButton((Some(id), button)) => {
                let pad = window.gamepads().iter()
                    .find(|pad| pad.id() == id);
                if let Some(pad) = pad {
                    pad[button]
                } else {
                    return false
                }
            }
            Button::GamepadButton((None, button)) => {
                return window.gamepads().iter()
                    .any(|pad| pad[button] == self.1)
            }
            Button::GamepadAxis((Some(id), axis, min, max)) => {
                let pad = window.gamepads().iter()
                    .find(|pad| pad.id() == id);
                match pad {
                    Some(pad) => return pad[axis] >= min && pad[axis] <= max,
                    None => return false
                }
            }
            Button::GamepadAxis((None, axis, min, max)) => {
                return window.gamepads().iter()
                    .any(|pad| pad[axis] > min && pad[axis] < max)
            }
            Button::Mouse(button) => window.mouse()[button],
            Button::Keyboard(button) => window.keyboard()[button]
        };
        state == self.1
    }
}


/// Check if any item is satisfied
pub fn any<A, I>(list: I) -> Any<A> where A: InputCheckable, I: IntoIterator<Item = A> {
    Any(list.into_iter().collect())
}

/// Check if all items are satisified
pub fn all<A, I>(list: I) -> All<A> where A: InputCheckable, I: IntoIterator<Item = A> {
    All(list.into_iter().collect())
}

#[allow(missing_docs)]
pub struct Any<A: InputCheckable>(Vec<A>);

impl<A: InputCheckable> InputCheckable for Any<A> {
    fn satisfied(&self, window: &Window) -> bool {
        self.0.iter().any(|x| x.satisfied(window))
    }
}

#[allow(missing_docs)]
pub struct All<A: InputCheckable>(Vec<A>);

impl<A: InputCheckable> InputCheckable for All<A> {
    fn satisfied(&self, window: &Window) -> bool {
        self.0.iter().all(|x| x.satisfied(window))
    }
}

#[allow(missing_docs)]
pub struct And<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for And<A, B> {
    fn satisfied(&self, window: &Window) -> bool {
        self.0.satisfied(window) && self.1.satisfied(window)
    }
}

#[allow(missing_docs)]
pub struct Or<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for Or<A, B> {
    fn satisfied(&self, window: &Window) -> bool {
        self.0.satisfied(window) || self.1.satisfied(window)
    }
}

#[allow(missing_docs)]
pub struct Xor<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for Xor<A, B> {
    fn satisfied(&self, window: &Window) -> bool {
        self.0.satisfied(window) != self.1.satisfied(window)
    }
}

#[allow(missing_docs)]
pub struct Not<A:InputCheckable>(A);

impl<A: InputCheckable> InputCheckable for Not<A> {
    fn satisfied(&self, window: &Window) -> bool {
        !self.0.satisfied(window)
    }
}

/// An InputCheckable that always returns True
pub struct True;

impl InputCheckable for True {
    fn satisfied(&self, _window: &Window) -> bool {
        true
    }
}

/// An InputCheckable that always returns False
pub struct False;

impl InputCheckable for False {
    fn satisfied(&self, _window: &Window) -> bool {
       false 
    }
}
