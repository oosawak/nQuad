//! 入力変換ブリッジ
//!
//! macroquad のキーコード・ゲームパッド入力を Nantaraquad の Key に変換

use macroquad::prelude::*;
use super::macroquad_backend::{Key, GamepadButton};

/// 入力変換ブリッジ
pub struct InputBridge;

impl InputBridge {
    /// macroquad の KeyCode を Nantaraquad Key に変換
    pub fn map_key(code: KeyCode) -> Option<Key> {
        match code {
            KeyCode::Up => Some(Key::Up),
            KeyCode::Down => Some(Key::Down),
            KeyCode::Left => Some(Key::Left),
            KeyCode::Right => Some(Key::Right),
            KeyCode::Z => Some(Key::A),
            KeyCode::X => Some(Key::B),
            KeyCode::Space => Some(Key::Start),
            KeyCode::Escape => Some(Key::Select),
            _ => None,
        }
    }
}
