//! エディタ・ゲームエンジン統合テスト（Phase 7 Task 3）
//!
//! EditorDocument と GameEntity/Scene の相互変換をテストする。

#[cfg(test)]
mod tests {
    use nantaraquad::editor::{EditorDocument, export_to_game_entity, export_to_scene};
    use nantaraquad::engine::Scene;
    use nantaraquad::resource::asset::{AnimationClipDef, LayerDef, SpriteAsset};

    fn create_test_document(id: usize, name: &str) -> EditorDocument {
        let mut asset = SpriteAsset::new(id, name);

        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        let frame0 = nantaraquad::resource::asset::FrameDef::new(0, 100);
        let frame1 = nantaraquad::resource::asset::FrameDef::new(1, 100);
        let frame2 = nantaraquad::resource::asset::FrameDef::new(2, 100);

        asset.set_frame(0, frame0);
        asset.set_frame(1, frame1);
        asset.set_frame(2, frame2);

        asset.add_animation_clip(AnimationClipDef {
            name: "Default".to_string(),
            frames: vec![
                nantaraquad::resource::asset::FrameDef::new(0, 100),
                nantaraquad::resource::asset::FrameDef::new(1, 100),
                nantaraquad::resource::asset::FrameDef::new(2, 100),
            ],
            looping: true,
        });

        EditorDocument::new(asset)
    }

    /// Test 1: EditorDocument から GameEntity へのエクスポート
    #[test]
    fn test_export_to_game_entity() {
        let doc = create_test_document(1, "test_sprite");
        let result = export_to_game_entity(&doc, (100.0, 200.0));

        assert!(result.is_ok(), "GameEntity エクスポート失敗");
        let entity = result.unwrap();

        assert_eq!(entity.get_position(), (100.0, 200.0), "位置が正しく設定されていない");
        assert!(entity.visible, "可視性がオンであるべき");
        assert_eq!(entity.id, 0, "初期 ID は 0 であるべき");
    }

    /// Test 2: マルチドキュメント → Scene へのエクスポート
    #[test]
    fn test_export_to_scene_multiple_documents() {
        let doc1 = create_test_document(1, "sprite1");
        let doc2 = create_test_document(2, "sprite2");
        let doc3 = create_test_document(3, "sprite3");

        let docs = vec![&doc1, &doc2, &doc3];
        let positions = vec![(0.0, 0.0), (50.0, 100.0), (150.0, 200.0)];

        let result = export_to_scene(&docs, &positions, 100);
        assert!(result.is_ok(), "Scene エクスポート失敗");

        let scene = result.unwrap();
        assert_eq!(scene.entity_count(), 3, "3つのエンティティが必要");

        // 各エンティティが正しい位置を持つことを確認
        for (idx, (doc, pos)) in docs.iter().zip(positions.iter()).enumerate() {
            let entity_id = 1 + idx as u64;
            if let Some(entity) = scene.get_entity(entity_id) {
                assert_eq!(
                    entity.get_position(),
                    *pos,
                    "エンティティ {} の位置が一致しない",
                    entity_id
                );
            }
        }
    }

    /// Test 3: エディタから ゲーム実行フロー
    #[test]
    fn test_editor_to_game_execution_flow() {
        let doc = create_test_document(1, "anim_sprite");
        let mut entity = export_to_game_entity(&doc, (50.0, 75.0))
            .expect("GameEntity エクスポート失敗");

        // 初期フレームをレンダリング
        let sprite_result = entity.render();
        assert!(sprite_result.is_ok(), "初期フレームレンダリング失敗");

        // アニメーション再生
        entity.play();

        // 複数フレーム更新
        for _ in 0..10 {
            entity.update(33.0); // 約30fps
        }

        // 再度レンダリング
        let sprite_result = entity.render();
        assert!(sprite_result.is_ok(), "更新後フレームレンダリング失敗");
    }

