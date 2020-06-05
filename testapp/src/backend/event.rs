use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent};
use widgets::event::{AxisValue, EvData, EvState, Key, KeySide};
use widgets::geometry::{Point, Size};

pub fn translate_event(event: WindowEvent) -> Option<EvData> {
    use widgets::event::EvData::*;

    Some(match event {
        WindowEvent::Resized(PhysicalSize { width: w, height: h }) => Resized(Size { w, h }),
        WindowEvent::Moved(PhysicalPosition { x, y }) => Moved(Point { x, y }),
        WindowEvent::CloseRequested => CloseRequest,
        WindowEvent::Destroyed => Destroyed,
        WindowEvent::DroppedFile(path) => FileDropped(path),
        WindowEvent::ReceivedCharacter(chr) => Character(chr),
        WindowEvent::Focused(is_focused) => Focused(is_focused),
        WindowEvent::KeyboardInput { input, .. } => Keyboard {
            state: translate_state(input.state),
            key: translate_keycode(input.virtual_keycode),
            scancode: input.scancode,
        },
        WindowEvent::CursorMoved {
            position: PhysicalPosition { x, y },
            ..
        } => MouseMoved(AxisValue::Position(Point { x, y })),
        WindowEvent::CursorEntered { .. } => PointerInside(true),
        WindowEvent::CursorLeft { .. } => PointerInside(false),
        WindowEvent::MouseWheel {
            delta: MouseScrollDelta::LineDelta(x, y),
            ..
        } => MouseMoved(AxisValue::Scroll(x, y)),
        WindowEvent::MouseInput { state, button, .. } => MouseButton {
            state: translate_state(state),
            button: translate_button(button),
        },
        WindowEvent::TouchpadPressure { pressure, .. } => MouseMoved(AxisValue::Pressure(pressure as f64)),
        _ => {
            return None;
        }
    })
}

fn translate_state(state: ElementState) -> EvState {
    match state {
        ElementState::Pressed => EvState::Pressed,
        ElementState::Released => EvState::Released,
    }
}

fn translate_button(button: MouseButton) -> widgets::event::MouseButton {
    use widgets::event::MouseButton::*;

    match button {
        MouseButton::Left => Left,
        MouseButton::Middle => Middle,
        MouseButton::Right => Right,
        MouseButton::Other(n) => Other(n),
    }
}

