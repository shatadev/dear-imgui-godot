use godot::classes::{InputEvent, InputEventKey, InputEventMouseButton, InputEventMouseMotion};
use godot::global::{Key as GKey, MouseButton};
use godot::prelude::*;

use imgui::{Io, Key as IKey, MouseButton as IMouse};

pub fn is_mouse_event(event: &Gd<InputEvent>) -> bool {
    event.clone().try_cast::<InputEventMouseButton>().is_ok()
        || event.clone().try_cast::<InputEventMouseMotion>().is_ok()
}

pub fn is_keyboard_event(event: &Gd<InputEvent>) -> bool {
    event.clone().try_cast::<InputEventKey>().is_ok()
}

pub fn feed_event(io: &mut Io, event: &Gd<InputEvent>) {
    if let Ok(mm) = event.clone().try_cast::<InputEventMouseMotion>() {
        let p = mm.get_position();
        io.add_mouse_pos_event([p.x, p.y]);
    } else if let Ok(mb) = event.clone().try_cast::<InputEventMouseButton>() {
        feed_mouse_button(io, &mb);
    } else if let Ok(k) = event.clone().try_cast::<InputEventKey>() {
        feed_key(io, &k);
    }
}

fn feed_mouse_button(io: &mut Io, mb: &Gd<InputEventMouseButton>) {
    let pressed = mb.is_pressed();
    let factor = {
        let f = mb.get_factor();
        if f <= 0.0 {
            1.0
        } else {
            f
        }
    };
    let b = mb.get_button_index();
    if b == MouseButton::LEFT {
        io.add_mouse_button_event(IMouse::Left, pressed);
    } else if b == MouseButton::RIGHT {
        io.add_mouse_button_event(IMouse::Right, pressed);
    } else if b == MouseButton::MIDDLE {
        io.add_mouse_button_event(IMouse::Middle, pressed);
    } else if b == MouseButton::XBUTTON1 {
        io.add_mouse_button_event(IMouse::Extra1, pressed);
    } else if b == MouseButton::XBUTTON2 {
        io.add_mouse_button_event(IMouse::Extra2, pressed);
    } else if b == MouseButton::WHEEL_UP {
        io.add_mouse_wheel_event([0.0, factor]);
    } else if b == MouseButton::WHEEL_DOWN {
        io.add_mouse_wheel_event([0.0, -factor]);
    } else if b == MouseButton::WHEEL_LEFT {
        io.add_mouse_wheel_event([-factor, 0.0]);
    } else if b == MouseButton::WHEEL_RIGHT {
        io.add_mouse_wheel_event([factor, 0.0]);
    }
}

fn feed_key(io: &mut Io, k: &Gd<InputEventKey>) {
    let pressed = k.is_pressed();

    io.add_key_event(IKey::ModCtrl, k.is_ctrl_pressed());
    io.add_key_event(IKey::ModShift, k.is_shift_pressed());
    io.add_key_event(IKey::ModAlt, k.is_alt_pressed());
    io.add_key_event(IKey::ModSuper, k.is_meta_pressed());

    if let Some(key) = map_key(k.get_keycode()) {
        io.add_key_event(key, pressed);
    }

    if pressed {
        let u = k.get_unicode();
        if u >= 0x20 {
            if let Some(c) = char::from_u32(u as u32) {
                io.add_input_character(c);
            }
        }
    }
}

