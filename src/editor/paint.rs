//! ピクセルペイント機能
//!
//! マウス入力でスプライトにピクセルを描画します。
//! Undo/Redo スタックに変更を記録します。

use super::history::{PixelChange, UndoRedoStack};
use super::state::EditorState;
use crate::api::{get_pixel, set_pixel};
use macroquad::prelude::*;

/// ピクセルペイントツール
pub struct PaintTool {
    /// 前フレームのマウス位置
    last_mouse_x: f32,
    last_mouse_y: f32,
    /// ペイント中か
    is_painting: bool,
}

impl PaintTool {
    /// 新規ペイントツールを作成
    pub fn new() -> Self {
        Self {
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            is_painting: false,
        }
    }

    /// マウス入力を処理
    pub fn update(&mut self, state: &EditorState, history: &mut UndoRedoStack) {
        let (mx, my) = mouse_position();

        // マウスボタン判定
        if is_mouse_button_pressed(MouseButton::Left) {
            self.is_painting = true;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.is_painting = false;
        }

        if self.is_painting {
            if let Some(sprite_id) = state.sprite_id {
                // マウス座標をキャンバス座標に変換（ズーム・パンを適用）
                if let Some((px, py)) = self.screen_to_sprite(mx, my, state) {
                    self.paint_pixel(sprite_id, px as u32, py as u32, state, history);

                    // アンチエイリアス的に線を引く（前フレームとの中間点）
                    if self.last_mouse_x != 0.0 || self.last_mouse_y != 0.0 {
                        self.paint_line(
                            sprite_id,
                            self.last_mouse_x,
                            self.last_mouse_y,
                            px,
                            py,
                            state,
                            history,
                        );
                    }
                }
            }
        }

        self.last_mouse_x = mx;
        self.last_mouse_y = my;
    }

    /// スクリーン座標をスプライト座標に変換
    fn screen_to_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        state: &EditorState,
    ) -> Option<(f32, f32)> {
        let canvas_x = (screen_x - state.pan_x) / state.zoom;
        let canvas_y = (screen_y - state.pan_y) / state.zoom;

        if canvas_x >= 0.0 && canvas_y >= 0.0 {
            Some((canvas_x, canvas_y))
        } else {
            None
        }
    }

    /// 1ピクセルを描画
    fn paint_pixel(
        &self,
        sprite_id: usize,
        x: u32,
        y: u32,
        state: &EditorState,
        history: &mut UndoRedoStack,
    ) {
        let color = state.brush_color;

        // 古い色を取得
        if let Some(old_color) = get_pixel(sprite_id, x, y) {
            // 新しい色を設定
            let _ = set_pixel(sprite_id, x, y, &color);

            // 変更を履歴に記録
            history.record(PixelChange {
                sprite_id,
                x,
                y,
                old_color,
                new_color: color.to_vec(),
            });
        }
    }

    /// 2点間を線で結ぶ（簡易 Bresenham）
    fn paint_line(
        &self,
        sprite_id: usize,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        state: &EditorState,
        history: &mut UndoRedoStack,
    ) {
        let steps = ((x1 - x0).abs().max((y1 - y0).abs()) as usize).max(1);
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = (x0 + (x1 - x0) * t) as u32;
            let y = (y0 + (y1 - y0) * t) as u32;
            self.paint_pixel(sprite_id, x, y, state, history);
        }
    }
}

impl Default for PaintTool {
    fn default() -> Self {
        Self::new()
    }
}
