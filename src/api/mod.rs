use crate::core::get_engine;
use crate::resource::{ColorMode, SpriteData};
use std::path::Path;

/// スプライトを指定位置に描画
///
/// グローバルエンジンのスプライトを描画します。
/// 内部的には Engine::draw_sprite() を呼び出す薄いラッパーです。
///
/// # 引数
/// - `id`: スプライト ID（add_sprite() の戻り値）
/// - `x`: スクリーン上の X座標
/// - `y`: スクリーン上の Y座標
///
/// # 例
/// ```ignore
/// draw_sprite(sprite_id, 100.0, 50.0);
/// ```
pub fn draw_sprite(id: usize, x: f32, y: f32) {
    let engine = get_engine();
    let _ = engine.draw_sprite(id, x, y);
}

/// スプライトをスケール付きで描画
///
/// # 引数
/// - `id`: スプライト ID
/// - `x`: スクリーン上の X座標
/// - `y`: スクリーン上の Y座標
/// - `scale`: スケール倍率（1.0 = 等倍、2.0 = 2倍、0.5 = 半分）
///
/// # 例
/// ```ignore
/// draw_sprite_scaled(sprite_id, 100.0, 50.0, 2.5);
/// ```
pub fn draw_sprite_scaled(id: usize, x: f32, y: f32, scale: f32) {
    let engine = get_engine();
    let _ = engine.draw_sprite_scaled(id, x, y, scale);
}

/// スプライトの指定ピクセルを変更（自動同期）
///
/// CPU上のピクセルを変更し、自動的にGPUテクスチャに反映します。
/// ピクセル値はカラーモードに応じて異なります：
/// - **Indexed256**: 1 バイト（パレットインデックス 0-255）
/// - **FullColor**: 4 バイト（RGBA）
///
/// # 引数
/// - `sprite_id`: スプライト ID
/// - `x`: X座標（0 から幅-1）
/// - `y`: Y座標（0 から高さ-1）
/// - `pixel_data`: ピクセル値
///
/// # 戻り値
/// - `Ok(())`: 成功
/// - `Err(String)`: スプライト ID が無効、座標範囲外、ピクセル値サイズ不正
///
/// # 例
/// ```ignore
/// // FullColor スプライト
/// set_pixel(sprite_id, 10, 10, &[255, 0, 0, 255])?; // 赤
///
/// // Indexed256 スプライト
/// set_pixel(sprite_id, 10, 10, &[2])?; // パレットインデックス 2
/// ```
pub fn set_pixel(sprite_id: usize, x: u32, y: u32, pixel_data: &[u8]) -> Result<(), String> {
    let mut engine = get_engine();
    engine.set_pixel(sprite_id, x, y, pixel_data)
}

/// スプライトの指定ピクセルを読み取る
///
/// # 引数
/// - `sprite_id`: スプライト ID
/// - `x`: X座標
/// - `y`: Y座標
///
/// # 戻り値
/// - `Some(Vec<u8>)`: ピクセル値
/// - `None`: スプライト ID が無効または座標が範囲外
pub fn get_pixel(sprite_id: usize, x: u32, y: u32) -> Option<Vec<u8>> {
    let engine = get_engine();
    engine.get_pixel(sprite_id, x, y)
}

/// スプライトをグローバルエンジンに追加
///
/// CPU側のスプライトをエンジンに登録し、自動的にGPUテクスチャに同期します。
/// テクスチャフィルタは FilterMode::Nearest に設定されます。
///
/// # 引数
/// - `sprite`: 追加するスプライトデータ
///
/// # 戻り値
/// 割り当てられたスプライト ID（以降の操作で使用）
///
/// # 例
/// ```ignore
/// let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
/// let sprite_id = add_sprite(sprite);
/// ```
pub fn add_sprite(sprite: SpriteData) -> usize {
    let mut engine = get_engine();
    engine.add_sprite(sprite)
}

