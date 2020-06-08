use crate::geometry::{Point, Position, Size};
use std::path::PathBuf;
use std::time::Instant;

mod dispatcher;
pub use dispatcher::*;

/// Raw key id from hardware.
pub type ScanCode = u32;

/// Input events that come from the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Raw keyboard input.
    Keyboard {
        state: EvState,
        key: Key,
        scancode: ScanCode,
    },
    /// Processed keyboard input as an unicode character.
    Character(char),
    /// Mouse pointer motion.
    MouseMoved(AxisValue),
    /// Mouse button input.
    MouseButton { state: EvState, button: MouseButton },
    /// Pointer has crossed the window boundaries.
    PointerInside(bool),
    /// A file has been dropped into the window.
    FileDropped(PathBuf),
    /// Window resized.
    Resized(Size),
    /// Window moved.
    Moved(Position),
    /// Window focused state.
    Focused(bool),
    /// Window close button pressed.
    CloseRequest,
    /// Window has been created.
    Created,
    /// Window has been destroyed.
    Destroyed,
}

/// Current state associated with an event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventContext {
    /// Instant when the event was received.
    pub timestamp: Instant,
    /// Last known cursor position, relative to the widget.
    pub pointer_pos: Point<f64>,
    /// Last known cursor position, relative to the window.
    pub abs_pos: Point<f64>,
    /// Current mouse button state.
    pub button_state: ButtonState,
    /// Current keyboard modifier state.
    pub mod_state: ModState,
}

impl EventContext {
    /// Creates a new event context using the specified data.
    #[inline]
    pub fn new(pointer_pos: Point<f64>, button_state: ButtonState, mod_state: ModState) -> Self {
        Self {
            timestamp: Instant::now(),
            pointer_pos,
            abs_pos: pointer_pos,
            button_state,
            mod_state,
        }
    }
}

/// State of keys or mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvState {
    Released,
    Pressed,
}

impl Default for EvState {
    #[inline]
    fn default() -> Self {
        EvState::Released
    }
}

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

impl MouseButton {
    /// Gets the button number.
    #[inline]
    pub fn number(self) -> u8 {
        match self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::Other(n) => n,
        }
    }

    /// Gets the bitmask for this button.
    #[inline]
    fn mask(self) -> u64 {
        match self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 4,
            MouseButton::Other(n) => 1u64 << n,
        }
    }
}

/// Axis of movement for mouse pointer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisValue {
    Position(Point<f64>),
    Scroll(f32, f32),
    Pressure(f64),
    Tilt(f64, f64),
}

/// Keyboard modifier state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Mouse button state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ButtonState(u64);

impl ButtonState {
    /// Sets the specificed button as pressed.
    #[inline]
    pub fn set(&mut self, button: MouseButton) {
        self.0 |= button.mask();
    }

    /// Sets the specificed button as released.
    #[inline]
    pub fn unset(&mut self, button: MouseButton) {
        self.0 &= !button.mask();
    }

    /// Checks if the specified button is pressed.
    #[inline]
    pub fn is_set(self, button: MouseButton) -> bool {
        self.0 & button.mask() != 0
    }

    /// Checks if the left button is pressed.
    #[inline]
    pub fn left(self) -> bool {
        self.is_set(MouseButton::Left)
    }

    /// Checks if the middle button is pressed.
    #[inline]
    pub fn middle(self) -> bool {
        self.is_set(MouseButton::Middle)
    }

    /// Checks if the right button is pressed.
    #[inline]
    pub fn right(self) -> bool {
        self.is_set(MouseButton::Right)
    }
}

/// Side for duplicated modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySide {
    Left,
    Right,
}

pub type IsNumpad = bool;

/// Symbolic key definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Number keys
    Num(u8, IsNumpad),
    /// Letters
    Letter(char),
    /// Function keys
    Fn(u8),
    /// The space bar
    Space,
    // Main control keys
    Escape,
    BackSpace,
    Tab,
    Enter(IsNumpad),
    CapsLock,
    Shift(KeySide),
    Control(KeySide),
    Alt(KeySide),
    Super(KeySide),
    Meta(KeySide),
    Compose,
    // Secondary control keys
    PrintScr,
    ScrollLock,
    Pause,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    // Arrow keys
    Up,
    Down,
    Left,
    Right,
    // Numpad extra
    NumLock,
    NumpadDelete,
    NumpadEnter,
    // Other stuff
    Plus(IsNumpad),
    Minus(IsNumpad),
    Multiply(IsNumpad),
    Slash(IsNumpad),
    Backslash,
    Comma(IsNumpad),
    Period,
    Colon,
    Semicolon,
    Apostrophe,
    Grave,
    LBracket,
    RBracket,
    Equals(IsNumpad),
    /// unknown key, raw id in scancode
    Unk,
}

/// The result of processing an input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventResult {
    Pass,
    Consumed,
}

impl EventResult {
    #[inline]
    pub fn consumed(self) -> bool {
        self == EventResult::Consumed
    }
}

impl Default for EventResult {
    #[inline]
    fn default() -> Self {
        EventResult::Pass
    }
}
