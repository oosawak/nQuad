//! 基本的なスプライト作成・描画の例
//!
//! このサンプルが示すこと：
//! - SpriteData を使った手動スプライト作成
//! - set_pixel() でのピクセル編集
//! - add_sprite() でエンジンに登録
//! - draw_sprite() での画面描画
//!
//! # 実行方法
//! ```bash
//! cargo run --example basic_sprite
//! ```

use macroquad::prelude::*;
use nantaraquad::{add_sprite, draw_sprite, ColorMode, SpriteData};

#[macroquad::main("Sprite Example")]
async fn main() {
    // 32x32 のフルカラースプライトを作成
    // ピクセルはすべて黒 [0, 0, 0, 0] で初期化される
    let mut sprite = SpriteData::new(32, 32, ColorMode::FullColor);

    // スプライト内に赤い正方形を描画
    // set_pixel() はスプライト内のピクセルを1つずつ編集
    for y in 5..27 {
        for x in 5..27 {
            let color = [255u8, 0, 0, 255]; // RGBA: 赤
            sprite.set_pixel(x as u32, y as u32, &color).ok();
        }
    }

    // スプライトをグローバルエンジンに追加
    // sprite_id は以降の描画操作で使用する ID
    let sprite_id = add_sprite(sprite);

    // メインループ
    loop {
        clear_background(BLACK);

        // スプライトを画面座標 (100, 100) に描画
        // ピクセルパーフェクト描画（FilterMode::Nearest が自動設定）
        draw_sprite(sprite_id, 100.0, 100.0);
        draw_text("Red Square Sprite", 100.0, 150.0, 20.0, WHITE);

        next_frame().await;
    }
}
