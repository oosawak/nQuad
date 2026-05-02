//! DocumentFormat v2 の to_format/from_format テスト

use nantaraquad::resource::{SpriteAsset, LayerDef, FrameDef, Cel, SpriteData, ColorMode, AnimationClipDef};
use std::collections::HashMap;

fn main() {
    // テスト: SpriteAsset → DocumentFormat → SpriteAsset の往復変換

    // 1. テスト用 SpriteAsset を作成
    let mut asset = SpriteAsset::new(0, "TestAsset");
    
    // レイヤーを追加
    let layer = LayerDef::new(0, "Layer 0");
    asset.add_layer_def(layer);
    
    // フレームを作成
    let mut frame = FrameDef::new(0, 100);
    let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
    let cel = Cel::new(0, pixels);
    frame.add_cel(cel);
    asset.set_frame(0, frame);
    
    // アニメーションクリップを追加
    let mut frame_for_clip = FrameDef::new(0, 100);
    frame_for_clip.add_cel(Cel::new(0, SpriteData::new(32, 32, ColorMode::FullColor)));
    let clip = AnimationClipDef {
        name: "Default".to_string(),
        frames: vec![frame_for_clip],
        looping: true,
    };
    asset.add_animation_clip(clip);

    // 2. DocumentFormat に変換
    let format = asset.to_format();
    println!("✓ to_format() succeeded");
    println!("  - Name: {}", format.metadata.name);
    println!("  - Width: {}, Height: {}", format.metadata.width, format.metadata.height);
    println!("  - Frame count: {}", format.metadata.frame_count);
    println!("  - Layers: {}", format.layers.len());
    println!("  - Clips: {}", format.clips.len());
    println!("  - Cel data: {}", format.cel_data.len());

    // 3. DocumentFormat から SpriteAsset に復元
    let restored = SpriteAsset::from_format(&format)
        .expect("Failed to restore from format");
    println!("✓ from_format() succeeded");
    println!("  - Name: {}", restored.name);
    println!("  - Layers: {}", restored.layer_defs.len());
    println!("  - Clips: {}", restored.animation_clips.len());
    println!("  - Frames: {}", restored.frame_data.len());

    // 4. 検証
    assert_eq!(asset.name, restored.name, "Name mismatch");
    assert_eq!(asset.layer_defs.len(), restored.layer_defs.len(), "Layer count mismatch");
    assert_eq!(asset.animation_clips.len(), restored.animation_clips.len(), "Clip count mismatch");
    assert_eq!(asset.frame_data.len(), restored.frame_data.len(), "Frame count mismatch");
    
    println!("\n✅ All checks passed!");
}
