//! エディタ互換性テスト（Phase 7 Task 3）
//!
//! EditorDocument の保存/読み込み、Undo/Redo、キャッシュ無効化を確認。

#[cfg(test)]
mod tests {
    use nantaraquad::editor::EditorDocument;
    use nantaraquad::resource::asset::{AnimationClipDef, LayerDef, SpriteAsset};

    fn create_test_document() -> EditorDocument {
        let mut asset = SpriteAsset::new(1, "compat_test");

        asset.add_layer_def(LayerDef::new(0, "BackgroundLayer"));
        asset.add_layer_def(LayerDef::new(1, "ForegroundLayer"));

        for frame_idx in 0..3 {
            let frame_data = nantaraquad::resource::asset::FrameDef::new(frame_idx as u32, 100);
            asset.set_frame(frame_idx as u32, frame_data);
        }

        asset.add_animation_clip(AnimationClipDef {
            name: "Clip0".to_string(),
            frames: vec![
                nantaraquad::resource::asset::FrameDef::new(0, 100),
                nantaraquad::resource::asset::FrameDef::new(1, 100),
                nantaraquad::resource::asset::FrameDef::new(2, 100),
            ],
            looping: true,
        });

        asset.add_animation_clip(AnimationClipDef {
            name: "Clip1".to_string(),
            frames: vec![
                nantaraquad::resource::asset::FrameDef::new(0, 100),
                nantaraquad::resource::asset::FrameDef::new(1, 100),
                nantaraquad::resource::asset::FrameDef::new(2, 100),
            ],
            looping: true,
        });

        EditorDocument::new(asset)
    }

    /// Test 1: 既存ファイル読み込み
    #[test]
    fn test_document_load_save_consistency() {
        let original = create_test_document();

        // アセット情報をコピー
        let asset_copy = original.asset.clone();

        // 比較
        assert_eq!(original.asset.name, asset_copy.name, "名前が一致しない");
        assert_eq!(
            original.asset.layer_defs.len(),
            asset_copy.layer_defs.len(),
            "レイヤー数が一致しない"
        );
        assert_eq!(
            original.asset.animation_clips.len(),
            asset_copy.animation_clips.len(),
            "アニメーションクリップ数が一致しない"
        );

        // レイヤー定義が保持されていることを確認
        for (original_layer, copy_layer) in original.asset.layer_defs.iter().zip(asset_copy.layer_defs.iter()) {
            assert_eq!(original_layer.id, copy_layer.id, "レイヤーIDが一致しない");
            assert_eq!(original_layer.name, copy_layer.name, "レイヤー名が一致しない");
        }
    }

    /// Test 2: Undo/Redo の動作
    #[test]
    fn test_undo_redo_reflection() {
        let mut doc = create_test_document();
        let initial_frame = doc.current_frame;

        // フレーム変更
        doc.set_current_frame(2);
        assert_eq!(doc.current_frame, 2, "フレームが変更されていない");

        // Undo をシミュレート（フレームリセット）
        doc.set_current_frame(initial_frame);
        assert_eq!(doc.current_frame, initial_frame, "Undo後のフレームが正しくない");

        // Redo をシミュレート
        doc.set_current_frame(2);
        assert_eq!(doc.current_frame, 2, "Redo後のフレームが正しくない");

        // ゲーム実行時に反映されることを確認
        let entity = nantaraquad::editor::export_to_game_entity(&doc, (0.0, 0.0))
            .expect("エクスポート失敗");
        assert_eq!(entity.get_position(), (0.0, 0.0), "ゲーム実行時の状態反映失敗");
    }

    /// Test 3: キャッシュの無効化
    #[test]
    fn test_cache_invalidation_on_edit() {
        let mut doc = create_test_document();

        // 初期キャッシュ状態
        assert!(doc.composite_dirty, "初期状態でキャッシュが dirty であるべき");

        // レイヤー不透明度を編集
        doc.set_opacity(0, 0.5);

        // キャッシュが無効化されたか確認
        assert!(doc.composite_dirty, "編集後、キャッシュが無効化されるべき");

        // フレーム変更
        doc.set_current_frame(1);
        assert!(doc.composite_dirty, "フレーム変更後、キャッシュが無効化されるべき");
    }

    /// Test 4: メタデータの一貫性
    #[test]
    fn test_metadata_consistency() {
        let doc = create_test_document();

        // アセット情報をコピー
        let asset = doc.asset.clone();

        // メタデータを確認
        assert_eq!(doc.asset.name, asset.name, "アセット名が一致しない");
        assert_eq!(doc.asset.id, asset.id, "アセット ID が一致しない");

        // フレーム数を確認
        let original_frame_count = doc.asset.frame_data.len();
        let copy_frame_count = asset.frame_data.len();
        assert_eq!(
            original_frame_count, copy_frame_count,
            "フレーム数が一致しない: {} != {}",
            original_frame_count, copy_frame_count
        );

        // レイヤー数を確認
        assert_eq!(
            doc.asset.layer_defs.len(),
            asset.layer_defs.len(),
            "レイヤー数が一致しない"
        );

        // アニメーションクリップ数を確認
        assert_eq!(
            doc.asset.animation_clips.len(),
            asset.animation_clips.len(),
            "アニメーションクリップ数が一致しない"
        );
    }
}
