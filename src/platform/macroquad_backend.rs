//! macroquad ウィンドウシステム統合
//!
//! Macroquad のウィンドウ、テクスチャシステム、フレームレート管理を
//! Nantaraquad のスプライトデータと統合します。

use crate::resource::SpriteData;
use crate::resource::ColorMode;
use macroquad::prelude::*;

/// macroquad ウィンドウシステムバックエンド
pub struct MacroquadBackend {
    width: u32,
    height: u32,
    fps: u32,
}

impl MacroquadBackend {
    /// 新しい MacroquadBackend を初期化
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        Self { width, height, fps }
    }

    /// ウィンドウ幅を取得
    pub fn width(&self) -> u32 {
        self.width
    }

    /// ウィンドウ高さを取得
    pub fn height(&self) -> u32 {
        self.height
    }

    /// ターゲット FPS を取得
    pub fn fps(&self) -> u32 {
        self.fps
    }

    /// Nantaraquad の SpriteData を macroquad の Image に変換
    pub fn sprite_to_image(&self, sprite: &SpriteData) -> Image {
        let width = sprite.width as usize;
        let height = sprite.height as usize;
        let mut data = vec![0u8; width * height * 4];

        match &sprite.mode {
            ColorMode::FullColor => {
                // FullColor: 各ピクセルが 4 バイト (RGBA)
                for y in 0..height {
                    for x in 0..width {
                        let src_offset = (y * width + x) * 4;
                        let dst_offset = (y * width + x) * 4;
                        
                        if src_offset + 3 < sprite.pixels.len() {
                            data[dst_offset..dst_offset + 4]
                                .copy_from_slice(&sprite.pixels[src_offset..src_offset + 4]);
                        }
                    }
                }
            }
            ColorMode::Indexed256(palette) => {
                // Indexed256: 各ピクセルが 1 バイト（インデックス）
                for y in 0..height {
                    for x in 0..width {
                        let src_offset = y * width + x;
                        if src_offset < sprite.pixels.len() {
                            let idx = sprite.pixels[src_offset] as usize;
                            let color = if idx < palette.len() {
                                palette[idx]
                            } else {
                                [0, 0, 0, 0]
                            };
                            let dst_offset = (y * width + x) * 4;
                            data[dst_offset] = color[0];
                            data[dst_offset + 1] = color[1];
                            data[dst_offset + 2] = color[2];
                            data[dst_offset + 3] = color[3];
                        }
                    }
                }
            }
        }

        Image {
            bytes: data,
            width: width as u16,
            height: height as u16,
        }
    }

    /// macroquad の入力状態を読む（キーボード）
    pub fn read_input() -> InputState {
        let mut keys_pressed = Vec::new();
        
        // キーボード入力
        if is_key_pressed(KeyCode::Up) {
            keys_pressed.push(Key::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            keys_pressed.push(Key::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            keys_pressed.push(Key::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            keys_pressed.push(Key::Right);
        }
        if is_key_pressed(KeyCode::Z) {
            keys_pressed.push(Key::A);
        }
        if is_key_pressed(KeyCode::X) {
            keys_pressed.push(Key::B);
        }
        if is_key_pressed(KeyCode::Space) {
            keys_pressed.push(Key::Start);
        }
        if is_key_pressed(KeyCode::Escape) {
            keys_pressed.push(Key::Select);
        }

        // ゲームパッド入力（簡略版）
        let gamepad_input = GamepadInput::default();

        InputState {
            keys_pressed,
            gamepad: gamepad_input,
        }
    }

    /// フレームレート管理（非同期）
    pub async fn next_frame() {
        next_frame().await;
    }
}

/// キー入力タイプ
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

/// 入力状態
#[derive(Clone, Debug, Default)]
pub struct InputState {
    pub keys_pressed: Vec<Key>,
    pub gamepad: GamepadInput,
}

impl InputState {
    /// 指定したキーが押されているか確認
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }
}

/// ゲームパッド入力
#[derive(Clone, Debug, Default)]
pub struct GamepadInput {
    pub buttons: Vec<GamepadButton>,
    pub stick_left: (f32, f32),
    pub stick_right: (f32, f32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    Start,
    Select,
}

impl GamepadInput {
    /// ゲームパッドボタンが押されているか確認
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        self.buttons.contains(&button)
    }
}
