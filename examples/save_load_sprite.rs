//! ディスク保存・読み込みとリアルタイムピクセル操作の例
//!
//! このサンプルが示すこと：
//! - ResourcePackage を使ったスプライトのディスク保存
//! - load_sprite() でのディスク読み込み
//! - set_pixel() でのリアルタイムピクセル編集（自動GPU同期）
//! - 動的なパターン生成
//!
//! # 実行方法
//! ```bash
//! cargo run --example save_load_sprite
//! ```

use macroquad::prelude::*;
use nantaraquad::{
    draw_sprite, load_sprite, resource::serialize::save_package, set_pixel, ColorMode,
    ResourcePackage, SpriteData,
};
use std::path::Path;

#[macroquad::main("Pixel Manipulation Example")]
async fn main() {
    // スプライトを作成してディスクに保存
    // /tmp は Linux/macOS ではテンポラリディレクトリ
    // Windows の場合は適切なパスに変更してください
    let sprite_path = "/tmp/nantaraquad_sprite.bin";
    create_and_save_sprite(sprite_path);

    // ディスクからスプライトを読み込み
    // ResourcePackage は bincode で保存され、復元時に自動的にエンジンに登録される
    match load_sprite(sprite_path) {
        Ok(sprite_id) => {
            let mut frame_count = 0u32;

            // メインループ
            loop {
                clear_background(BLACK);

                // フレームごとにピクセルを動的に更新
                // set_pixel() は自動的に GPU テクスチャに同期される
                if frame_count % 10 == 0 {
                    let x = (frame_count / 10) % 32;
                    let y = (frame_count / 10) / 32;
                    if x < 32 && y < 32 {
                        // RGB: x と y に基づいた色を計算
                        let color = [(x * 8) as u8, (y * 8) as u8, 128u8, 255u8];
                        set_pixel(sprite_id, x as u32, y as u32, &color).ok();
                    }
                }

                draw_sprite(sprite_id, 100.0, 100.0);
                draw_text("Dynamic Pixel Update", 100.0, 150.0, 20.0, WHITE);
                draw_text(
                    &format!("Frame: {}", frame_count),
                    100.0,
                    180.0,
                    20.0,
                    WHITE,
                );

                frame_count += 1;
                next_frame().await;

                // デモンストレーション用に300フレーム後に終了
                if frame_count > 300 {
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load sprite: {}", e);
        }
    }

    // クリーンアップ: 一時ファイルを削除
    std::fs::remove_file(sprite_path).ok();
}

/// スプライトを作成してディスクに保存するヘルパー関数
///
/// グラデーション効果を持つ 32x32 FullColor スプライトを
/// ResourcePackage として bincode で保存します。
fn create_and_save_sprite(path: &str) {
    // グラデーション用スプライト作成
    let mut sprite = SpriteData::new(32, 32, ColorMode::FullColor);

    // X/Y座標に応じた RGB グラデーションを描画
    for y in 0..32 {
        for x in 0..32 {
            let r = (x * 8) as u8;
            let g = (y * 8) as u8;
            let b = 128u8;
            sprite.set_pixel(x as u32, y as u32, &[r, g, b, 255]).ok();
        }
    }

    // スプライトをリソースパッケージに追加
    let mut pkg = ResourcePackage::new();
    pkg.add_sprite(sprite);

    // パッケージをディスクに保存（bincode形式）
    save_package(&pkg, Path::new(path)).ok();
}
