use super::*;
use crate::window::WindowContents;
use winit::event::{ElementState, WindowEvent};

pub(crate) fn window_event(event: WindowEvent, window: &WindowContents) -> Option<Event> {
    use WindowEvent::*;
    Some(match event {
        Resized(ls) => Event::Resized(ResizedEvent {
            size: ps_to_logical_vec(ls, window.scale()),
        }),
        ScaleFactorChanged { scale_factor, .. } => {
            Event::ScaleFactorChanged(ScaleFactorChangedEvent {
                scale: scale_factor as f32,
            })
        }
        ReceivedCharacter(chr) => Event::ReceivedCharacter(ReceivedCharacterEvent { chr }),
        Focused(focus) => Event::FocusChanged(FocusChangedEvent { focus }),
        KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                },
            ..
        } => Event::KeyboardInput(KeyboardEvent {
            key: key.into(),
            is_down: state == ElementState::Pressed,
        }),
        CursorMoved {
            device_id,
            position,
            ..
        } => Event::PointerMoved(PointerMovedEvent {
            id: PointerId(device_id),
            location: pp_to_logical_vec(position, window.scale()),
        }),
        CursorEntered { device_id, .. } => {
            Event::PointerEntered(PointerEnteredEvent(PointerId(device_id)))
        }
        CursorLeft { device_id, .. } => Event::PointerLeft(PointerLeftEvent(PointerId(device_id))),
        MouseWheel { delta, .. } => Event::ScrollInput(delta.into()),
        MouseInput {
            device_id,
            button,
            state,
            ..
        } => Event::PointerInput(PointerInputEvent {
            id: PointerId(device_id),
            button: button.into(),
            is_down: state == ElementState::Pressed,
        }),
        ModifiersChanged(state) => Event::ModifiersChanged(convert_modifiers(state)),
        _ => return None,
    })
}

#[cfg(feature = "gilrs")]
pub(crate) fn gamepad_event(event: gilrs::Event) -> Option<Event> {
    use gilrs::ev::EventType::*;
    let gilrs::Event { id, event, .. } = event;
    let id = GamepadId(id);
    Some(match event {
        ButtonPressed(btn, _) => Event::GamepadButton(GamepadButtonEvent {
            id,
            button: convert_gilrs_button(btn)?,
            is_down: true,
            is_repeat: false,
        }),
        ButtonRepeated(btn, _) => Event::GamepadButton(GamepadButtonEvent {
            id,
            button: convert_gilrs_button(btn)?,
            is_down: true,
            is_repeat: true,
        }),
        ButtonReleased(btn, _) => Event::GamepadButton(GamepadButtonEvent {
            id,
            button: convert_gilrs_button(btn)?,
            is_down: false,
            is_repeat: false,
        }),
        AxisChanged(axis, value, _) => Event::GamepadAxis(GamepadAxisEvent {
            id,
            axis: convert_gilrs_axis(axis)?,
            value,
        }),
        Connected => Event::GamepadConnected(GamepadConnectedEvent(id)),
        Disconnected => Event::GamepadDisconnected(GamepadDisconnectedEvent(id)),
        ButtonChanged(_, _, _) | Dropped => return None,
    })
}

fn convert_modifiers(modifiers: winit::event::ModifiersState) -> ModifiersChangedEvent {
    ModifiersChangedEvent {
        shift: modifiers.shift(),
        ctrl: modifiers.ctrl(),
        alt: modifiers.alt(),
        logo: modifiers.logo(),
    }
}

fn ps_to_logical_vec<P: winit::dpi::Pixel>(
    ls: winit::dpi::PhysicalSize<P>,
    scale: f32,
) -> Vector2<f32> {
    Vector2 {
        x: ls.width.cast::<f32>() / scale,
        y: ls.height.cast::<f32>() / scale,
    }
}

fn pp_to_logical_vec<P: winit::dpi::Pixel>(
    ls: winit::dpi::PhysicalPosition<P>,
    scale: f32,
) -> Vector2<f32> {
    Vector2 {
        x: ls.x.cast::<f32>() / scale,
        y: ls.y.cast::<f32>() / scale,
    }
}