//TODO: not properly tested, need a numpad
fn translate_keycode(keycode: Option<VirtualKeyCode>) -> Key {
    let keycode = if let Some(kc) = keycode { kc } else { return Key::Unk };

    match keycode {
        VirtualKeyCode::Key0 => Key::Num(0, false),
        VirtualKeyCode::Key1 => Key::Num(1, false),
        VirtualKeyCode::Key2 => Key::Num(2, false),
        VirtualKeyCode::Key3 => Key::Num(3, false),
        VirtualKeyCode::Key4 => Key::Num(4, false),
        VirtualKeyCode::Key5 => Key::Num(5, false),
        VirtualKeyCode::Key6 => Key::Num(6, false),
        VirtualKeyCode::Key7 => Key::Num(7, false),
        VirtualKeyCode::Key8 => Key::Num(8, false),
        VirtualKeyCode::Key9 => Key::Num(9, false),
        VirtualKeyCode::A => Key::Letter('a'),
        VirtualKeyCode::B => Key::Letter('b'),
        VirtualKeyCode::C => Key::Letter('c'),
        VirtualKeyCode::D => Key::Letter('d'),
        VirtualKeyCode::E => Key::Letter('e'),
        VirtualKeyCode::F => Key::Letter('f'),
        VirtualKeyCode::G => Key::Letter('g'),
        VirtualKeyCode::H => Key::Letter('h'),
        VirtualKeyCode::I => Key::Letter('i'),
        VirtualKeyCode::J => Key::Letter('j'),
        VirtualKeyCode::K => Key::Letter('k'),
        VirtualKeyCode::L => Key::Letter('l'),
        VirtualKeyCode::M => Key::Letter('m'),
        VirtualKeyCode::N => Key::Letter('n'),
        VirtualKeyCode::O => Key::Letter('o'),
        VirtualKeyCode::P => Key::Letter('p'),
        VirtualKeyCode::Q => Key::Letter('q'),
        VirtualKeyCode::R => Key::Letter('r'),
        VirtualKeyCode::S => Key::Letter('s'),
        VirtualKeyCode::T => Key::Letter('t'),
        VirtualKeyCode::U => Key::Letter('u'),
        VirtualKeyCode::V => Key::Letter('v'),
        VirtualKeyCode::W => Key::Letter('w'),
        VirtualKeyCode::X => Key::Letter('x'),
        VirtualKeyCode::Y => Key::Letter('y'),
        VirtualKeyCode::Z => Key::Letter('z'),
        VirtualKeyCode::Escape => Key::Escape,
        VirtualKeyCode::F1 => Key::Fn(1),
        VirtualKeyCode::F2 => Key::Fn(2),
        VirtualKeyCode::F3 => Key::Fn(3),
        VirtualKeyCode::F4 => Key::Fn(4),
        VirtualKeyCode::F5 => Key::Fn(5),
        VirtualKeyCode::F6 => Key::Fn(6),
        VirtualKeyCode::F7 => Key::Fn(7),
        VirtualKeyCode::F8 => Key::Fn(8),
        VirtualKeyCode::F9 => Key::Fn(9),
        VirtualKeyCode::F10 => Key::Fn(10),
        VirtualKeyCode::F11 => Key::Fn(11),
        VirtualKeyCode::F12 => Key::Fn(12),
        VirtualKeyCode::F13 => Key::Fn(13),
        VirtualKeyCode::F14 => Key::Fn(14),
        VirtualKeyCode::F15 => Key::Fn(15),
        VirtualKeyCode::F16 => Key::Fn(16),
        VirtualKeyCode::F17 => Key::Fn(17),
        VirtualKeyCode::F18 => Key::Fn(18),
        VirtualKeyCode::F19 => Key::Fn(19),
        VirtualKeyCode::F20 => Key::Fn(20),
        VirtualKeyCode::F21 => Key::Fn(21),
        VirtualKeyCode::F22 => Key::Fn(22),
        VirtualKeyCode::F23 => Key::Fn(23),
        VirtualKeyCode::F24 => Key::Fn(24),
        VirtualKeyCode::Snapshot => Key::PrintScr,
        VirtualKeyCode::Scroll => Key::ScrollLock,
        VirtualKeyCode::Pause => Key::Pause,
        VirtualKeyCode::Insert => Key::Insert,
        VirtualKeyCode::Home => Key::Home,
        VirtualKeyCode::Delete => Key::Delete,
        VirtualKeyCode::End => Key::End,
        VirtualKeyCode::PageDown => Key::PageDown,
        VirtualKeyCode::PageUp => Key::PageUp,
        VirtualKeyCode::Left => Key::Left,
        VirtualKeyCode::Up => Key::Up,
        VirtualKeyCode::Right => Key::Right,
        VirtualKeyCode::Down => Key::Down,
        VirtualKeyCode::Back => Key::BackSpace,
        VirtualKeyCode::Return => Key::Enter(false),
        VirtualKeyCode::Space => Key::Space,
        VirtualKeyCode::Compose => Key::Compose,
        VirtualKeyCode::Numlock => Key::NumLock,
        VirtualKeyCode::Numpad0 => Key::Num(0, true),
        VirtualKeyCode::Numpad1 => Key::Num(1, true),
        VirtualKeyCode::Numpad2 => Key::Num(2, true),
        VirtualKeyCode::Numpad3 => Key::Num(3, true),
        VirtualKeyCode::Numpad4 => Key::Num(4, true),
        VirtualKeyCode::Numpad5 => Key::Num(5, true),
        VirtualKeyCode::Numpad6 => Key::Num(6, true),
        VirtualKeyCode::Numpad7 => Key::Num(7, true),
        VirtualKeyCode::Numpad8 => Key::Num(8, true),
        VirtualKeyCode::Numpad9 => Key::Num(9, true),
        //VirtualKeyCode::AbntC1,
        //VirtualKeyCode::AbntC2,
        VirtualKeyCode::Add => Key::Plus(false),
        VirtualKeyCode::Apostrophe => Key::Apostrophe,
        //VirtualKeyCode::Apps,
        //VirtualKeyCode::At,
        //VirtualKeyCode::Ax,
        VirtualKeyCode::Backslash => Key::Backslash,
        //VirtualKeyCode::Calculator,
        //VirtualKeyCode::Capital,
        VirtualKeyCode::Colon => Key::Colon,
        VirtualKeyCode::Comma => Key::Comma(false),
        //VirtualKeyCode::Convert,
        //VirtualKeyCode::Decimal,
        VirtualKeyCode::Divide => Key::Slash(true),
        VirtualKeyCode::Equals => Key::Equals(false),
        VirtualKeyCode::Grave => Key::Grave,
        //VirtualKeyCode::Kana,
        //VirtualKeyCode::Kanji,
        VirtualKeyCode::LAlt => Key::Alt(KeySide::Left),
        VirtualKeyCode::LBracket => Key::LBracket,
        VirtualKeyCode::LControl => Key::Control(KeySide::Left),
        VirtualKeyCode::LShift => Key::Shift(KeySide::Left),
        VirtualKeyCode::LWin => Key::Meta(KeySide::Left),
        //VirtualKeyCode::Mail,
        //VirtualKeyCode::MediaSelect,
        //VirtualKeyCode::MediaStop,
        VirtualKeyCode::Minus => Key::Minus(false),
        VirtualKeyCode::Multiply => Key::Multiply(false),
        //VirtualKeyCode::Mute,
        //VirtualKeyCode::MyComputer,
        //VirtualKeyCode::NavigateForward,
        //VirtualKeyCode::NavigateBackward,
        //VirtualKeyCode::NextTrack,
        //VirtualKeyCode::NoConvert,
        VirtualKeyCode::NumpadComma => Key::Comma(true),
        VirtualKeyCode::NumpadEnter => Key::Enter(true),
        VirtualKeyCode::NumpadEquals => Key::Equals(true),
        //VirtualKeyCode::OEM102,
        VirtualKeyCode::Period => Key::Period,
        //VirtualKeyCode::PlayPause,
        //VirtualKeyCode::Power,
        //VirtualKeyCode::PrevTrack,
        VirtualKeyCode::RAlt => Key::Alt(KeySide::Right),
        VirtualKeyCode::RBracket => Key::RBracket,
        VirtualKeyCode::RControl => Key::Control(KeySide::Right),
        VirtualKeyCode::RShift => Key::Shift(KeySide::Right),
        VirtualKeyCode::RWin => Key::Meta(KeySide::Right),
        VirtualKeyCode::Semicolon => Key::Semicolon,
        VirtualKeyCode::Slash => Key::Slash(false),
        //VirtualKeyCode::Sleep,
        //VirtualKeyCode::Stop,
        VirtualKeyCode::Subtract => Key::Minus(true),
        //VirtualKeyCode::Sysrq,
        VirtualKeyCode::Tab => Key::Tab,
        //VirtualKeyCode::Underline,
        //VirtualKeyCode::Unlabeled,
        //VirtualKeyCode::VolumeDown,
        //VirtualKeyCode::VolumeUp,
        //VirtualKeyCode::Wake,
        //VirtualKeyCode::WebBack,
        //VirtualKeyCode::WebFavorites,
        //VirtualKeyCode::WebForward,
        //VirtualKeyCode::WebHome,
        //VirtualKeyCode::WebRefresh,
        //VirtualKeyCode::WebSearch,
        //VirtualKeyCode::WebStop,
        //VirtualKeyCode::Yen,
        //VirtualKeyCode::Copy,
        //VirtualKeyCode::Paste,
        //VirtualKeyCode::Cut,
        unk => {
            dbg!(unk);
            Key::Unk
        }
    }
}