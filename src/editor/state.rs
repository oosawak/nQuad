//! エディタの状態管理
//!
//! 現在のスプライト ID、選択中のツール、カラー、ズーム、パンなどを管理します。

use crate::resource::ColorMode;

/// エディタの状態
#[derive(Clone, Debug)]
pub struct EditorState {
    /// 編集中のスプライト ID
    pub sprite_id: Option<usize>,
    /// 選択中のブラシカラー
    pub brush_color: [u8; 4],
    /// ブラシサイズ（ピクセル）
    pub brush_size: u32,
    /// キャンバスズーム倍率（1.0 = 等倍）
    pub zoom: f32,
    /// キャンバスパン X
    pub pan_x: f32,
    /// キャンバスパン Y
    pub pan_y: f32,
    /// 現在のカラーモード（読み取り用）
    pub current_color_mode: Option<ColorMode>,
}

impl EditorState {
    /// 新規エディタ状態を作成
    pub fn new() -> Self {
        Self {
            sprite_id: None,
            brush_color: [255, 0, 0, 255], // 赤
            brush_size: 1,
            zoom: 8.0,
            pan_x: 0.0,
            pan_y: 0.0,
            current_color_mode: None,
        }
    }

    /// スプライト ID と カラーモードを設定
    pub fn set_sprite(&mut self, sprite_id: usize, color_mode: ColorMode) {
        self.sprite_id = Some(sprite_id);
        self.current_color_mode = Some(color_mode);
        self.zoom = 8.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
    }

    /// ズーム イン
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(32.0);
    }

    /// ズーム アウト
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(1.0);
    }

    /// ブラシカラーを設定（Indexed256 の場合はインデックス）
    pub fn set_brush_color(&mut self, color: [u8; 4]) {
        self.brush_color = color;
    }

    /// ブラシサイズを設定
    pub fn set_brush_size(&mut self, size: u32) {
        self.brush_size = size.max(1);
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
