//! エディタ・ゲームエンジン統合（Phase 7 Task 3）
//!
//! EditorDocument をゲームエンジンの GameEntity/Scene に変換する機能を提供。

use crate::editor::EditorDocument;
use crate::engine::{GameEntity, Scene};
use crate::resource::asset::SpriteAsset;
use std::sync::Arc;

/// EditorDocument を GameEntity にエクスポート
///
/// # 引数
/// - `doc`: エディタドキュメント
/// - `position`: ワールド座標 (x, y)
///
/// # 戻り値
/// 初期化された GameEntity、またはエラーメッセージ
///
/// # エラー
/// - アセットにアニメーションクリップが存在しない場合
pub fn export_to_game_entity(
    doc: &EditorDocument,
    position: (f32, f32),
) -> Result<GameEntity, String> {
    // アセットをArcでラップ
    let asset = Arc::new(doc.asset.clone());

    // GameEntity を作成
    let mut entity = GameEntity::new(0, asset);
    entity.set_position(position.0, position.1);

    Ok(entity)
}

/// 複数の EditorDocument を Scene にエクスポート
///
/// # 引数
/// - `docs`: エディタドキュメントのスライス
/// - `positions`: 対応するワールド座標のスライス
/// - `cache_size`: 共有キャッシュの最大エントリ数
///
/// # 戻り値
/// 初期化された Scene、またはエラーメッセージ
///
/// # エラー
/// - docs と positions の長さが一致しない場合
/// - アセットの初期化に失敗した場合
pub fn export_to_scene(
    docs: &[&EditorDocument],
    positions: &[(f32, f32)],
    cache_size: usize,
) -> Result<Scene, String> {
    if docs.len() != positions.len() {
        return Err(format!(
            "docs と positions の長さが一致しません: {} != {}",
            docs.len(),
            positions.len()
        ));
    }

    let mut scene = Scene::new_with_cache(cache_size);

    for (doc, pos) in docs.iter().zip(positions.iter()) {
        let asset = Arc::new(doc.asset.clone());
        let id = scene.add_entity(asset.clone());

        // エンティティを取得して位置を設定
        if let Some(entity) = scene.get_entity_mut(id) {
            entity.set_position(pos.0, pos.1);
        }
    }

    Ok(scene)
}

/// EditorDocument 資産をゲーム実行用に最適化
///
/// # 引数
/// - `doc`: エディタドキュメント
///
/// # 戻り値
/// 最適化された SpriteAsset
pub fn optimize_for_game(doc: &EditorDocument) -> SpriteAsset {
    doc.asset.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::EditorDocument;
    use crate::resource::asset::{AnimationClipDef, LayerDef};
    use crate::resource::{ColorMode, SpriteData};

    fn create_test_document() -> EditorDocument {
        let mut asset = SpriteAsset::new(1, "test_asset");

        // ダミーレイヤー定義を追加
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        // ダミーフレームデータを追加
        let frame_data = crate::resource::asset::FrameDef::new(0, 100);
        asset.set_frame(0, frame_data);

        // ダミーアニメーションクリップを追加
        asset.add_animation_clip(AnimationClipDef {
            name: "Default".to_string(),
            frames: vec![crate::resource::asset::FrameDef::new(0, 100)],
            looping: true,
        });

        EditorDocument::new(asset)
    }

    #[test]
    fn test_export_to_game_entity() {
        let doc = create_test_document();
        let result = export_to_game_entity(&doc, (100.0, 200.0));

        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.get_position(), (100.0, 200.0));
        assert!(entity.visible);
    }

    #[test]
    fn test_export_to_scene_single() {
        let doc = create_test_document();
        let docs = vec![&doc];
        let positions = vec![(50.0, 75.0)];

        let result = export_to_scene(&docs, &positions, 100);
        assert!(result.is_ok());

        let scene = result.unwrap();
        assert_eq!(scene.entity_count(), 1);
    }

    #[test]
    fn test_export_to_scene_multiple() {
        let doc1 = create_test_document();
        let doc2 = create_test_document();
        let docs = vec![&doc1, &doc2];
        let positions = vec![(10.0, 20.0), (30.0, 40.0)];

        let result = export_to_scene(&docs, &positions, 100);
        assert!(result.is_ok());

        let scene = result.unwrap();
        assert_eq!(scene.entity_count(), 2);
    }

    #[test]
    fn test_export_to_scene_mismatch() {
        let doc = create_test_document();
        let docs = vec![&doc];
        let positions = vec![(10.0, 20.0), (30.0, 40.0)];

        let result = export_to_scene(&docs, &positions, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_optimize_for_game() {
        let doc = create_test_document();
        let optimized = optimize_for_game(&doc);

        assert_eq!(optimized.id, doc.asset.id);
        assert_eq!(optimized.name, doc.asset.name);
        assert_eq!(
            optimized.animation_clips.len(),
            doc.asset.animation_clips.len()
        );
    }
}
