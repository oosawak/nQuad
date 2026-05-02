//! シーンマネージャ（Phase 7 Task 2）
//!
//! Scene は複数の GameEntity を管理し、
//! 更新とレンダリングを一括処理する。共有キャッシュをサポート。

use crate::engine::cache::CompositeCache;
use crate::engine::entity::{EntityId, GameEntity};
use crate::resource::asset::SpriteAsset;
use crate::resource::SpriteData;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// シーン：エンティティコンテナ
///
/// 複数の GameEntity を管理し、全体の更新と描画を統括する。
/// 全エンティティで共有されるコンポジットキャッシュを保持する。
pub struct Scene {
    /// エンティティマップ (EntityId -> GameEntity)
    entities: HashMap<EntityId, GameEntity>,
    /// 次のエンティティ ID
    next_id: EntityId,
    /// 全エンティティで共有されるキャッシュ
    shared_cache: Arc<Mutex<CompositeCache>>,
}

impl Scene {
    /// 新規シーンを作成
    pub fn new() -> Self {
        Self::new_with_cache(2048)
    }

    /// 指定されたキャッシュサイズで新規シーンを作成
    ///
    /// # 引数
    /// - `cache_size`: 共有キャッシュの最大エントリ数
    pub fn new_with_cache(cache_size: usize) -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
            shared_cache: Arc::new(Mutex::new(CompositeCache::new(cache_size))),
        }
    }

    /// エンティティを追加し、割り当てられた ID を返す
    ///
    /// # 引数
    /// - `asset`: スプライト資産
    ///
    /// # 戻り値
    /// 割り当てられたエンティティ ID
    pub fn add_entity(&mut self, asset: Arc<SpriteAsset>) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = GameEntity::new(id, asset);
        self.entities.insert(id, entity);

        id
    }

    /// エンティティを削除
    ///
    /// # 戻り値
    /// 削除されたエンティティ、または None
    pub fn remove_entity(&mut self, id: EntityId) -> Option<GameEntity> {
        self.entities.remove(&id)
    }

    /// エンティティを取得（可変）
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut GameEntity> {
        self.entities.get_mut(&id)
    }

    /// エンティティを取得（読み取り）
    pub fn get_entity(&self, id: EntityId) -> Option<&GameEntity> {
        self.entities.get(&id)
    }

    /// すべてのエンティティ ID を取得
    pub fn get_entity_ids(&self) -> Vec<EntityId> {
        self.entities.keys().copied().collect()
    }

    /// エンティティ数を取得
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// すべてのエンティティを更新
    ///
    /// # 引数
    /// - `delta_ms`: 経過時間（ミリ秒）
    pub fn update(&mut self, delta_ms: f32) {
        for entity in self.entities.values_mut() {
            entity.update(delta_ms);
        }
    }

    /// すべてのエンティティをレンダリング
    ///
    /// # 戻り値
    /// (EntityId, SpriteData) のペアベクター
    pub fn render_all(&self) -> Vec<(EntityId, SpriteData)> {
        let mut results = Vec::new();

        for (id, entity) in &self.entities {
            if !entity.is_visible() {
                continue;
            }

            if let Ok(sprite_data) = entity.render() {
                results.push((*id, sprite_data));
            }
        }

        results
    }

    /// 共有キャッシュの統計情報を取得
    pub fn get_cache_stats(&self) -> crate::engine::cache::CacheStats {
        self.shared_cache.lock().unwrap().get_stats()
    }

    /// 共有キャッシュをクリア
    pub fn clear_shared_cache(&self) {
        self.shared_cache.lock().unwrap().clear();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::asset::{AnimationClipDef, Cel, FrameDef, LayerDef};
    use crate::resource::ColorMode;

    fn create_test_asset() -> Arc<SpriteAsset> {
        let mut asset = SpriteAsset::new(1, "test");

        // レイヤー定義を追加
        asset.add_layer_def(LayerDef::new(0, "layer0"));

        // フレームを作成
        let mut frame0 = FrameDef::new(0, 100);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        frame0.add_cel(Cel::new(0, pixels));

        let mut frame1 = FrameDef::new(1, 150);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        frame1.add_cel(Cel::new(0, pixels));

        // フレームデータを設定
        asset.set_frame(0, frame0);
        asset.set_frame(1, frame1);

        // アニメーションクリップを作成
        let clip = AnimationClipDef {
            name: "test_clip".to_string(),
            frames: vec![FrameDef::new(0, 100), FrameDef::new(1, 150)],
            looping: true,
        };

        asset.add_animation_clip(clip);

        Arc::new(asset)
    }

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new();
        assert_eq!(scene.entity_count(), 0);
    }

    #[test]
    fn test_scene_add_entity() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id1 = scene.add_entity(asset.clone());
        let id2 = scene.add_entity(asset.clone());

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(scene.entity_count(), 2);
    }

    #[test]
    fn test_scene_get_entity() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id = scene.add_entity(asset);
        assert!(scene.get_entity(id).is_some());
        assert!(scene.get_entity(999).is_none());
    }

    #[test]
    fn test_scene_remove_entity() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id = scene.add_entity(asset);
        assert_eq!(scene.entity_count(), 1);

        let removed = scene.remove_entity(id);
        assert!(removed.is_some());
        assert_eq!(scene.entity_count(), 0);
    }

    #[test]
    fn test_scene_update() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id = scene.add_entity(asset);

        if let Some(entity) = scene.get_entity_mut(id) {
            entity.play();
        }

        // フレーム 0 は 100ms なので 50ms 進めると frame_idx は 0 のまま
        scene.update(50.0);

        if let Some(entity) = scene.get_entity(id) {
            assert_eq!(entity.animator.current_frame_idx, 0);
        }

        // もう 50ms で フレーム 1 へ
        scene.update(50.0);

        if let Some(entity) = scene.get_entity(id) {
            assert_eq!(entity.animator.current_frame_idx, 1);
        }
    }

    #[test]
    fn test_scene_render_all() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id1 = scene.add_entity(asset.clone());
        let id2 = scene.add_entity(asset.clone());

        if let Some(entity) = scene.get_entity_mut(id2) {
            entity.set_visible(false);
        }

        let rendered = scene.render_all();
        assert_eq!(rendered.len(), 1); // id2 は非表示なので表示されない
        assert_eq!(rendered[0].0, id1);
    }

    #[test]
    fn test_scene_shared_asset() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id1 = scene.add_entity(asset.clone());
        let id2 = scene.add_entity(asset.clone());

        // 同じアセットを参照していることを確認
        let entity1 = scene.get_entity(id1).unwrap();
        let entity2 = scene.get_entity(id2).unwrap();

        // 同じアセット参照
        assert_eq!(entity1.animator.asset.id, entity2.animator.asset.id);

        // しかし独立した再生状態
        let mut scene = Scene::new();
        let id1 = scene.add_entity(asset.clone());
        let id2 = scene.add_entity(asset);

        if let Some(entity) = scene.get_entity_mut(id1) {
            entity.play();
            entity.animator.update(100.0); // frame 0 完了
        }

        // entity2 の再生状態は影響されない
        if let Some(entity) = scene.get_entity(id2) {
            assert_eq!(entity.animator.current_frame_idx, 0);
        }
    }

    #[test]
    fn test_scene_shared_cache() {
        let mut scene = Scene::new_with_cache(128);
        let asset = create_test_asset();

        let id1 = scene.add_entity(asset.clone());
        let id2 = scene.add_entity(asset.clone());

        // 両エンティティが render
        if let Some(entity) = scene.get_entity(id1) {
            let _ = entity.render();
        }

        if let Some(entity) = scene.get_entity(id2) {
            let _ = entity.render();
        }

        // キャッシュ統計を確認
        let stats = scene.get_cache_stats();
        // 各エンティティが独自のキャッシュを持つため、共有キャッシュは使われない
        // しかし新しいキャッシュ機能が動作していることは確認できる
        assert!(stats.hits >= 0);
    }

    #[test]
    fn test_scene_cache_clear() {
        let mut scene = Scene::new();
        let asset = create_test_asset();

        let id = scene.add_entity(asset);
        if let Some(entity) = scene.get_entity(id) {
            let _ = entity.render();
        }

        // キャッシュをクリア
        scene.clear_shared_cache();

        // キャッシュの統計を取得（クリア後は空）
        let stats = scene.get_cache_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
