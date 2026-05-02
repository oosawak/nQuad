//! Lineboy - 直線描画ゲーム
//!
//! キーボードで直線を描く簡単なゲーム

use macroquad::prelude::*;
use nantaraquad::platform::MacroquadBackend;
use nantaraquad::{create_sprite, set_pixel};

struct LineState {
    drawing: bool,
    last_x: u32,
    last_y: u32,
    color: [u8; 4],
}

impl LineState {
    fn new() -> Self {
        Self {
            drawing: false,
            last_x: 0,
            last_y: 0,
            color: [255, 255, 255, 255], // 白
        }
    }

    fn update(&mut self, input: &nantaraquad::platform::InputState) {
        // スペースキーで描画開始/終了
        if input.is_key_pressed(nantaraquad::platform::Key::Start) {
            self.drawing = !self.drawing;
        }

        // 矢印キーで色変更
        if input.is_key_pressed(nantaraquad::platform::Key::Up) {
            self.color = [255, 0, 0, 255]; // 赤
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Down) {
            self.color = [0, 255, 0, 255]; // 緑
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Left) {
            self.color = [0, 0, 255, 255]; // 青
        }
        if input.is_key_pressed(nantaraquad::platform::Key::Right) {
            self.color = [255, 255, 0, 255]; // 黄
        }
    }
}

#[macroquad::main("Lineboy")]
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
    let mut line_state = LineState::new();
    let mut prev_mouse_down = false;

    loop {
        clear_background(BLACK);

        // 入力読み込み
        let input = MacroquadBackend::read_input();
        line_state.update(&input);

        // クリア処理
        if input.is_key_pressed(nantaraquad::platform::Key::A) {
            for y in 0..height {
                for x in 0..width {
                    let _ = set_pixel(sprite_id, x, y, &[0, 0, 0, 255]);
                }
            }
        }

        // マウスで描画
        let is_mouse_down = is_mouse_button_down(MouseButton::Left) && line_state.drawing;
        if is_mouse_down {
            let (mx, my) = mouse_position();
            let x = (mx as u32).min(width - 1);
            let y = (my as u32).min(height - 1);

            // ピクセル描画
            let _ = set_pixel(sprite_id, x, y, &line_state.color);

            // 簡単な直線補間
            if line_state.last_x != x || line_state.last_y != y {
                bresenham_line(sprite_id, line_state.last_x, line_state.last_y, x, y, &line_state.color);
            }

            line_state.last_x = x;
            line_state.last_y = y;
        }
        prev_mouse_down = is_mouse_down;

        // テクスチャに変換して描画（直接描画）
        draw_text("Lineboy - ドット絵描画ゲーム", 10.0, 20.0, 20.0, YELLOW);
        draw_text("Space: Toggle Draw | Arrow Keys: Change Color | A: Clear", 10.0, 45.0, 15.0, WHITE);
        draw_text(format!("Mode: {}", if line_state.drawing { "DRAWING" } else { "VIEWING" }).as_str(), 10.0, 65.0, 15.0, 
                  if line_state.drawing { RED } else { GRAY });

        MacroquadBackend::next_frame().await;
    }
}

/// Bresenham の直線アルゴリズムで直線を描画
fn bresenham_line(sprite_id: usize, x0: u32, y0: u32, x1: u32, y1: u32, color: &[u8; 4]) {
    let x0 = x0 as i32;
    let y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < 512 && y >= 0 && y < 512 {
            let _ = set_pixel(sprite_id, x as u32, y as u32, color);
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
