use input::{Button, ButtonState, Mouse, Keyboard};

/// A trait to see if a type can examine a mouse and keyboard and return a bool
pub trait InputCheckable: Sized {
    /// Check against the mouse and keyboard
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool;

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

impl InputCheckable for Button {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        match *self {
            Button::Mouse(button) => mouse[button],
            Button::Keyboard(button) => keyboard[button]
        }.is_down()
    }
}

impl InputCheckable for (Button, ButtonState) {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        let state = match self.0 {
            Button::Mouse(button) => mouse[button],
            Button::Keyboard(button) => keyboard[button]
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
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        self.0.iter().any(|x| x.satisfied(mouse, keyboard))
    }
}

#[allow(missing_docs)]
pub struct All<A: InputCheckable>(Vec<A>);

impl<A: InputCheckable> InputCheckable for All<A> {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        self.0.iter().all(|x| x.satisfied(mouse, keyboard))
    }
}

#[allow(missing_docs)]
pub struct And<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for And<A, B> {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        self.0.satisfied(mouse, keyboard) && self.1.satisfied(mouse, keyboard)
    }
}

#[allow(missing_docs)]
pub struct Or<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for Or<A, B> {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        self.0.satisfied(mouse, keyboard) || self.1.satisfied(mouse, keyboard)
    }
}

#[allow(missing_docs)]
pub struct Xor<A: InputCheckable, B: InputCheckable>(A, B);

impl<A: InputCheckable, B: InputCheckable> InputCheckable for Xor<A, B> {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        self.0.satisfied(mouse, keyboard) != self.1.satisfied(mouse, keyboard)
    }
}

#[allow(missing_docs)]
pub struct Not<A:InputCheckable>(A);

impl<A: InputCheckable> InputCheckable for Not<A> {
    fn satisfied(&self, mouse: &Mouse, keyboard: &Keyboard) -> bool {
        !self.0.satisfied(mouse, keyboard)
    }
}

/// An InputCheckable that always returns True
pub struct True;

impl InputCheckable for True {
    fn satisfied(&self, _mouse: &Mouse, _keyboard: &Keyboard) -> bool {
        true
    }
}

/// An InputCheckable that always returns False
pub struct False;

impl InputCheckable for False {
    fn satisfied(&self, _mouse: &Mouse, _keyboard: &Keyboard) -> bool {
       false 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom::Vector;

    fn dummy() -> (Mouse, Keyboard) {
        (Mouse {
            pos: Vector::zero(),
            buttons: [ButtonState::NotPressed; 3],
            wheel: Vector::zero()
        }, Keyboard {
            keys: [ButtonState::NotPressed; 256]
        })
    }

    #[test]
    fn boolean_algebra() {
        let (mouse, keyboard) = dummy();
        assert!(True.satisfied(&mouse, &keyboard));
        assert!(!False.satisfied(&mouse, &keyboard));
        assert!(!True.and(False).satisfied(&mouse, &keyboard));
        assert!(True.or(False).satisfied(&mouse, &keyboard));
    }
}
