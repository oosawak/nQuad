//! キーボードショートカット管理
//!
//! エディタの各種操作をキーボードから実行します。

use macroquad::prelude::*;

/// キーボード入力マネージャー
pub struct InputManager {
    /// Ctrl キーが押されているか
    pub ctrl_pressed: bool,
    /// Shift キーが押されているか
    pub shift_pressed: bool,
}

impl InputManager {
    /// 新規インプットマネージャーを作成
    pub fn new() -> Self {
        Self {
            ctrl_pressed: false,
            shift_pressed: false,
        }
    }

    /// フレームごとのキー状態を更新
    pub fn update(&mut self) {
        self.ctrl_pressed = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        self.shift_pressed = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    }

    /// Ctrl+S が押された
    pub fn save_pressed(&self) -> bool {
        self.ctrl_pressed && is_key_pressed(KeyCode::S)
    }

    /// Ctrl+O が押された
    pub fn open_pressed(&self) -> bool {
        self.ctrl_pressed && is_key_pressed(KeyCode::O)
    }

    /// Ctrl+Z が押された（Undo）
    pub fn undo_pressed(&self) -> bool {
        self.ctrl_pressed && is_key_pressed(KeyCode::Z)
    }

    /// Ctrl+Y が押された（Redo）
    pub fn redo_pressed(&self) -> bool {
        self.ctrl_pressed && is_key_pressed(KeyCode::Y)
    }

    /// + キー（ズームイン）
    pub fn zoom_in_pressed(&self) -> bool {
        is_key_pressed(KeyCode::Equal)
    }

    /// - キー（ズームアウト）
    pub fn zoom_out_pressed(&self) -> bool {
        is_key_pressed(KeyCode::Minus)
    }

    /// W キー（パン上）
    pub fn pan_up(&self) -> bool {
        is_key_down(KeyCode::W)
    }

    /// S キー（パン下）
    pub fn pan_down(&self) -> bool {
        is_key_down(KeyCode::S)
    }

    /// A キー（パン左）
    pub fn pan_left(&self) -> bool {
        is_key_down(KeyCode::A)
    }

    /// D キー（パン右）
    pub fn pan_right(&self) -> bool {
        is_key_down(KeyCode::D)
    }

    /// スペースキー（クリアまたはリセット）
    pub fn space_pressed(&self) -> bool {
        is_key_pressed(KeyCode::Space)
    }

    /// Escape キー（終了）
    pub fn escape_pressed(&self) -> bool {
        is_key_pressed(KeyCode::Escape)
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
