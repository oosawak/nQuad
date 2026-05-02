//! 画像フィルター
//!
//! 反転、グレースケール、明度調整などのシンプルなフィルター。

use crate::resource::SpriteData;

/// フィルター処理
pub struct Filters;

impl Filters {
    /// 色を反転
    pub fn invert(sprite: &mut SpriteData) -> Result<(), String> {
        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(_) => {
                // インデックスカラーの反転は非効率なのでスキップ
                Err("Invert not supported for Indexed256 mode".to_string())
            }
            crate::resource::ColorMode::FullColor => {
                for pixel in sprite.pixels.chunks_mut(4) {
                    if pixel.len() == 4 {
                        pixel[0] = 255 - pixel[0]; // R
                        pixel[1] = 255 - pixel[1]; // G
                        pixel[2] = 255 - pixel[2]; // B
                                                   // A は反転しない
                    }
                }
                Ok(())
            }
        }
    }

    /// グレースケール化
    pub fn grayscale(sprite: &mut SpriteData) -> Result<(), String> {
        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(_) => {
                Err("Grayscale not supported for Indexed256 mode".to_string())
            }
            crate::resource::ColorMode::FullColor => {
                for pixel in sprite.pixels.chunks_mut(4) {
                    if pixel.len() == 4 {
                        let r = pixel[0] as f32;
                        let g = pixel[1] as f32;
                        let b = pixel[2] as f32;
                        // 標準的な輝度計算
                        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                        pixel[0] = gray;
                        pixel[1] = gray;
                        pixel[2] = gray;
                        // A は変更しない
                    }
                }
                Ok(())
            }
        }
    }

    /// 明度を調整
    ///
    /// # 引数
    /// - `brightness`: -1.0 ～ 1.0（-1.0 で黒、0.0 で無変化、1.0 で白）
    pub fn adjust_brightness(sprite: &mut SpriteData, brightness: f32) -> Result<(), String> {
        let brightness = brightness.clamp(-1.0, 1.0);

        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(_) => {
                Err("Brightness not supported for Indexed256 mode".to_string())
            }
            crate::resource::ColorMode::FullColor => {
                for pixel in sprite.pixels.chunks_mut(4) {
                    if pixel.len() == 4 {
                        if brightness > 0.0 {
                            // 明るくする（白に近づける）
                            pixel[0] =
                                ((pixel[0] as f32) + (255.0 - pixel[0] as f32) * brightness) as u8;
                            pixel[1] =
                                ((pixel[1] as f32) + (255.0 - pixel[1] as f32) * brightness) as u8;
                            pixel[2] =
                                ((pixel[2] as f32) + (255.0 - pixel[2] as f32) * brightness) as u8;
                        } else {
                            // 暗くする（黒に近づける）
                            pixel[0] = ((pixel[0] as f32) * (1.0 + brightness)) as u8;
                            pixel[1] = ((pixel[1] as f32) * (1.0 + brightness)) as u8;
                            pixel[2] = ((pixel[2] as f32) * (1.0 + brightness)) as u8;
                        }
                    }
                }
                Ok(())
            }
        }
    }

    /// コントラスト調整
    ///
    /// # 引数
    /// - `contrast`: 0.0 ～ 2.0（0.5 で暗い、1.0 で無変化、2.0 で高コントラスト）
    pub fn adjust_contrast(sprite: &mut SpriteData, contrast: f32) -> Result<(), String> {
        let contrast = contrast.clamp(0.0, 2.0);

        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(_) => {
                Err("Contrast not supported for Indexed256 mode".to_string())
            }
            crate::resource::ColorMode::FullColor => {
                let factor = (contrast - 1.0) * 127.0; // 中心値を127.5とした係数

                for pixel in sprite.pixels.chunks_mut(4) {
                    if pixel.len() == 4 {
                        for i in 0..3 {
                            let value = pixel[i] as f32 - 127.5;
                            let adjusted = (value * contrast + factor) as i16;
                            pixel[i] = adjusted.clamp(0, 255) as u8;
                        }
                    }
                }
                Ok(())
            }
        }
    }

    /// ガウシアンブラー（簡易版）
    ///
    /// 隣接ピクセルとの平均化で実現
    pub fn blur(sprite: &mut SpriteData) -> Result<(), String> {
        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(_) => {
                Err("Blur not supported for Indexed256 mode".to_string())
            }
            crate::resource::ColorMode::FullColor => {
                let width = sprite.width as usize;
                let height = sprite.height as usize;
                let mut blurred = sprite.pixels.clone();

                for y in 1..(height - 1) {
                    for x in 1..(width - 1) {
                        let center = (y * width + x) * 4;

                        for c in 0..4 {
                            let mut sum = 0u32;
                            let mut count = 0u32;

                            // 3x3 近傍をサンプリング
                            for dy in -1..=1 {
                                for dx in -1..=1 {
                                    let ny = (y as i32 + dy) as usize;
                                    let nx = (x as i32 + dx) as usize;
                                    let idx = (ny * width + nx) * 4 + c as usize;
                                    if idx < sprite.pixels.len() {
                                        sum += sprite.pixels[idx] as u32;
                                        count += 1;
                                    }
                                }
                            }

                            if count > 0 {
                                blurred[center + c as usize] = (sum / count) as u8;
                            }
                        }
                    }
                }

                sprite.pixels = blurred;
                Ok(())
            }
        }
    }
}
