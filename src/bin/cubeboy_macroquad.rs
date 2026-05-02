//! Cubeboy - 立方体描画ゲーム
//!
//! ゲームパッドで立方体を操作して描画する

use macroquad::prelude::*;
use nantaraquad::platform::MacroquadBackend;
use nantaraquad::{create_sprite, set_pixel};

struct Cube {
    x: f32,
    y: f32,
    size: u32,
    rotation: f32,
    color: [u8; 4],
}

impl Cube {
    fn new() -> Self {
        Self {
            x: 256.0,
            y: 256.0,
            size: 32,
            rotation: 0.0,
            color: [0, 255, 255, 255], // シアン
        }
    }

    fn update(&mut self, input: &nantaraquad::platform::InputState) {
        // キーボード操作
        if input.is_key_pressed(nantaraquad::platform::Key::Up) {
            self.y -= 5.0;
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Down) {
            self.y += 5.0;
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Left) {
            self.x -= 5.0;
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Right) {
            self.x += 5.0;
        }

        // 色変更
        if input.is_key_pressed(nantaraquad::platform::Key::A) {
            self.color = [255, 0, 0, 255]; // 赤
        }
        if input.is_key_pressed(nantaraquad::platform::Key::B) {
            self.color = [0, 255, 0, 255]; // 緑
        }

        // 回転
        self.rotation += 2.0;
        if self.rotation >= 360.0 {
            self.rotation = 0.0;
        }

        // 画面内に制限
        self.x = self.x.max(0.0).min(512.0 - self.size as f32);
        self.y = self.y.max(0.0).min(512.0 - self.size as f32);
    }

    fn draw_to_sprite(&self, sprite_id: usize) {
        let x = self.x as u32;
        let y = self.y as u32;
        let s = self.size;

        // 立方体の正面を描画（簡略版）
        // 外枠を描画
        for i in 0..s {
            if x + i < 512 {
                let _ = set_pixel(sprite_id, x + i, y, &self.color);
                let _ = set_pixel(sprite_id, x + i, y + s - 1, &self.color);
            }
        }

        for i in 0..s {
            if y + i < 512 {
                let _ = set_pixel(sprite_id, x, y + i, &self.color);
                let _ = set_pixel(sprite_id, x + s - 1, y + i, &self.color);
            }
        }

        // 内部グリッド
        let grid_step = s / 4;
        for i in 0..=4 {
            let pos = i * grid_step;
            if pos < s {
                for j in 0..s {
                    let j_pos = pos;
                    if x + pos < 512 && y + j < 512 {
                        let _ = set_pixel(sprite_id, x + pos, y + j, &[
                            (self.color[0] as f32 * 0.7) as u8,
                            (self.color[1] as f32 * 0.7) as u8,
                            (self.color[2] as f32 * 0.7) as u8,
                            self.color[3],
                        ]);
                    }
                    if x + j < 512 && y + j_pos < 512 {
                        let _ = set_pixel(sprite_id, x + j, y + j_pos, &[
                            (self.color[0] as f32 * 0.7) as u8,
                            (self.color[1] as f32 * 0.7) as u8,
                            (self.color[2] as f32 * 0.7) as u8,
                            self.color[3],
                        ]);
                    }
                }
            }
        }
    }
}

#[macroquad::main("Cubeboy")]
async fn main() {
    let width = 512;
    let height = 512;

    // スプライトを作成（キャンバス）
    let sprite_id = create_sprite(width, height);

    // 初期化：すべて黒で塗りつぶし
    for y in 0..height {
        for x in 0..width {
            let _ = set_pixel(sprite_id, x, y, &[0, 0, 0, 255]);
        }
    }

    let backend = MacroquadBackend::new(width, height, 60);
    let mut cube = Cube::new();

    loop {
        // 背景をクリア
        clear_background(BLACK);

        // フレーム開始時にキャンバスを黒でクリア
        for y in 0..height {
            for x in 0..width {
                let _ = set_pixel(sprite_id, x, y, &[0, 0, 0, 255]);
            }
        }

        // 入力読み込みと更新
        let input = MacroquadBackend::read_input();
        cube.update(&input);

        // キューブを描画
        cube.draw_to_sprite(sprite_id);

        // 状態表示
        draw_text("Cubeboy - 立方体描画ゲーム", 10.0, 20.0, 20.0, YELLOW);
        draw_text("Arrow Keys: Move | A: Red | B: Green", 10.0, 45.0, 15.0, WHITE);
        draw_text(format!("Pos: ({:.0}, {:.0}) Color: {:?}", cube.x, cube.y, cube.color).as_str(), 10.0, 65.0, 15.0, GRAY);

        MacroquadBackend::next_frame().await;
    }
}
