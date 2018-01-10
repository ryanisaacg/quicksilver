#![allow(missing_docs)]
pub enum Key {
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, 
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, 
    F13, F14, F15, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right, 
    Down, Back, Return, Space, Compose, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, 
    Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, 
    Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, 
    LMenu, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward, 
    NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause, 
    Power, PrevTrack, RAlt, RBracket, RControl, RMenu, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, 
    Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, 
    WebRefresh, WebSearch, WebStop, Yen,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate glutin;

    #[test]
    fn key_constants_match() {
        assert_eq!(Key::Key1 as u32, glutin::VirtualKeyCode::Key1 as u32);
        assert_eq!(Key::Key2 as u32, glutin::VirtualKeyCode::Key2 as u32);
        assert_eq!(Key::Key3 as u32, glutin::VirtualKeyCode::Key3 as u32);
        assert_eq!(Key::Key4 as u32, glutin::VirtualKeyCode::Key4 as u32);
        assert_eq!(Key::Key5 as u32, glutin::VirtualKeyCode::Key5 as u32);
        assert_eq!(Key::Key6 as u32, glutin::VirtualKeyCode::Key6 as u32);
        assert_eq!(Key::Key7 as u32, glutin::VirtualKeyCode::Key7 as u32);
        assert_eq!(Key::Key8 as u32, glutin::VirtualKeyCode::Key8 as u32);
        assert_eq!(Key::Key9 as u32, glutin::VirtualKeyCode::Key9 as u32);
        assert_eq!(Key::Key0 as u32, glutin::VirtualKeyCode::Key0 as u32);
        assert_eq!(Key::A as u32, glutin::VirtualKeyCode::A as u32);
        assert_eq!(Key::B as u32, glutin::VirtualKeyCode::B as u32);
        assert_eq!(Key::C as u32, glutin::VirtualKeyCode::C as u32);
        assert_eq!(Key::D as u32, glutin::VirtualKeyCode::D as u32);
        assert_eq!(Key::E as u32, glutin::VirtualKeyCode::E as u32);
        assert_eq!(Key::F as u32, glutin::VirtualKeyCode::F as u32);
        assert_eq!(Key::G as u32, glutin::VirtualKeyCode::G as u32);
        assert_eq!(Key::H as u32, glutin::VirtualKeyCode::H as u32);
        assert_eq!(Key::I as u32, glutin::VirtualKeyCode::I as u32);
        assert_eq!(Key::J as u32, glutin::VirtualKeyCode::J as u32);
        assert_eq!(Key::K as u32, glutin::VirtualKeyCode::K as u32);
        assert_eq!(Key::L as u32, glutin::VirtualKeyCode::L as u32);
        assert_eq!(Key::M as u32, glutin::VirtualKeyCode::M as u32);
        assert_eq!(Key::N as u32, glutin::VirtualKeyCode::N as u32);
        assert_eq!(Key::O as u32, glutin::VirtualKeyCode::O as u32);
        assert_eq!(Key::P as u32, glutin::VirtualKeyCode::P as u32);
        assert_eq!(Key::Q as u32, glutin::VirtualKeyCode::Q as u32);
        assert_eq!(Key::R as u32, glutin::VirtualKeyCode::R as u32);
        assert_eq!(Key::S as u32, glutin::VirtualKeyCode::S as u32);
        assert_eq!(Key::T as u32, glutin::VirtualKeyCode::T as u32);
        assert_eq!(Key::U as u32, glutin::VirtualKeyCode::U as u32);
        assert_eq!(Key::V as u32, glutin::VirtualKeyCode::V as u32);
        assert_eq!(Key::W as u32, glutin::VirtualKeyCode::W as u32);
        assert_eq!(Key::X as u32, glutin::VirtualKeyCode::X as u32);
        assert_eq!(Key::Y as u32, glutin::VirtualKeyCode::Y as u32);
        assert_eq!(Key::Z as u32, glutin::VirtualKeyCode::Z as u32);
        assert_eq!(Key::Escape as u32, glutin::VirtualKeyCode::Escape as u32);
        assert_eq!(Key::F1 as u32, glutin::VirtualKeyCode::F1 as u32);
        assert_eq!(Key::F2 as u32, glutin::VirtualKeyCode::F2 as u32);
        assert_eq!(Key::F3 as u32, glutin::VirtualKeyCode::F3 as u32);
        assert_eq!(Key::F4 as u32, glutin::VirtualKeyCode::F4 as u32);
        assert_eq!(Key::F5 as u32, glutin::VirtualKeyCode::F5 as u32);
        assert_eq!(Key::F6 as u32, glutin::VirtualKeyCode::F6 as u32);
        assert_eq!(Key::F7 as u32, glutin::VirtualKeyCode::F7 as u32);
        assert_eq!(Key::F8 as u32, glutin::VirtualKeyCode::F8 as u32);
        assert_eq!(Key::F9 as u32, glutin::VirtualKeyCode::F9 as u32);
        assert_eq!(Key::F10 as u32, glutin::VirtualKeyCode::F10 as u32);
        assert_eq!(Key::F11 as u32, glutin::VirtualKeyCode::F11 as u32);
        assert_eq!(Key::F12 as u32, glutin::VirtualKeyCode::F12 as u32);
        assert_eq!(Key::F13 as u32, glutin::VirtualKeyCode::F13 as u32);
        assert_eq!(Key::F14 as u32, glutin::VirtualKeyCode::F14 as u32);
        assert_eq!(Key::F15 as u32, glutin::VirtualKeyCode::F15 as u32);
        assert_eq!(Key::Snapshot as u32, glutin::VirtualKeyCode::Snapshot as u32);
        assert_eq!(Key::Scroll as u32, glutin::VirtualKeyCode::Scroll as u32);
        assert_eq!(Key::Pause as u32, glutin::VirtualKeyCode::Pause as u32);
        assert_eq!(Key::Insert as u32, glutin::VirtualKeyCode::Insert as u32);
        assert_eq!(Key::Home as u32, glutin::VirtualKeyCode::Home as u32);
        assert_eq!(Key::Delete as u32, glutin::VirtualKeyCode::Delete as u32);
        assert_eq!(Key::End as u32, glutin::VirtualKeyCode::End as u32);
        assert_eq!(Key::PageDown as u32, glutin::VirtualKeyCode::PageDown as u32);
        assert_eq!(Key::PageUp as u32, glutin::VirtualKeyCode::PageUp as u32);
        assert_eq!(Key::Left as u32, glutin::VirtualKeyCode::Left as u32);
        assert_eq!(Key::Up as u32, glutin::VirtualKeyCode::Up as u32);
        assert_eq!(Key::Right as u32, glutin::VirtualKeyCode::Right as u32);
        assert_eq!(Key::Down as u32, glutin::VirtualKeyCode::Down as u32);
        assert_eq!(Key::Back as u32, glutin::VirtualKeyCode::Back as u32);
        assert_eq!(Key::Return as u32, glutin::VirtualKeyCode::Return as u32);
        assert_eq!(Key::Space as u32, glutin::VirtualKeyCode::Space as u32);
        assert_eq!(Key::Compose as u32, glutin::VirtualKeyCode::Compose as u32);
        assert_eq!(Key::Numlock as u32, glutin::VirtualKeyCode::Numlock as u32);
        assert_eq!(Key::Numpad0 as u32, glutin::VirtualKeyCode::Numpad0 as u32);
        assert_eq!(Key::Numpad1 as u32, glutin::VirtualKeyCode::Numpad1 as u32);
        assert_eq!(Key::Numpad2 as u32, glutin::VirtualKeyCode::Numpad2 as u32);
        assert_eq!(Key::Numpad3 as u32, glutin::VirtualKeyCode::Numpad3 as u32);
        assert_eq!(Key::Numpad4 as u32, glutin::VirtualKeyCode::Numpad4 as u32);
        assert_eq!(Key::Numpad5 as u32, glutin::VirtualKeyCode::Numpad5 as u32);
        assert_eq!(Key::Numpad6 as u32, glutin::VirtualKeyCode::Numpad6 as u32);
        assert_eq!(Key::Numpad7 as u32, glutin::VirtualKeyCode::Numpad7 as u32);
        assert_eq!(Key::Numpad8 as u32, glutin::VirtualKeyCode::Numpad8 as u32);
        assert_eq!(Key::Numpad9 as u32, glutin::VirtualKeyCode::Numpad9 as u32);
        assert_eq!(Key::AbntC1 as u32, glutin::VirtualKeyCode::AbntC1 as u32);
        assert_eq!(Key::AbntC2 as u32, glutin::VirtualKeyCode::AbntC2 as u32);
        assert_eq!(Key::Add as u32, glutin::VirtualKeyCode::Add as u32);
        assert_eq!(Key::Apostrophe as u32, glutin::VirtualKeyCode::Apostrophe as u32);
        assert_eq!(Key::Apps as u32, glutin::VirtualKeyCode::Apps as u32);
        assert_eq!(Key::At as u32, glutin::VirtualKeyCode::At as u32);
        assert_eq!(Key::Ax as u32, glutin::VirtualKeyCode::Ax as u32);
        assert_eq!(Key::Backslash as u32, glutin::VirtualKeyCode::Backslash as u32);
        assert_eq!(Key::Calculator as u32, glutin::VirtualKeyCode::Calculator as u32);
        assert_eq!(Key::Capital as u32, glutin::VirtualKeyCode::Capital as u32);
        assert_eq!(Key::Colon as u32, glutin::VirtualKeyCode::Colon as u32);
        assert_eq!(Key::Comma as u32, glutin::VirtualKeyCode::Comma as u32);
        assert_eq!(Key::Convert as u32, glutin::VirtualKeyCode::Convert as u32);
        assert_eq!(Key::Decimal as u32, glutin::VirtualKeyCode::Decimal as u32);
        assert_eq!(Key::Divide as u32, glutin::VirtualKeyCode::Divide as u32);
        assert_eq!(Key::Equals as u32, glutin::VirtualKeyCode::Equals as u32);
        assert_eq!(Key::Grave as u32, glutin::VirtualKeyCode::Grave as u32);
        assert_eq!(Key::Kana as u32, glutin::VirtualKeyCode::Kana as u32);
        assert_eq!(Key::Kanji as u32, glutin::VirtualKeyCode::Kanji as u32);
        assert_eq!(Key::LAlt as u32, glutin::VirtualKeyCode::LAlt as u32);
        assert_eq!(Key::LBracket as u32, glutin::VirtualKeyCode::LBracket as u32);
        assert_eq!(Key::LControl as u32, glutin::VirtualKeyCode::LControl as u32);
        assert_eq!(Key::LMenu as u32, glutin::VirtualKeyCode::LMenu as u32);
        assert_eq!(Key::LShift as u32, glutin::VirtualKeyCode::LShift as u32);
        assert_eq!(Key::LWin as u32, glutin::VirtualKeyCode::LWin as u32);
        assert_eq!(Key::Mail as u32, glutin::VirtualKeyCode::Mail as u32);
        assert_eq!(Key::MediaSelect as u32, glutin::VirtualKeyCode::MediaSelect as u32);
        assert_eq!(Key::MediaStop as u32, glutin::VirtualKeyCode::MediaStop as u32);
        assert_eq!(Key::Minus as u32, glutin::VirtualKeyCode::Minus as u32);
        assert_eq!(Key::Multiply as u32, glutin::VirtualKeyCode::Multiply as u32);
        assert_eq!(Key::Mute as u32, glutin::VirtualKeyCode::Mute as u32);
        assert_eq!(Key::MyComputer as u32, glutin::VirtualKeyCode::MyComputer as u32);
        assert_eq!(Key::NavigateForward as u32, glutin::VirtualKeyCode::NavigateForward as u32);
        assert_eq!(Key::NavigateBackward as u32, glutin::VirtualKeyCode::NavigateBackward as u32);
        assert_eq!(Key::NextTrack as u32, glutin::VirtualKeyCode::NextTrack as u32);
        assert_eq!(Key::NoConvert as u32, glutin::VirtualKeyCode::NoConvert as u32);
        assert_eq!(Key::NumpadComma as u32, glutin::VirtualKeyCode::NumpadComma as u32);
        assert_eq!(Key::NumpadEnter as u32, glutin::VirtualKeyCode::NumpadEnter as u32);
        assert_eq!(Key::NumpadEquals as u32, glutin::VirtualKeyCode::NumpadEquals as u32);
        assert_eq!(Key::OEM102 as u32, glutin::VirtualKeyCode::OEM102 as u32);
        assert_eq!(Key::Period as u32, glutin::VirtualKeyCode::Period as u32);
        assert_eq!(Key::PlayPause as u32, glutin::VirtualKeyCode::PlayPause as u32);
        assert_eq!(Key::Power as u32, glutin::VirtualKeyCode::Power as u32);
        assert_eq!(Key::PrevTrack as u32, glutin::VirtualKeyCode::PrevTrack as u32);
        assert_eq!(Key::RAlt as u32, glutin::VirtualKeyCode::RAlt as u32);
        assert_eq!(Key::RBracket as u32, glutin::VirtualKeyCode::RBracket as u32);
        assert_eq!(Key::RControl as u32, glutin::VirtualKeyCode::RControl as u32);
        assert_eq!(Key::RMenu as u32, glutin::VirtualKeyCode::RMenu as u32);
        assert_eq!(Key::RShift as u32, glutin::VirtualKeyCode::RShift as u32);
        assert_eq!(Key::RWin as u32, glutin::VirtualKeyCode::RWin as u32);
        assert_eq!(Key::Semicolon as u32, glutin::VirtualKeyCode::Semicolon as u32);
        assert_eq!(Key::Slash as u32, glutin::VirtualKeyCode::Slash as u32);
        assert_eq!(Key::Sleep as u32, glutin::VirtualKeyCode::Sleep as u32);
        assert_eq!(Key::Stop as u32, glutin::VirtualKeyCode::Stop as u32);
        assert_eq!(Key::Subtract as u32, glutin::VirtualKeyCode::Subtract as u32);
        assert_eq!(Key::Sysrq as u32, glutin::VirtualKeyCode::Sysrq as u32);
        assert_eq!(Key::Tab as u32, glutin::VirtualKeyCode::Tab as u32);
        assert_eq!(Key::Underline as u32, glutin::VirtualKeyCode::Underline as u32);
        assert_eq!(Key::Unlabeled as u32, glutin::VirtualKeyCode::Unlabeled as u32);
        assert_eq!(Key::VolumeDown as u32, glutin::VirtualKeyCode::VolumeDown as u32);
        assert_eq!(Key::VolumeUp as u32, glutin::VirtualKeyCode::VolumeUp as u32);
        assert_eq!(Key::Wake as u32, glutin::VirtualKeyCode::Wake as u32);
        assert_eq!(Key::WebBack as u32, glutin::VirtualKeyCode::WebBack as u32);
        assert_eq!(Key::WebFavorites as u32, glutin::VirtualKeyCode::WebFavorites as u32);
        assert_eq!(Key::WebForward as u32, glutin::VirtualKeyCode::WebForward as u32);
        assert_eq!(Key::WebHome as u32, glutin::VirtualKeyCode::WebHome as u32);
        assert_eq!(Key::WebRefresh as u32, glutin::VirtualKeyCode::WebRefresh as u32);
        assert_eq!(Key::WebSearch as u32, glutin::VirtualKeyCode::WebSearch as u32);
        assert_eq!(Key::WebStop as u32, glutin::VirtualKeyCode::WebStop as u32);
        assert_eq!(Key::Yen as u32, glutin::VirtualKeyCode::Yen as u32);
    }
}
