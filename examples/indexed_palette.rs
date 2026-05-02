//! インデックスカラーパレット（256色）の例
//!
//! このサンプルが示すこと：
//! - Indexed256 モードでのスプライト作成
//! - カスタムカラーパレットの定義
//! - パレットインデックスを使ったパターン描画
//! - メモリ効率の良い色管理
//!
//! # 実行方法
//! ```bash
//! cargo run --example indexed_palette
//! ```

use macroquad::prelude::*;
use nantaraquad::{add_sprite, draw_sprite, ColorMode, SpriteData};

#[macroquad::main("Indexed Palette Example")]
async fn main() {
    // 4色のシンプルなカラーパレットを定義
    // インデックス値が 0-255 の配列添字として機能
    let palette = vec![
        [0, 0, 0, 255],   // インデックス 0: 黒
        [255, 0, 0, 255], // インデックス 1: 赤
        [0, 255, 0, 255], // インデックス 2: 緑
        [0, 0, 255, 255], // インデックス 3: 青
    ];

    // 32x32 のインデックスカラースプライトを作成
    // Indexed256 モードではピクセルが 1 バイト/個
    // (FullColor は 4 バイト/個 なので、メモリを 1/4 に削減)
    let mut sprite = SpriteData::new(32, 32, ColorMode::Indexed256(palette));

    // チェッカーボードパターンをパレットインデックスで描画
    for y in 0..32 {
        for x in 0..32 {
            // (x + y) % 4 で 0-3 の値を算出 → パレットインデックスに
            let idx = ((x + y) % 4) as u8;
            sprite.set_pixel(x as u32, y as u32, &[idx]).ok();
        }
    }

    // スプライトをエンジンに追加
    let sprite_id = add_sprite(sprite);

    // メインループ
    loop {
        clear_background(BLACK);

        // インデックスカラーのスプライトを描画
        draw_sprite(sprite_id, 100.0, 100.0);
        draw_text("Indexed Palette Sprite", 100.0, 150.0, 20.0, WHITE);

        next_frame().await;
    }
}