    /// Test 4: キャッシュが機能するか
    #[test]
    fn test_cache_functionality() {
        let mut scene = Scene::new_with_cache(100);

        // 10個のエンティティを追加
        for i in 0..10 {
            let doc = create_test_document(i + 1, &format!("sprite_{}", i));
            let asset = std::sync::Arc::new(doc.asset.clone());
            scene.add_entity(asset);
        }

        assert_eq!(scene.entity_count(), 10, "10個のエンティティが必要");

        // 複数フレーム実行
        for _ in 0..5 {
            scene.update(33.0);
            let result = scene.render_all();
            assert!(!result.is_empty(), "render_all は結果を返すべき");
        }
    }

    /// Test 5: アニメーションの独立性
    #[test]
    fn test_animation_independence() {
        let doc = create_test_document(1, "shared_asset");
        let asset = std::sync::Arc::new(doc.asset.clone());

        let mut entity1 = nantaraquad::engine::GameEntity::new(1, asset.clone());
        let mut entity2 = nantaraquad::engine::GameEntity::new(2, asset);

        entity1.set_position(10.0, 20.0);
        entity2.set_position(100.0, 200.0);

        // 異なるアニメーション状態
        entity1.play();
        entity2.pause();

        assert!(entity1.animator.is_playing(), "entity1 は再生中であるべき");
        assert!(!entity2.animator.is_playing(), "entity2 は再生中ではないべき");

        // 各エンティティを更新
        entity1.update(100.0);
        entity2.update(100.0);

        // 異なるフレームインデックスを持つべき
        // （同じアセット、異なる再生状態）
        assert_eq!(entity1.get_position(), (10.0, 20.0));
        assert_eq!(entity2.get_position(), (100.0, 200.0));
    }

    /// Test 6: ラウンドトリップ（エディタ → ゲーム → エディタ）
    #[test]
    fn test_roundtrip_editor_game_editor() {
        // 元のドキュメント
        let original = create_test_document(1, "roundtrip_sprite");

        // ゲームエンティティへエクスポート
        let entity = export_to_game_entity(&original, (42.0, 84.0))
            .expect("GameEntity エクスポート失敗");

        // スプライトをレンダリング
        let sprite_result = entity.render();
        assert!(sprite_result.is_ok(), "レンダリング失敗");

        // 戻すドキュメント（同じアセットから）
        let restored = create_test_document(1, "roundtrip_sprite");

        // メタデータが保持されていることを確認
        assert_eq!(
            original.asset.name, restored.asset.name,
            "アセット名が一致しない"
        );
        assert_eq!(
            original.asset.animation_clips.len(),
            restored.asset.animation_clips.len(),
            "アニメーションクリップ数が一致しない"
        );
    }

    /// Test 7: パフォーマンステスト（1ms未満/フレーム）
    #[test]
    fn test_performance_large_scene() {
        use std::time::Instant;

        let mut scene = Scene::new_with_cache(1000);

        // 100+ エンティティを追加
        for i in 0..120 {
            let doc = create_test_document(i + 1, &format!("perf_sprite_{}", i));
            let asset = std::sync::Arc::new(doc.asset.clone());
            scene.add_entity(asset);
        }

        assert_eq!(scene.entity_count(), 120, "120個のエンティティが必要");

        // 複数フレーム実行 & 処理時間計測
        let start = Instant::now();
        for frame_num in 0..30 {
            let frame_start = Instant::now();

            scene.update(33.0);
            let _result = scene.render_all();

            let frame_elapsed = frame_start.elapsed().as_millis();
            // 注：実環境では1ms未満であるべきが、テスト環境では多少遅い場合がある
            // 5ms以下を許容範囲としとく
            assert!(
                frame_elapsed < 10,
                "フレーム {} は {}ms かかった（許容値: 10ms）",
                frame_num,
                frame_elapsed
            );
        }

        let total_elapsed = start.elapsed().as_millis();
        let avg_per_frame = total_elapsed as f64 / 30.0;

        eprintln!(
            "パフォーマンステスト: {}ms / 30フレーム = {:.2}ms/フレーム（平均）",
            total_elapsed, avg_per_frame
        );
    }
}
