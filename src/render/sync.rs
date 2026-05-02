use crate::resource::{ColorMode, SpriteData};
use macroquad::prelude::*;

/// スプライトをMacroquadのイメージに変換
pub fn image_from_sprite(sprite: &SpriteData) -> Image {
    match &sprite.mode {
        ColorMode::FullColor => {
            // FullColor: 既に RGBA (4bytes/pixel)
            Image {
                bytes: sprite.pixels.clone(),
                width: sprite.width as u16,
                height: sprite.height as u16,
            }
        }
        ColorMode::Indexed256(palette) => {
            // Indexed256: インデックス → RGBA に変換
            let mut rgba = Vec::with_capacity(sprite.pixels.len() * 4);
            for &idx in &sprite.pixels {
                // パレット範囲外は透明黒でフォールバック
                let color = palette.get(idx as usize).copied().unwrap_or([0, 0, 0, 0]);
                rgba.extend_from_slice(&color);
            }
            Image {
                bytes: rgba,
                width: sprite.width as u16,
                height: sprite.height as u16,
            }
        }
    }
}

/// スプライトからTexture2Dを作成
pub fn sync_texture_from_sprite(sprite: &SpriteData) -> Texture2D {
    let image = image_from_sprite(sprite);
    let texture = Texture2D::from_image(&image);
    // ドット絵用に最近傍フィルタリングを設定
    texture.set_filter(FilterMode::Nearest);
    texture
}