fn map_key(k: GKey) -> Option<IKey> {
    let table: &[(GKey, IKey)] = &[
        (GKey::TAB, IKey::Tab),
        (GKey::LEFT, IKey::LeftArrow),
        (GKey::RIGHT, IKey::RightArrow),
        (GKey::UP, IKey::UpArrow),
        (GKey::DOWN, IKey::DownArrow),
        (GKey::PAGEUP, IKey::PageUp),
        (GKey::PAGEDOWN, IKey::PageDown),
        (GKey::HOME, IKey::Home),
        (GKey::END, IKey::End),
        (GKey::INSERT, IKey::Insert),
        (GKey::DELETE, IKey::Delete),
        (GKey::BACKSPACE, IKey::Backspace),
        (GKey::SPACE, IKey::Space),
        (GKey::ENTER, IKey::Enter),
        (GKey::ESCAPE, IKey::Escape),
        (GKey::CTRL, IKey::LeftCtrl),
        (GKey::SHIFT, IKey::LeftShift),
        (GKey::ALT, IKey::LeftAlt),
        (GKey::META, IKey::LeftSuper),
        (GKey::MENU, IKey::Menu),
        (GKey::CAPSLOCK, IKey::CapsLock),
        (GKey::SCROLLLOCK, IKey::ScrollLock),
        (GKey::NUMLOCK, IKey::NumLock),
        (GKey::PRINT, IKey::PrintScreen),
        (GKey::PAUSE, IKey::Pause),
        (GKey::APOSTROPHE, IKey::Apostrophe),
        (GKey::COMMA, IKey::Comma),
        (GKey::MINUS, IKey::Minus),
        (GKey::PERIOD, IKey::Period),
        (GKey::SLASH, IKey::Slash),
        (GKey::SEMICOLON, IKey::Semicolon),
        (GKey::EQUAL, IKey::Equal),
        (GKey::BRACKETLEFT, IKey::LeftBracket),
        (GKey::BACKSLASH, IKey::Backslash),
        (GKey::BRACKETRIGHT, IKey::RightBracket),
        (GKey::QUOTELEFT, IKey::GraveAccent),
        (GKey::KEY_0, IKey::Alpha0),
        (GKey::KEY_1, IKey::Alpha1),
        (GKey::KEY_2, IKey::Alpha2),
        (GKey::KEY_3, IKey::Alpha3),
        (GKey::KEY_4, IKey::Alpha4),
        (GKey::KEY_5, IKey::Alpha5),
        (GKey::KEY_6, IKey::Alpha6),
        (GKey::KEY_7, IKey::Alpha7),
        (GKey::KEY_8, IKey::Alpha8),
        (GKey::KEY_9, IKey::Alpha9),
        (GKey::A, IKey::A),
        (GKey::B, IKey::B),
        (GKey::C, IKey::C),
        (GKey::D, IKey::D),
        (GKey::E, IKey::E),
        (GKey::F, IKey::F),
        (GKey::G, IKey::G),
        (GKey::H, IKey::H),
        (GKey::I, IKey::I),
        (GKey::J, IKey::J),
        (GKey::K, IKey::K),
        (GKey::L, IKey::L),
        (GKey::M, IKey::M),
        (GKey::N, IKey::N),
        (GKey::O, IKey::O),
        (GKey::P, IKey::P),
        (GKey::Q, IKey::Q),
        (GKey::R, IKey::R),
        (GKey::S, IKey::S),
        (GKey::T, IKey::T),
        (GKey::U, IKey::U),
        (GKey::V, IKey::V),
        (GKey::W, IKey::W),
        (GKey::X, IKey::X),
        (GKey::Y, IKey::Y),
        (GKey::Z, IKey::Z),
        (GKey::F1, IKey::F1),
        (GKey::F2, IKey::F2),
        (GKey::F3, IKey::F3),
        (GKey::F4, IKey::F4),
        (GKey::F5, IKey::F5),
        (GKey::F6, IKey::F6),
        (GKey::F7, IKey::F7),
        (GKey::F8, IKey::F8),
        (GKey::F9, IKey::F9),
        (GKey::F10, IKey::F10),
        (GKey::F11, IKey::F11),
        (GKey::F12, IKey::F12),
        (GKey::KP_0, IKey::Keypad0),
        (GKey::KP_1, IKey::Keypad1),
        (GKey::KP_2, IKey::Keypad2),
        (GKey::KP_3, IKey::Keypad3),
        (GKey::KP_4, IKey::Keypad4),
        (GKey::KP_5, IKey::Keypad5),
        (GKey::KP_6, IKey::Keypad6),
        (GKey::KP_7, IKey::Keypad7),
        (GKey::KP_8, IKey::Keypad8),
        (GKey::KP_9, IKey::Keypad9),
        (GKey::KP_PERIOD, IKey::KeypadDecimal),
        (GKey::KP_DIVIDE, IKey::KeypadDivide),
        (GKey::KP_MULTIPLY, IKey::KeypadMultiply),
        (GKey::KP_SUBTRACT, IKey::KeypadSubtract),
        (GKey::KP_ADD, IKey::KeypadAdd),
        (GKey::KP_ENTER, IKey::KeypadEnter),
        (GKey::BACKTAB, IKey::Tab),
        (GKey::HELP, IKey::Insert),
        (GKey::ASTERISK, IKey::KeypadMultiply),
        (GKey::PLUS, IKey::KeypadAdd),
        (GKey::COLON, IKey::Semicolon),
    ];

    table.iter().find(|(g, _)| *g == k).map(|&(_, i)| i)
}
