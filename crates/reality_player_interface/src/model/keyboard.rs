use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::game_action::GameActions;

// Based on https://w3c.github.io/uievents-code/#code-value-tables
// These come from winit
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    MetaLeft,
    MetaRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Hyper,
    Super,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
}

// Binding Key to Arbitrary GameAction
// GameAction is defined by the library user
// It MUST be serializable to string
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct KeyboardConfig<GA> where GA: GameActions {
    bindings: HashMap<KeyCode, Vec<GA>>,
}

impl<GA> KeyboardConfig<GA> where GA: GameActions {
    pub fn new(bindings: HashMap<KeyCode, Vec<GA>>) -> KeyboardConfig<GA> {
        KeyboardConfig { bindings }
    }

    pub fn get_actions(&self, key_code: &KeyCode) -> Option<&Vec<GA>> {
        self.bindings.get(key_code)
    }
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub enum KeyState {
//     Pressed,
//     Released,
// }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyEvent {
    Press,
    Release,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardEvent {
    key_code: KeyCode,
    event: KeyEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardEventTimed {
    timestamp_delta: u64,
    event: KeyboardEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardTimeline {
    timestamp_initial: u64,
    events: Vec<KeyboardEventTimed>,
}
