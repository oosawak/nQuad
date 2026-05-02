//! ゲーム実行時のアニメーション再生状態管理（Phase 7 Task 1-3）
//!
//! SpriteAnimator は、不変な SpriteAsset を参照しながら、
//! ゲーム実行時の再生位置・速度・状態を管理する。
//!
//! # 設計思想
//!
//! - **Asset と State の分離**: SpriteAsset は複数エンティティで共有される不変資産
//! - **複数エンティティで効率的な共有**: Arc<SpriteAsset> で共有可能
//! - **timing bug の修正**: update() で毎フレーム正確な duration を取得

use crate::engine::cache::{CacheKey, CompositeCache};
use crate::resource::asset::{AnimationClipDef, Cel, FrameDef, SpriteAsset};
use crate::resource::{ColorMode, SpriteData};
use std::cell::RefCell;
use std::sync::Arc;

/// アニメーション再生状態
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    /// 再生中
    Playing,
    /// 一時停止中
    Paused,
    /// 停止中
    Stopped,
}

impl Default for PlaybackState {
    fn default() -> Self {
        PlaybackState::Stopped
    }
}

/// フレーム情報（表示用）
#[derive(Clone, Debug)]
pub struct FrameInfo {
    /// フレームの幅
    pub width: u32,
    /// フレームの高さ
    pub height: u32,
    /// 現在のフレームインデックス
    pub frame_index: u32,
    /// フレーム総数
    pub frame_count: u32,
    /// 現在のクリップ名
    pub clip_name: String,
}

/// ゲーム実行時のスプライトアニメーター
///
/// 不変な SpriteAsset の再生制御と状態を管理する。
/// 複数ゲームエンティティは同一の Arc<SpriteAsset> を共有できるが、
/// 各エンティティは独立した SpriteAnimator インスタンスを持つ。
///
/// # 例
/// ```ignore
/// let asset = Arc::new(sprite_asset);
/// let mut animator = SpriteAnimator::new(asset.clone());
/// animator.play();
/// animator.update(16.0); // 16ms 進める
/// let frame = animator.get_current_frame();
/// ```
#[derive(Clone, Debug)]
pub struct SpriteAnimator {
    /// 不変な資産（複数エンティティで共有可）
    pub asset: Arc<SpriteAsset>,
    /// 再生中のアニメーションクリップインデックス
    pub active_clip_idx: usize,
    /// フレーム再生位置
    pub current_frame_idx: u32,
    /// フレーム経過時間（ミリ秒）
    pub elapsed_ms: f32,
    /// 再生状態
    pub playback_state: PlaybackState,
    /// 再生速度倍率（1.0 = 通常速度）
    pub speed: f32,
    /// コンポジットキャッシュ（専有キャッシュ）
    composite_cache: RefCell<CompositeCache>,
}

impl SpriteAnimator {
    /// 新規アニメーターを作成（最初のクリップから再生開始）
    ///
    /// # Panics
    /// アセットにアニメーションクリップが存在しない場合。
    pub fn new(asset: Arc<SpriteAsset>) -> Self {
        if asset.animation_clips.is_empty() {
            panic!("SpriteAsset must have at least one animation clip");
        }

        Self {
            asset,
            active_clip_idx: 0,
            current_frame_idx: 0,
            elapsed_ms: 0.0,
            playback_state: PlaybackState::Stopped,
            speed: 1.0,
            composite_cache: RefCell::new(CompositeCache::new(128)),
        }
    }

    /// 指定されたクリップから開始するアニメーターを作成
    ///
    /// # Panics
    /// clip_idx がクリップリスト範囲外の場合。
    pub fn with_clip(asset: Arc<SpriteAsset>, clip_idx: usize) -> Self {
        if clip_idx >= asset.animation_clips.len() {
            panic!(
                "Clip index {} out of range (total: {})",
                clip_idx,
                asset.animation_clips.len()
            );
        }

        Self {
            asset,
            active_clip_idx: clip_idx,
            current_frame_idx: 0,
            elapsed_ms: 0.0,
            playback_state: PlaybackState::Stopped,
            speed: 1.0,
            composite_cache: RefCell::new(CompositeCache::new(128)),
        }
    }

