// A bevy system that converts a keycode to a game action based on `KeyboardConfig`

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use crate::model::keyboard::{KeyCode as RKeyCode, KeyboardConfig};
use crate::model::custom_types::GameAction;
use crate::model::player_update::{GameInputEvent, InputEventType};

pub fn bevy_keycode_to_action<GA>(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut game_action_events: EventWriter<GameInputEvent<GA>>,
    config: Res<KeyboardConfig<GA>>,
) where GA: GameAction {
    for ev in evr_kbd.read() {
        let Some(key) = bevy_keycode_to_keyboard_config_key(ev.key_code) else {
            continue;
        };

        if let Some(game_actions) = config.clone().get_actions(&key) {
            for game_action in game_actions {
                let action_type = match ev.state {
                    ButtonState::Pressed => InputEventType::Begin,
                    ButtonState::Released => InputEventType::End,
                };

                debug!(
                    "keycode: {:?}, state: {:?}, game action: {:?}, action type: {:?}",
                    ev.key_code,
                    ev.state,
                    game_action,
                    action_type
                );
                game_action_events.send(GameInputEvent::new(game_action.clone(), action_type));
            }
        }
    }
}

fn bevy_keycode_to_keyboard_config_key(key_code: KeyCode) -> Option<RKeyCode> {
    let x = match key_code {
        KeyCode::Backquote => RKeyCode::Backquote,
        KeyCode::Backslash => RKeyCode::Backslash,
        KeyCode::BracketLeft => RKeyCode::BracketLeft,
        KeyCode::BracketRight => RKeyCode::BracketRight,
        KeyCode::Comma => RKeyCode::Comma,
        KeyCode::Digit0 => RKeyCode::Digit0,
        KeyCode::Digit1 => RKeyCode::Digit1,
        KeyCode::Digit2 => RKeyCode::Digit2,
        KeyCode::Digit3 => RKeyCode::Digit3,
        KeyCode::Digit4 => RKeyCode::Digit4,
        KeyCode::Digit5 => RKeyCode::Digit5,
        KeyCode::Digit6 => RKeyCode::Digit6,
        KeyCode::Digit7 => RKeyCode::Digit7,
        KeyCode::Digit8 => RKeyCode::Digit8,
        KeyCode::Digit9 => RKeyCode::Digit9,
        KeyCode::Equal => RKeyCode::Equal,
        KeyCode::IntlBackslash => RKeyCode::IntlBackslash,
        KeyCode::IntlRo => RKeyCode::IntlRo,
        KeyCode::IntlYen => RKeyCode::IntlYen,
        KeyCode::KeyA => RKeyCode::KeyA,
        KeyCode::KeyB => RKeyCode::KeyB,
        KeyCode::KeyC => RKeyCode::KeyC,
        KeyCode::KeyD => RKeyCode::KeyD,
        KeyCode::KeyE => RKeyCode::KeyE,
        KeyCode::KeyF => RKeyCode::KeyF,
        KeyCode::KeyG => RKeyCode::KeyG,
        KeyCode::KeyH => RKeyCode::KeyH,
        KeyCode::KeyI => RKeyCode::KeyI,
        KeyCode::KeyJ => RKeyCode::KeyJ,
        KeyCode::KeyK => RKeyCode::KeyK,
        KeyCode::KeyL => RKeyCode::KeyL,
        KeyCode::KeyM => RKeyCode::KeyM,
        KeyCode::KeyN => RKeyCode::KeyN,
        KeyCode::KeyO => RKeyCode::KeyO,
        KeyCode::KeyP => RKeyCode::KeyP,
        KeyCode::KeyQ => RKeyCode::KeyQ,
        KeyCode::KeyR => RKeyCode::KeyR,
        KeyCode::KeyS => RKeyCode::KeyS,
        KeyCode::KeyT => RKeyCode::KeyT,
        KeyCode::KeyU => RKeyCode::KeyU,
        KeyCode::KeyV => RKeyCode::KeyV,
        KeyCode::KeyW => RKeyCode::KeyW,
        KeyCode::KeyX => RKeyCode::KeyX,
        KeyCode::KeyY => RKeyCode::KeyY,
        KeyCode::KeyZ => RKeyCode::KeyZ,
        KeyCode::Minus => RKeyCode::Minus,
        KeyCode::Period => RKeyCode::Period,
        KeyCode::Quote => RKeyCode::Quote,
        KeyCode::Semicolon => RKeyCode::Semicolon,
        KeyCode::Slash => RKeyCode::Slash,
        KeyCode::AltLeft => RKeyCode::AltLeft,
        KeyCode::AltRight => RKeyCode::AltRight,
        KeyCode::Backspace => RKeyCode::Backspace,
        KeyCode::CapsLock => RKeyCode::CapsLock,
        KeyCode::ContextMenu => RKeyCode::ContextMenu,
        KeyCode::ControlLeft => RKeyCode::ControlLeft,
        KeyCode::ControlRight => RKeyCode::ControlRight,
        KeyCode::Enter => RKeyCode::Enter,
        KeyCode::SuperLeft => RKeyCode::MetaLeft,  // Super/Win key maps to Meta
        KeyCode::SuperRight => RKeyCode::MetaRight,
        KeyCode::ShiftLeft => RKeyCode::ShiftLeft,
        KeyCode::ShiftRight => RKeyCode::ShiftRight,
        KeyCode::Space => RKeyCode::Space,
        KeyCode::Tab => RKeyCode::Tab,
        KeyCode::Convert => RKeyCode::Convert,
        KeyCode::KanaMode => RKeyCode::KanaMode,
        KeyCode::Lang1 => RKeyCode::Lang1,
        KeyCode::Lang2 => RKeyCode::Lang2,
        KeyCode::Lang3 => RKeyCode::Lang3,
        KeyCode::Lang4 => RKeyCode::Lang4,
        KeyCode::Lang5 => RKeyCode::Lang5,
        KeyCode::NonConvert => RKeyCode::NonConvert,
        KeyCode::Delete => RKeyCode::Delete,
        KeyCode::End => RKeyCode::End,
        KeyCode::Help => RKeyCode::Help,
        KeyCode::Home => RKeyCode::Home,
        KeyCode::Insert => RKeyCode::Insert,
        KeyCode::PageDown => RKeyCode::PageDown,
        KeyCode::PageUp => RKeyCode::PageUp,
        KeyCode::ArrowDown => RKeyCode::ArrowDown,
        KeyCode::ArrowLeft => RKeyCode::ArrowLeft,
        KeyCode::ArrowRight => RKeyCode::ArrowRight,
        KeyCode::ArrowUp => RKeyCode::ArrowUp,
        KeyCode::NumLock => RKeyCode::NumLock,
        KeyCode::Numpad0 => RKeyCode::Numpad0,
        KeyCode::Numpad1 => RKeyCode::Numpad1,
        KeyCode::Numpad2 => RKeyCode::Numpad2,
        KeyCode::Numpad3 => RKeyCode::Numpad3,
        KeyCode::Numpad4 => RKeyCode::Numpad4,
        KeyCode::Numpad5 => RKeyCode::Numpad5,
        KeyCode::Numpad6 => RKeyCode::Numpad6,
        KeyCode::Numpad7 => RKeyCode::Numpad7,
        KeyCode::Numpad8 => RKeyCode::Numpad8,
        KeyCode::Numpad9 => RKeyCode::Numpad9,
        _ => return None,
    };

    Some(x)
}