/// フルカラースプライトを新規作成して登録
///
/// FullColor モードで新規スプライトを作成し、エンジンに登録します。
/// ピクセルはすべて黒（`[0, 0, 0, 0]`）で初期化されます。
///
/// # 引数
/// - `width`: スプライトの幅（ピクセル）
/// - `height`: スプライトの高さ（ピクセル）
///
/// # 戻り値
/// 割り当てられたスプライト ID
///
/// # 例
/// ```ignore
/// let sprite_id = create_sprite(32, 32);
/// set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?; // 赤を設定
/// ```
pub fn create_sprite(width: u32, height: u32) -> usize {
    let sprite = SpriteData::new(width, height, ColorMode::FullColor);
    add_sprite(sprite)
}

/// インデックスカラースプライトを新規作成して登録
///
/// Indexed256 モードで新規スプライトを作成し、エンジンに登録します。
/// パレットを指定する必要があります。
///
/// # 引数
/// - `width`: スプライトの幅（ピクセル）
/// - `height`: スプライトの高さ（ピクセル）
/// - `palette`: RGBA カラーパレット（最大256色）
///
/// # 戻り値
/// 割り当てられたスプライト ID
///
/// # 例
/// ```ignore
/// let palette = vec![
///     [0, 0, 0, 255],       // インデックス 0: 黒
///     [255, 0, 0, 255],     // インデックス 1: 赤
/// ];
/// let sprite_id = create_indexed_sprite(16, 16, palette);
/// set_pixel(sprite_id, 0, 0, &[1])?; // 赤を設定
/// ```
pub fn create_indexed_sprite(width: u32, height: u32, palette: Vec<[u8; 4]>) -> usize {
    let sprite = SpriteData::new(width, height, ColorMode::Indexed256(palette));
    add_sprite(sprite)
}

/// ディスクからスプライトパッケージを読み込み
///
/// bincode 形式で保存されたリソースパッケージを読み込み、
/// 含まれるすべてのスプライトをエンジンに登録します。
///
/// # 引数
/// - `path`: ファイルパス（例: `"/tmp/sprites.bin"`）
///
/// # 戻り値
/// - `Ok(usize)`: 読み込んだ最初のスプライトの ID
/// - `Err(String)`: ファイルが見つからない、パース失敗、スプライトが空など
///
/// # 例
/// ```ignore
/// match load_sprite("/tmp/my_sprites.bin") {
///     Ok(sprite_id) => draw_sprite(sprite_id, 100.0, 100.0),
///     Err(e) => eprintln!("Failed to load: {}", e),
/// }
/// ```
pub fn load_sprite(path: &str) -> Result<usize, String> {
    let mut engine = get_engine();
    let pkg = crate::resource::serialize::load_package_safe(Path::new(path))?;

    if pkg.sprites.is_empty() {
        return Err("No sprites in package".to_string());
    }

    let start_id = engine.res.sprites.len();
    for sprite in pkg.sprites {
        engine.add_sprite(sprite);
    }

    Ok(start_id)
}

/// スプライト数を取得
///
/// グローバルエンジンに登録されているスプライトの総数を返します。
///
/// # 戻り値
/// スプライト数
///
/// # 例
/// ```ignore
/// println!("Sprites: {}", sprite_count());
/// ```
pub fn sprite_count() -> usize {
    let engine = get_engine();
    engine.sprite_count()
}

// ===== pyxel 互換 API =====
pub mod drawing;
pub mod input;
pub mod camera;
pub mod game;
pub mod pyxel_compat;
pub mod framework;
pub mod audio_compat;
pub mod pyxel;
pub mod particles;
pub mod introspect;

// エクスポート
pub use drawing::{DrawingContext, PYXEL_PALETTE};
pub use input::{InputState, Key};
pub use camera::Camera;
pub use game::{GameEngine, Scene};
pub use framework::{GameApp, GameRunner};
pub use audio_compat::PyxelAudio;
pub use particles::{Particle, ParticleSystem};
pub use introspect::{ApiReference, ApiFunction, Parameter, ReturnType, build_api_reference};
pub use pyxel::{
    // Drawing
    cls, pset, pget, line, rect, rectfill, circle, circfill,
    // Sprite
    spr,
    // Input
    btn, btnp,
    // Camera
    camera, zoom,
    // Audio
    sfx, music, stop, music_stop,
    // Color
    set_palette,
    // Framework
    frame_time, frames_for_ms,
    // Animation
    spr_anim, anim_update, anim_set_frame, anim_get_frame,
    // Text & Debug
    print, stat,
};
