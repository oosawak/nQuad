//! Platform integration module
//!
//! macroquad バックエンド、入力ブリッジ、描画統合を提供

pub mod macroquad_backend;
pub mod input_bridge;

pub use macroquad_backend::{
    MacroquadBackend, InputState, InputState as PlatformInputState,
    Key, GamepadInput, GamepadButton,
};
pub use input_bridge::InputBridge;