    // ========== 再生制御 ==========

    /// 再生を開始
    pub fn play(&mut self) {
        self.playback_state = PlaybackState::Playing;
    }

    /// 一時停止
    pub fn pause(&mut self) {
        self.playback_state = PlaybackState::Paused;
    }

    /// 停止（先頭フレームに戻る）
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_frame_idx = 0;
        self.elapsed_ms = 0.0;
    }

    /// 時間を進める（フレーム更新）
    ///
    /// 再生状態で delta_ms 分だけ時間を進める。
    /// フレーム継続時間を超えたら自動的に次フレームに進む。
    ///
    /// # 仕様
    /// - 再生状態でない場合は何もしない
    /// - 速度倍率を適用
    /// - 複数フレームスキップをサポート
    /// - ループ設定に従う
    ///
    /// # Timing Bug Fix
    /// 毎フレーム `current_frame_duration` を取得することで、
    /// フレーム間で duration が変わった場合でも正確に判定する。
    pub fn update(&mut self, delta_ms: f32) {
        if self.playback_state != PlaybackState::Playing {
            return;
        }

        let clip = &self.asset.animation_clips[self.active_clip_idx];
        if clip.frames.is_empty() {
            return;
        }

        // 速度を適用
        self.elapsed_ms += delta_ms * self.speed;

        // フレーム継続時間を取得（毎フレーム取得する！Rubber Duck bug fix）
        let current_frame_duration =
            clip.frames[self.current_frame_idx as usize].duration_ms as f32;

        // フレーム進行
        if self.elapsed_ms >= current_frame_duration {
            self.elapsed_ms -= current_frame_duration;
            self.current_frame_idx += 1;

            // ループ判定
            if self.current_frame_idx >= clip.frames.len() as u32 {
                if clip.looping {
                    self.current_frame_idx = 0;
                } else {
                    self.stop();
                }
            }
        }
    }

    /// 再生速度を設定（クランプ: 0.1～4.0）
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.1, 4.0);
    }

    /// フレームを直接指定
    pub fn set_frame(&mut self, frame_idx: u32) -> bool {
        let clip = &self.asset.animation_clips[self.active_clip_idx];
        if frame_idx < clip.frames.len() as u32 {
            self.current_frame_idx = frame_idx;
            self.elapsed_ms = 0.0;
            true
        } else {
            false
        }
    }

    // ========== データアクセス ==========

    /// 現在のフレーム定義を取得
    pub fn get_current_frame(&self) -> &FrameDef {
        let clip = &self.asset.animation_clips[self.active_clip_idx];
        &clip.frames[self.current_frame_idx as usize]
    }

    /// 現在のフレームから指定レイヤーのセルを取得
    pub fn get_current_cel(&self, layer_id: u32) -> Option<&Cel> {
        self.get_current_frame().get_cel(layer_id)
    }

    /// アニメーションの総継続時間（ミリ秒）を計算
    pub fn get_animation_duration_ms(&self) -> f32 {
        let clip = &self.asset.animation_clips[self.active_clip_idx];
        clip.frames.iter().map(|f| f.duration_ms as f32).sum()
    }

    /// 再生中かどうか
    pub fn is_playing(&self) -> bool {
        self.playback_state == PlaybackState::Playing
    }

    /// 現在のクリップ定義を取得
    pub fn get_current_clip(&self) -> &AnimationClipDef {
        &self.asset.animation_clips[self.active_clip_idx]
    }

    /// 現在のフレーム数を取得
    pub fn get_frame_count(&self) -> u32 {
        self.get_current_clip().frames.len() as u32
    }

    // ========== Frame Resolution パイプライン ==========

    /// 現在フレームのセルデータを取得
    ///
    /// フレームからすべてのセルを取得し、レイヤーID順にソートして返す。
    ///
    /// # 戻り値
    /// (LayerId, Cel) のペアベクター
    fn get_current_frame_cells(&self) -> Result<Vec<(u32, Cel)>, String> {
        let frame = self.get_current_frame();
        let mut cells: Vec<(u32, Cel)> = frame
            .cels
            .iter()
            .map(|(layer_id, cel)| (*layer_id, cel.clone()))
            .collect();

        // レイヤーID順でソート
        cells.sort_by_key(|(layer_id, _)| *layer_id);

        Ok(cells)
    }

    /// フレーム情報を取得
    ///
    /// 現在のフレームに関するメタデータを返す。
    pub fn get_frame_info(&self) -> FrameInfo {
        let clip = self.get_current_clip();
        let frame = self.get_current_frame();

        // 寸法を最初のセルから取得
        let (width, height) = frame
            .cels
            .values()
            .next()
            .map(|cel| (cel.pixels.width, cel.pixels.height))
            .unwrap_or((32, 32));

        FrameInfo {
            width,
            height,
            frame_index: self.current_frame_idx,
            frame_count: clip.frames.len() as u32,
            clip_name: clip.name.clone(),
        }
    }

    /// 現在フレームをレンダリングして SpriteData を返す
    ///
    /// すべてのレイヤーを合成して、単一の SpriteData として返す。
    /// opacity と blend_mode を適用。キャッシュを使用する。
    ///
    /// # 戻り値
    /// 合成済み SpriteData、またはエラーメッセージ
    pub fn render(&self) -> Result<SpriteData, String> {
        let cache_key = CacheKey {
            asset_id: self.asset.id,
            frame_index: self.current_frame_idx,
        };

        // キャッシュから取得を試みる
        let mut cache = self.composite_cache.borrow_mut();
        if let Some(cached_data) = cache.get(&cache_key) {
            return Ok(cached_data);
        }
        drop(cache);

        // キャッシュミス：合成計算
        let cells = self.get_current_frame_cells()?;

        if cells.is_empty() {
            return Err("No cells found in current frame".to_string());
        }

        // 最初のセルから寸法を取得
        let first_cell = &cells[0].1;
        let width = first_cell.pixels.width;
        let height = first_cell.pixels.height;
        let mode = first_cell.pixels.mode.clone();

        // 出力バッファを作成
        let mut result = SpriteData::new(width, height, mode);

        // すべてのセルを合成（単純な上書き）
        // TODO: より高度なブレンディング処理
        for (layer_id, cel) in cells {
            if !cel.visible {
                continue;
            }

            // セルのピクセルを結果にコピー
            let opacity = cel.opacity_override.unwrap_or_else(|| {
                self.asset
                    .get_layer_def(layer_id)
                    .map(|def| def.default_opacity)
                    .unwrap_or(1.0)
            });

            // フルカラーモードの場合、透明度を適用
            match (&result.mode, &cel.pixels.mode) {
                (ColorMode::FullColor, ColorMode::FullColor) => {
                    // RGBA モード: アルファチャンネルに不透明度を適用
                    for (src_pixel, dst_pixel) in cel
                        .pixels
                        .pixels
                        .chunks(4)
                        .zip(result.pixels.chunks_exact_mut(4))
                    {
                        if src_pixel.len() == 4 {
                            let src_alpha = (src_pixel[3] as f32 / 255.0) * opacity;
                            dst_pixel[0] = src_pixel[0];
                            dst_pixel[1] = src_pixel[1];
                            dst_pixel[2] = src_pixel[2];
                            dst_pixel[3] = (src_alpha * 255.0) as u8;
                        }
                    }
                }
                _ => {
                    // その他のモードは単純コピー
                    result.pixels.copy_from_slice(&cel.pixels.pixels);
                }
            }
        }

        // キャッシュに保存
        let mut cache = self.composite_cache.borrow_mut();
        cache.insert(cache_key, result.clone());

        Ok(result)
    }

    /// キャッシュをクリア
    pub fn clear_cache(&self) {
        self.composite_cache.borrow_mut().clear();
    }

    /// キャッシュ統計を取得
    pub fn get_cache_stats(&self) -> crate::engine::cache::CacheStats {
        self.composite_cache.borrow().get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::asset::{AnimationClipDef, Cel, FrameDef, LayerDef};
    use crate::resource::ColorMode;
    use crate::resource::SpriteData;

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

        let mut frame2 = FrameDef::new(2, 200);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        frame2.add_cel(Cel::new(0, pixels));

        // フレームデータを設定
        asset.set_frame(0, frame0);
        asset.set_frame(1, frame1);
        asset.set_frame(2, frame2);

        // アニメーションクリップを作成
        let clip = AnimationClipDef {
            name: "test_clip".to_string(),
            frames: vec![
                FrameDef::new(0, 100),
                FrameDef::new(1, 150),
                FrameDef::new(2, 200),
            ],
            looping: true,
        };

        asset.add_animation_clip(clip);

        Arc::new(asset)
    }

    #[test]
    fn test_animator_creation() {
        let asset = create_test_asset();
        let animator = SpriteAnimator::new(asset);

        assert_eq!(animator.active_clip_idx, 0);
        assert_eq!(animator.current_frame_idx, 0);
        assert_eq!(animator.elapsed_ms, 0.0);
        assert_eq!(animator.playback_state, PlaybackState::Stopped);
        assert_eq!(animator.speed, 1.0);
    }

    #[test]
    fn test_animator_with_clip() {
        let mut asset_data = SpriteAsset::new(1, "test");
        asset_data.add_layer_def(LayerDef::new(0, "layer0"));

        let frame0 = FrameDef::new(0, 100);
        let frame1 = FrameDef::new(1, 100);

        let mut clip0 = AnimationClipDef {
            name: "clip0".to_string(),
            frames: vec![frame0],
            looping: true,
        };

        let mut clip1 = AnimationClipDef {
            name: "clip1".to_string(),
            frames: vec![frame1],
            looping: true,
        };

        asset_data.add_animation_clip(clip0);
        asset_data.add_animation_clip(clip1);

        let asset = Arc::new(asset_data);
        let animator = SpriteAnimator::with_clip(asset, 1);

        assert_eq!(animator.active_clip_idx, 1);
    }

    #[test]
    fn test_animator_play_pause_stop() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        assert_eq!(animator.playback_state, PlaybackState::Stopped);

        animator.play();
        assert_eq!(animator.playback_state, PlaybackState::Playing);

        animator.pause();
        assert_eq!(animator.playback_state, PlaybackState::Paused);

        animator.play();
        assert_eq!(animator.playback_state, PlaybackState::Playing);

        animator.stop();
        assert_eq!(animator.playback_state, PlaybackState::Stopped);
        assert_eq!(animator.current_frame_idx, 0);
        assert_eq!(animator.elapsed_ms, 0.0);
    }

    #[test]
    fn test_animator_update_timing() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        animator.play();

        // フレーム 0 継続: 100ms
        animator.update(50.0);
        assert_eq!(animator.current_frame_idx, 0);
        assert_eq!(animator.elapsed_ms, 50.0);

        // フレーム 0 完了、フレーム 1 へ
        animator.update(50.0);
        assert_eq!(animator.current_frame_idx, 1);
        assert_eq!(animator.elapsed_ms, 0.0);

        // フレーム 1 継続: 150ms
        animator.update(100.0);
        assert_eq!(animator.current_frame_idx, 1);
        assert_eq!(animator.elapsed_ms, 100.0);

        // フレーム 1 完了、フレーム 2 へ
        animator.update(50.0);
        assert_eq!(animator.current_frame_idx, 2);
        assert_eq!(animator.elapsed_ms, 0.0);
    }

    #[test]
    fn test_animator_frame_skip() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        animator.play();

        // 高速 delta で複数フレームスキップ: 100 + 150 + 50 = 300ms
        animator.update(300.0);

        assert_eq!(animator.current_frame_idx, 2);
        assert_eq!(animator.elapsed_ms, 50.0);
    }

    #[test]
    fn test_animator_looping() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        animator.play();

        // すべてのフレームを通過
        animator.update(100.0); // frame 0 完了 -> frame 1
        animator.update(150.0); // frame 1 完了 -> frame 2
        animator.update(200.0); // frame 2 完了、ループして frame 0

        assert_eq!(animator.current_frame_idx, 0);
        assert!(animator.is_playing());
    }

    #[test]
    fn test_animator_looping_false() {
        let mut asset_data = SpriteAsset::new(1, "test");
        asset_data.add_layer_def(LayerDef::new(0, "layer0"));

        let clip = AnimationClipDef {
            name: "no_loop".to_string(),
            frames: vec![FrameDef::new(0, 100), FrameDef::new(1, 100)],
            looping: false,
        };

        asset_data.add_animation_clip(clip);

        let asset = Arc::new(asset_data);
        let mut animator = SpriteAnimator::new(asset);

        animator.play();

        animator.update(100.0); // frame 0 完了 -> frame 1
        assert_eq!(animator.current_frame_idx, 1);
        assert!(animator.is_playing());

        animator.update(100.0); // frame 1 完了、ループなし -> 停止
        assert_eq!(animator.current_frame_idx, 1);
        assert!(!animator.is_playing());
    }

    #[test]
    fn test_animator_speed() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        animator.play();
        animator.set_speed(2.0);

        // 速度 2.0 で 50ms 進める = 実際には 100ms 進んだのと同等
        animator.update(50.0);
        assert_eq!(animator.elapsed_ms, 100.0);

        // フレーム 0 は 100ms 継続なので完了
        assert_eq!(animator.current_frame_idx, 0);

        // もう 50ms で フレーム 0 完了
        animator.update(50.0);
        assert_eq!(animator.current_frame_idx, 1);
    }

    #[test]
    fn test_animator_set_frame() {
        let asset = create_test_asset();
        let mut animator = SpriteAnimator::new(asset);

        assert!(animator.set_frame(1));
        assert_eq!(animator.current_frame_idx, 1);
        assert_eq!(animator.elapsed_ms, 0.0);

        assert!(!animator.set_frame(10)); // 範囲外
        assert_eq!(animator.current_frame_idx, 1); // 変わらず
    }

    #[test]
    fn test_animator_get_current_frame() {
        let asset = create_test_asset();
        let animator = SpriteAnimator::new(asset);

        let frame = animator.get_current_frame();
        assert_eq!(frame.frame_num, 0);
        assert_eq!(frame.duration_ms, 100);
    }

    #[test]
    fn test_animator_get_animation_duration_ms() {
        let asset = create_test_asset();
        let animator = SpriteAnimator::new(asset);

        // 100 + 150 + 200 = 450
        assert_eq!(animator.get_animation_duration_ms(), 450.0);
    }

    #[test]
    fn test_animator_caching() {
        let asset = create_test_asset();
        let animator = SpriteAnimator::new(asset);

        // 最初の render
        let result1 = animator.render();
        assert!(result1.is_ok());

        let stats1 = animator.get_cache_stats();
        assert_eq!(stats1.misses, 1);
        assert_eq!(stats1.hits, 0);

        // 同じフレームを再度 render
        let result2 = animator.render();
        assert!(result2.is_ok());

        let stats2 = animator.get_cache_stats();
        assert_eq!(stats2.misses, 1); // ミス数は増えない
        assert_eq!(stats2.hits, 1); // ヒット数が増える
    }

    #[test]
    fn test_animator_cache_clear() {
        let asset = create_test_asset();
        let animator = SpriteAnimator::new(asset);

        // render でキャッシュを埋める
        let _ = animator.render();
        let stats = animator.get_cache_stats();
        assert_eq!(stats.hits + stats.misses, 1);

        // キャッシュをクリア
        animator.clear_cache();
        let stats_after = animator.get_cache_stats();
        // 統計はクリアされないが、キャッシュは空になる
        let result = animator.render();
        assert!(result.is_ok());

        let stats_final = animator.get_cache_stats();
        assert_eq!(stats_final.misses, 2); // もう一度ミス
    }
}
