//! ゲームエンティティ管理（Phase 7 Task 1）
//!
//! GameEntity は SpriteAnimator を保有し、ゲーム世界上の実体を表す。
//! 複数の entity は同じ Arc<SpriteAsset> を共有できる。

use crate::engine::animator::SpriteAnimator;
use crate::resource::asset::SpriteAsset;
use crate::resource::SpriteData;
use std::sync::Arc;

/// エンティティ ID 型
pub type EntityId = u64;

/// ゲームエンティティ：ゲーム世界上の実体
///
/// SpriteAnimator を保有し、位置、可視性を管理する。
/// 複数エンティティは同じ SpriteAsset を共有可能。
#[derive(Clone)]
pub struct GameEntity {
    /// エンティティ ID（シーン内で一意）
    pub id: EntityId,
    /// ワールド座標（X, Y）
    pub position: (f32, f32),
    /// アニメーター（独立した再生状態）
    pub animator: SpriteAnimator,
    /// 表示/非表示
    pub visible: bool,
}

impl GameEntity {
    /// 新規エンティティを作成
    ///
    /// # 引数
    /// - `id`: エンティティ ID
    /// - `asset`: スプライト資産（複数エンティティで共有可）
    ///
    /// # Panics
    /// アセットにアニメーションクリップが存在しない場合
    pub fn new(id: EntityId, asset: Arc<SpriteAsset>) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            animator: SpriteAnimator::new(asset),
            visible: true,
        }
    }

    /// 指定クリップからエンティティを作成
    pub fn with_clip(id: EntityId, asset: Arc<SpriteAsset>, clip_idx: usize) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            animator: SpriteAnimator::with_clip(asset, clip_idx),
            visible: true,
        }
    }

    /// エンティティ位置を設定
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    /// エンティティ位置を取得
    pub fn get_position(&self) -> (f32, f32) {
        self.position
    }

    /// 時間を進める（フレーム更新）
    ///
    /// # 引数
    /// - `delta_ms`: 経過時間（ミリ秒）
    pub fn update(&mut self, delta_ms: f32) {
        self.animator.update(delta_ms);
    }

    /// 現在のフレームをレンダリング
    ///
    /// # 戻り値
    /// 合成済み SpriteData、またはエラーメッセージ
    pub fn render(&self) -> Result<SpriteData, String> {
        self.animator.render()
    }

    /// 再生を開始
    pub fn play(&mut self) {
        self.animator.play();
    }

    /// 再生を一時停止
    pub fn pause(&mut self) {
        self.animator.pause();
    }

    /// 再生を停止
    pub fn stop(&mut self) {
        self.animator.stop();
    }

    /// 可視性を設定
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// 可視性を取得
    pub fn is_visible(&self) -> bool {
        self.visible
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
    fn test_entity_creation() {
        let asset = create_test_asset();
        let entity = GameEntity::new(1, asset);

        assert_eq!(entity.id, 1);
        assert_eq!(entity.position, (0.0, 0.0));
        assert!(entity.visible);
    }

    #[test]
    fn test_entity_position() {
        let asset = create_test_asset();
        let mut entity = GameEntity::new(1, asset);

        entity.set_position(100.0, 200.0);
        assert_eq!(entity.get_position(), (100.0, 200.0));
    }

    #[test]
    fn test_entity_visibility() {
        let asset = create_test_asset();
        let mut entity = GameEntity::new(1, asset);

        assert!(entity.is_visible());
        entity.set_visible(false);
        assert!(!entity.is_visible());
    }
}
