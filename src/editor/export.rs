//! PNG エクスポート機能
//!
//! スプライトを PNG ファイルとして保存します。

use crate::resource::SpriteData;
use std::path::Path;

/// PNG エクスポーター
pub struct PngExporter;

impl PngExporter {
    /// スプライトを PNG として保存
    ///
    /// # 引数
    /// - `sprite`: エクスポート対象のスプライト
    /// - `path`: 保存先パス（.png）
    ///
    /// # 戻り値
    /// - `Ok(())`: 保存成功
    /// - `Err(String)`: 保存失敗
    pub fn export(sprite: &SpriteData, path: &str) -> Result<(), String> {
        let path = Path::new(path);

        // Image バッファを作成（RGBA）
        let mut img_buffer = image::ImageBuffer::new(sprite.width, sprite.height);

        // スプライトのピクセルを image バッファに変換
        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(palette) => {
                // インデックスカラー → RGBA
                for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
                    let offset = (y as usize * sprite.width as usize) + x as usize;
                    if offset < sprite.pixels.len() {
                        let idx = sprite.pixels[offset] as usize;
                        let color = palette.get(idx).copied().unwrap_or([0, 0, 0, 0]);
                        *pixel = image::Rgba(color);
                    }
                }
            }
            crate::resource::ColorMode::FullColor => {
                // RGBA をそのままコピー
                for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
                    let offset = ((y as usize * sprite.width as usize) + x as usize) * 4;
                    if offset + 3 < sprite.pixels.len() {
                        let r = sprite.pixels[offset];
                        let g = sprite.pixels[offset + 1];
                        let b = sprite.pixels[offset + 2];
                        let a = sprite.pixels[offset + 3];
                        *pixel = image::Rgba([r, g, b, a]);
                    }
                }
            }
        }

        // PNG として保存
        img_buffer
            .save(path)
            .map_err(|e| format!("PNG save failed: {}", e))?;

        Ok(())
    }

    /// 複数のスプライトをスプリートシート（タイル状）で PNG 保存
    ///
    /// # 引数
    /// - `sprites`: スプライトリスト
    /// - `cols`: タイル列数
    /// - `path`: 保存先パス
    ///
    /// # 戻り値
    /// - `Ok(())`: 保存成功
    /// - `Err(String)`: 保存失敗（サイズが異なるなど）
    pub fn export_spritesheet(sprites: &[SpriteData], cols: u32, path: &str) -> Result<(), String> {
        if sprites.is_empty() {
            return Err("No sprites to export".to_string());
        }

        // すべてのスプライトが同じサイズか確認
        let first_size = (sprites[0].width, sprites[0].height);
        for sprite in sprites {
            if (sprite.width, sprite.height) != first_size {
                return Err("All sprites must have the same size".to_string());
            }
        }

        let sprite_width = first_size.0;
        let sprite_height = first_size.1;
        let rows = ((sprites.len() as u32 + cols - 1) / cols).max(1);

        // スプリートシート用バッファを作成
        let sheet_width = sprite_width * cols;
        let sheet_height = sprite_height * rows;
        let mut sheet = image::ImageBuffer::new(sheet_width, sheet_height);

        // 各スプライトを配置
        for (idx, sprite) in sprites.iter().enumerate() {
            let col = (idx as u32) % cols;
            let row = (idx as u32) / cols;
            let offset_x = col * sprite_width;
            let offset_y = row * sprite_height;

            Self::composite_sprite(&mut sheet, sprite, offset_x, offset_y)?;
        }

        sheet
            .save(Path::new(path))
            .map_err(|e| format!("Spritesheet save failed: {}", e))?;

        Ok(())
    }

    /// スプリートシートにスプライトを合成
    fn composite_sprite(
        sheet: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        sprite: &SpriteData,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<(), String> {
        match &sprite.mode {
            crate::resource::ColorMode::Indexed256(palette) => {
                for y in 0..sprite.height {
                    for x in 0..sprite.width {
                        let idx = (y as usize * sprite.width as usize) + x as usize;
                        if idx < sprite.pixels.len() {
                            let color_idx = sprite.pixels[idx] as usize;
                            let color = palette.get(color_idx).copied().unwrap_or([0, 0, 0, 0]);
                            if let Some(pixel) =
                                sheet.get_pixel_mut_checked(offset_x + x, offset_y + y)
                            {
                                *pixel = image::Rgba(color);
                            }
                        }
                    }
                }
            }
            crate::resource::ColorMode::FullColor => {
                for y in 0..sprite.height {
                    for x in 0..sprite.width {
                        let offset = ((y as usize * sprite.width as usize) + x as usize) * 4;
                        if offset + 3 < sprite.pixels.len() {
                            let r = sprite.pixels[offset];
                            let g = sprite.pixels[offset + 1];
                            let b = sprite.pixels[offset + 2];
                            let a = sprite.pixels[offset + 3];
                            if let Some(pixel) =
                                sheet.get_pixel_mut_checked(offset_x + x, offset_y + y)
                            {
                                *pixel = image::Rgba([r, g, b, a]);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
