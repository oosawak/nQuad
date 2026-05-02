use std::collections::HashSet;

/// pyxel 互換のキー定義
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,
    // WASD
    W,
    A,
    S,
    D,
    // Gamepad
    GamepadDpadUp,
    GamepadDpadDown,
    GamepadDpadLeft,
    GamepadDpadRight,
    GamepadButtonA,
    GamepadButtonB,
    GamepadButtonX,
    GamepadButtonY,
    // Others
    Space,
    Enter,
    Escape,
}

/// 入力状態を管理
pub struct InputState {
    pressed: HashSet<Key>,
    released: HashSet<Key>,
}

impl InputState {
    /// 新規入力状態を作成
    pub fn new() -> Self {
        InputState {
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }

    /// btn(key) - キーが押されているか
    pub fn btn(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    /// btnp(key) - キーが今フレーム押されたか（フレーム開始時に released に移動されるまで）
    pub fn btnp(&self, key: Key) -> bool {
        self.released.contains(&key)
    }

    /// フレーム更新（released を消す、新しいキー入力を受け付ける）
    pub fn update_frame(&mut self) {
        self.released.clear();
    }

    /// キー押下を登録
    pub fn press_key(&mut self, key: Key) {
        if !self.pressed.contains(&key) {
            self.released.insert(key);
        }
        self.pressed.insert(key);
    }

    /// キー解放を登録
    pub fn release_key(&mut self, key: Key) {
        self.pressed.remove(&key);
    }

    /// すべてのキーをリセット
    pub fn reset(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_creation() {
        let input = InputState::new();
        assert!(!input.btn(Key::Up));
        assert!(!input.btnp(Key::Up));
    }

    #[test]
    fn test_key_press() {
        let mut input = InputState::new();
        input.press_key(Key::Up);
        assert!(input.btn(Key::Up));
        assert!(input.btnp(Key::Up));
    }

    #[test]
    fn test_key_release() {
        let mut input = InputState::new();
        input.press_key(Key::Up);
        input.release_key(Key::Up);
        assert!(!input.btn(Key::Up));
    }

    #[test]
    fn test_frame_update() {
        let mut input = InputState::new();
        input.press_key(Key::Up);
        assert!(input.btnp(Key::Up));
        input.update_frame();
        assert!(!input.btnp(Key::Up));
        assert!(input.btn(Key::Up)); // still pressed
    }

    #[test]
    fn test_multiple_keys() {
        let mut input = InputState::new();
        input.press_key(Key::Up);
        input.press_key(Key::Left);
        assert!(input.btn(Key::Up));
        assert!(input.btn(Key::Left));
        assert!(!input.btn(Key::Down));
    }
}
