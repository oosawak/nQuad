//! nQuad 型エイリアスと公開 API
//!
//! 短縮名称 "nQuad" (nQ) 統一で、明確な API インターフェース

// ===== リソース型 =====
pub use crate::resource::{ColorMode as nQColorMode, SpriteData as nQSpriteData};

// ===== エディタ型 =====
pub use crate::editor::{
    AnimationClip as nQAnimationClip, AnimationController as nQAnimationController,
    BlendMode as nQBlendMode, EditCommand as nQEditCommand,
    EditCommandHistory as nQEditCommandHistory, EditorState as nQEditorState,
    EditorUI as nQEditorUI, Frame as nQFrame, Layer as nQLayer, LayerStack as nQLayerStack,
    SpriteDocument as nQDocument,
};

/// Sprite ID 型（ゲームエンジンで使用）
pub type nQSpriteId = usize;

/// ドキュメント ID 型
pub type nQDocumentId = usize;

/// レイヤー ID 型
pub type nQLayerId = u32;

/// アニメーション再生状態
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum nQPlaybackState {
    Stopped,
    Playing,
    Paused,
}

impl From<crate::editor::animation::PlaybackState> for nQPlaybackState {
    fn from(state: crate::editor::animation::PlaybackState) -> Self {
        use crate::editor::animation::PlaybackState;
        match state {
            PlaybackState::Stopped => nQPlaybackState::Stopped,
            PlaybackState::Playing => nQPlaybackState::Playing,
            PlaybackState::Paused => nQPlaybackState::Paused,
        }
    }
}

/// 描画パラメータ
#[derive(Clone, Debug)]
pub struct nQDrawParams {
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub opacity: f32,
}

impl Default for nQDrawParams {
    fn default() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            opacity: 1.0,
        }
    }
}

/// キーコード（macroquad の KeyCode エイリアス）
pub use macroquad::input::KeyCode as nQKeyCode;

/// マウスボタン
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum nQMouseButton {
    Left,
    Right,
    Middle,
}

/// 色型（RGBA）
pub type nQColor = [u8; 4];

// ===== ヘルパー関数 =====

/// RGB 色を RGBA に変換（アルファ = 255）
pub fn nq_color(r: u8, g: u8, b: u8) -> nQColor {
    [r, g, b, 255]
}

/// RGBA 色を生成
pub fn nq_color_rgba(r: u8, g: u8, b: u8, a: u8) -> nQColor {
    [r, g, b, a]
}

// プリセット色
pub mod colors {
    use super::nQColor;

    pub const BLACK: nQColor = [0, 0, 0, 255];
    pub const WHITE: nQColor = [255, 255, 255, 255];
    pub const RED: nQColor = [255, 0, 0, 255];
    pub const GREEN: nQColor = [0, 255, 0, 255];
    pub const BLUE: nQColor = [0, 0, 255, 255];
    pub const YELLOW: nQColor = [255, 255, 0, 255];
    pub const CYAN: nQColor = [0, 255, 255, 255];
    pub const MAGENTA: nQColor = [255, 0, 255, 255];
    pub const TRANSPARENT: nQColor = [0, 0, 0, 0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_helpers() {
        let col = nq_color(255, 0, 0);
        assert_eq!(col, [255, 0, 0, 255]);

        let col_rgba = nq_color_rgba(0, 255, 0, 128);
        assert_eq!(col_rgba, [0, 255, 0, 128]);
    }

    #[test]
    fn test_draw_params_default() {
        let params = nQDrawParams::default();
        assert_eq!(params.scale_x, 1.0);
        assert_eq!(params.rotation, 0.0);
        assert!(!params.flip_x);
    }
}
