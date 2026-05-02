// パフォーマンステスト：キャッシング機能の効果検証

#[cfg(test)]
mod performance_tests {
    use nantaraquad::engine::{CompositeCache, CacheKey, SpriteAnimator, Scene, GameEntity};
    use nantaraquad::resource::asset::{AnimationClipDef, Cel, FrameDef, LayerDef, SpriteAsset};
    use nantaraquad::resource::{ColorMode, SpriteData};
    use std::sync::Arc;
    use std::time::Instant;

    fn create_test_asset() -> Arc<SpriteAsset> {
        let mut asset = SpriteAsset::new(1, "test");

        // レイヤー定義を追加
        asset.add_layer_def(LayerDef::new(0, "layer0"));

        // フレームを作成（100個のフレーム）
        for i in 0..100 {
            let mut frame = FrameDef::new(i as u32, 100);
            let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
            frame.add_cel(Cel::new(0, pixels));
            asset.set_frame(i, frame);
        }

        // アニメーションクリップを作成
        let frames = (0..100)
            .map(|i| FrameDef::new(i as u32, 100))
            .collect();

        let clip = AnimationClipDef {
            name: "test_clip".to_string(),
            frames,
            looping: true,
        };

        asset.add_animation_clip(clip);

        Arc::new(asset)
    }

    #[test]
    #[ignore] // 手動実行用
    fn test_cache_performance_comparison() {
        let asset = create_test_asset();
        let num_renders = 10000;

        // Test 1: キャッシング有効
        {
            println!("\nTest: キャッシング有効 (同じフレームの render を {} 回)", num_renders);
            let animator = SpriteAnimator::new(asset.clone());

            let start = Instant::now();
            for _ in 0..num_renders {
                let _ = animator.render();
            }
            let elapsed = start.elapsed();

            let stats = animator.get_cache_stats();
            println!("  結果:");
            println!("    - ヒット数: {}", stats.hits);
            println!("    - ミス数: {}", stats.misses);
            println!("    - キャッシュヒット率: {:.2}%", (stats.hits as f64 / (stats.hits + stats.misses) as f64) * 100.0);
            println!("    - 所要時間: {:?}", elapsed);
            println!("    - 1回当たり: {:?}", elapsed / num_renders as u32);
        }

        // Test 2: キャッシング無効（リセット後に異なるフレームを render）
        {
            println!("\nTest: キャッシング無効 (異なるフレームの render を複数回)");
            let asset = create_test_asset();
            let mut animator = SpriteAnimator::new(asset);

            let start = Instant::now();
            for i in 0..100 {
                animator.set_frame(i);
                let _ = animator.render();
            }
            let elapsed = start.elapsed();

            let stats = animator.get_cache_stats();
            println!("  結果:");
            println!("    - ヒット数: {}", stats.hits);
            println!("    - ミス数: {}", stats.misses);
            println!("    - キャッシュヒット率: {:.2}%", (stats.hits as f64 / (stats.hits + stats.misses) as f64) * 100.0);
            println!("    - 所要時間: {:?}", elapsed);
        }

        // Test 3: 複数エンティティでのキャッシング効果
        {
            println!("\nTest: 複数エンティティ (10エンティティ × 1000 render)");
            let mut scene = Scene::new_with_cache(2048);
            let asset = create_test_asset();

            // 10個のエンティティを追加
            for _ in 0..10 {
                scene.add_entity(asset.clone());
            }

            let start = Instant::now();
            for _ in 0..1000 {
                let _ = scene.render_all();
            }
            let elapsed = start.elapsed();

            let stats = scene.get_cache_stats();
            println!("  結果:");
            println!("    - 全エンティティキャッシュヒット数: {}", stats.hits);
            println!("    - 全エンティティキャッシュミス数: {}", stats.misses);
            println!("    - 所要時間: {:?}", elapsed);
            println!("    - 1回当たり: {:?}", elapsed / 1000 as u32);
        }
    }

    #[test]
    fn test_cache_effectiveness() {
        let asset = create_test_asset();

        // 同じフレームを複数回 render
        let animator = SpriteAnimator::new(asset);

        // 最初の render（キャッシュミス）
        let _ = animator.render();
        let stats_1 = animator.get_cache_stats();
        assert_eq!(stats_1.misses, 1);
        assert_eq!(stats_1.hits, 0);

        // 2回目の render（キャッシュヒット）
        let _ = animator.render();
        let stats_2 = animator.get_cache_stats();
        assert_eq!(stats_2.misses, 1);
        assert_eq!(stats_2.hits, 1);

        // 以後のすべての render がキャッシュヒット
        for _ in 0..98 {
            let _ = animator.render();
        }
        let stats_final = animator.get_cache_stats();
        assert_eq!(stats_final.misses, 1);
        assert_eq!(stats_final.hits, 99);

        println!("✓ キャッシング有効: ヒット率 {:.2}%", 
            (99.0 / 100.0) * 100.0);
    }
}
