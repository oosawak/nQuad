//! Phase 7 Task 1 Verification Script
//! This script tests the Game Engine API implementation

use nantaraquad::engine::{SpriteAnimator, GameEntity, Scene, FrameInfo};
use nantaraquad::resource::asset::{SpriteAsset, AnimationClipDef, FrameDef, LayerDef, Cel};
use nantaraquad::resource::{SpriteData, ColorMode};
use std::sync::Arc;

fn create_test_asset() -> Arc<SpriteAsset> {
    let mut asset = SpriteAsset::new(1, "test");
    asset.add_layer_def(LayerDef::new(0, "layer0"));

    let mut frame0 = FrameDef::new(0, 100);
    let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
    frame0.add_cel(Cel::new(0, pixels));

    let mut frame1 = FrameDef::new(1, 150);
    let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
    frame1.add_cel(Cel::new(0, pixels));

    asset.set_frame(0, frame0);
    asset.set_frame(1, frame1);

    let clip = AnimationClipDef {
        name: "test_clip".to_string(),
        frames: vec![
            FrameDef::new(0, 100),
            FrameDef::new(1, 150),
        ],
        looping: true,
    };

    asset.add_animation_clip(clip);
    Arc::new(asset)
}

fn main() {
    println!("=== Phase 7 Task 1 Verification ===\n");

    // Test 1: SpriteAnimator with new methods
    println!("Test 1: SpriteAnimator.get_frame_info()");
    let asset = create_test_asset();
    let animator = SpriteAnimator::new(asset.clone());
    let frame_info = animator.get_frame_info();
    println!("  Frame Index: {}", frame_info.frame_index);
    println!("  Frame Count: {}", frame_info.frame_count);
    println!("  Clip Name: {}", frame_info.clip_name);
    println!("  Width: {}, Height: {}", frame_info.width, frame_info.height);
    assert_eq!(frame_info.frame_count, 2);
    println!("  ✓ PASSED\n");

    // Test 2: SpriteAnimator.render()
    println!("Test 2: SpriteAnimator.render()");
    let result = animator.render();
    assert!(result.is_ok());
    let sprite = result.unwrap();
    println!("  Rendered sprite: {}x{}", sprite.width, sprite.height);
    println!("  ✓ PASSED\n");

    // Test 3: GameEntity creation and management
    println!("Test 3: GameEntity creation");
    let entity = GameEntity::new(1, asset.clone());
    assert_eq!(entity.id, 1);
    assert_eq!(entity.position, (0.0, 0.0));
    assert!(entity.is_visible());
    println!("  Entity ID: {}", entity.id);
    println!("  Position: {:?}", entity.position);
    println!("  ✓ PASSED\n");

    // Test 4: GameEntity position management
    println!("Test 4: GameEntity position management");
    let mut entity = GameEntity::new(2, asset.clone());
    entity.set_position(100.0, 200.0);
    let pos = entity.get_position();
    assert_eq!(pos, (100.0, 200.0));
    println!("  Set position to: {:?}", pos);
    println!("  ✓ PASSED\n");

    // Test 5: Scene management with multiple entities
    println!("Test 5: Scene with multiple entities");
    let mut scene = Scene::new();
    let id1 = scene.add_entity(asset.clone());
    let id2 = scene.add_entity(asset.clone());
    
    assert_eq!(scene.entity_count(), 2);
    println!("  Added 2 entities");
    println!("  Entity 1 ID: {}", id1);
    println!("  Entity 2 ID: {}", id2);
    
    if let Some(entity) = scene.get_entity_mut(id1) {
        entity.play();
    }
    
    scene.update(50.0);
    let entities = scene.get_entity_ids();
    println!("  Entity count after update: {}", entities.len());
    assert_eq!(entities.len(), 2);
    println!("  ✓ PASSED\n");

    // Test 6: Arc<SpriteAsset> sharing
    println!("Test 6: Arc<SpriteAsset> sharing between entities");
    let mut scene = Scene::new();
    let id1 = scene.add_entity(asset.clone());
    let id2 = scene.add_entity(asset.clone());
    
    // Both entities share the same asset
    let entity1 = scene.get_entity(id1).unwrap();
    let entity2 = scene.get_entity(id2).unwrap();
    
    assert_eq!(entity1.animator.asset.id, entity2.animator.asset.id);
    println!("  Both entities reference the same asset");
    
    // But have independent playback states
    if let Some(e1) = scene.get_entity_mut(id1) {
        e1.play();
        e1.animator.update(100.0); // advance frame 0
    }
    
    let e2 = scene.get_entity(id2).unwrap();
    assert_eq!(e2.animator.current_frame_idx, 0); // entity2 not affected
    println!("  Independent playback states confirmed");
    println!("  ✓ PASSED\n");

    println!("=== All tests PASSED! ===");
}
