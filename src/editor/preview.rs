//! スプライトのリアルタイムプレビュー
//!
//! Macroquad の描画機構を使ってスプライトを画面に表示します。

use super::state::EditorState;
use crate::api::draw_sprite_scaled;
use macroquad::prelude::*;

/// スプライトプレビューレンダラー
pub struct SpritePreview;

impl SpritePreview {
    /// キャンバスにスプライトを描画（ズーム・パン適用）
    pub fn draw_canvas(state: &EditorState) {
        if let Some(sprite_id) = state.sprite_id {
            // キャンバスの背景を描画
            draw_rectangle(
                state.pan_x,
                state.pan_y,
                1024.0,
                768.0,
                Color::new(0.2, 0.2, 0.2, 1.0),
            );

            // グリッドを描画（ズーム時）
            if state.zoom > 4.0 {
                Self::draw_grid(state);
            }

            // スプライトを描画（パンとズームを適用）
            draw_sprite_scaled(sprite_id, state.pan_x, state.pan_y, state.zoom);
        }
    }

    /// グリッド線を描画
    fn draw_grid(state: &EditorState) {
        let grid_size = state.zoom as i32;
        if grid_size <= 0 {
            return;
        }

        let color = Color::new(0.3, 0.3, 0.3, 0.5);

        // 縦線
        let mut x = state.pan_x as i32;
        while x < 1024 {
            draw_line(
                x as f32,
                state.pan_y,
                x as f32,
                state.pan_y + 768.0,
                1.0,
                color,
            );
            x += grid_size;
        }

        // 横線
        let mut y = state.pan_y as i32;
        while y < 768 {
            draw_line(
                state.pan_x,
                y as f32,
                state.pan_x + 1024.0,
                y as f32,
                1.0,
                color,
            );
            y += grid_size;
        }
    }

    /// マウスカーソル位置のプレビューを描画
    pub fn draw_cursor_preview(state: &EditorState) {
        let (mx, my) = mouse_position();

        // ブラシのプレビュー円を描画
        let brush_size = state.brush_size as f32 * state.zoom / 2.0;
        if brush_size > 0.1 {
            draw_circle_lines(mx, my, brush_size, 2.0, YELLOW);
        }
    }

    /// スプライト情報をテキスト表示
    pub fn draw_info(state: &EditorState) {
        let mut text_y = 10.0;
        let line_height = 20.0;

        draw_text("Nantaraquad Editor", 10.0, text_y, 20.0, WHITE);
        text_y += line_height;

        if let Some(sprite_id) = state.sprite_id {
            draw_text(&format!("Sprite: #{}", sprite_id), 10.0, text_y, 16.0, GRAY);
            text_y += line_height;

            draw_text(
                &format!(
                    "Color: RGB({},{},{})",
                    state.brush_color[0], state.brush_color[1], state.brush_color[2]
                ),
                10.0,
                text_y,
                16.0,
                GRAY,
            );
            text_y += line_height;

            draw_text(
                &format!("Brush: {}px", state.brush_size),
                10.0,
                text_y,
                16.0,
                GRAY,
            );
            text_y += line_height;

            draw_text(
                &format!("Zoom: {:.1}x", state.zoom),
                10.0,
                text_y,
                16.0,
                GRAY,
            );
        } else {
            draw_text("No sprite loaded", 10.0, text_y, 16.0, RED);
        }
    }
}
