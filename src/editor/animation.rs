//! アニメーション・フレームシステム（Phase 6）
//!
//! フレーム定義、アニメーションクリップ、再生制御を提供します。

use crate::editor::LayerStack;

/// アニメーションフレーム：タイムスタンプ + レイヤースタック状態
#[derive(Clone, Debug)]
pub struct Frame {
    /// フレーム番号（0 から開始）
    pub frame_num: u32,
    /// フレーム継続時間（ミリ秒）
    pub duration_ms: u32,
    /// このフレームのレイヤースタック状態
    pub layers: LayerStack,
}

impl Frame {
    /// 新しいフレームを作成
    ///
    /// # 例
    /// ```ignore
    /// let frame = Frame::new(0, 100, layer_stack);
    /// assert_eq!(frame.frame_num, 0);
    /// assert_eq!(frame.duration_ms, 100);
    /// ```
    pub fn new(frame_num: u32, duration_ms: u32, layers: LayerStack) -> Self {
        Self {
            frame_num,
            duration_ms,
            layers,
        }
    }

    /// デフォルト継続時間（100ms = 10fps）
    pub const DEFAULT_DURATION_MS: u32 = 100;
}

/// アニメーション再生状態
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    /// 停止
    Stopped,
    /// 再生中
    Playing,
    /// 一時停止
    Paused,
}

impl Default for PlaybackState {
    fn default() -> Self {
        PlaybackState::Stopped
    }
}

/// アニメーションクリップ：フレームシーケンス + 再生制御
#[derive(Clone, Debug)]
pub struct AnimationClip {
    /// クリップ名
    pub name: String,
    /// フレームリスト
    frames: Vec<Frame>,
    /// 現在のフレームインデックス
    current_frame_idx: usize,
    /// 再生状態
    playback_state: PlaybackState,
    /// ループするか
    looping: bool,
    /// 経過時間（ミリ秒）
    elapsed_ms: f32,
    /// 再生速度倍率（1.0 = 通常速度、2.0 = 2倍速）
    speed: f32,
}

impl AnimationClip {
    /// 新しいアニメーションクリップを作成（初期フレーム 1 個）
    pub fn new(name: impl Into<String>, first_frame: Frame) -> Self {
        Self {
            name: name.into(),
            frames: vec![first_frame],
            current_frame_idx: 0,
            playback_state: PlaybackState::Stopped,
            looping: true,
            elapsed_ms: 0.0,
            speed: 1.0,
        }
    }

    /// フレームを追加
    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    /// 現在のフレームを取得
    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame_idx]
    }

    /// 全フレームを取得
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// フレーム一覧を取得（別名）
    pub fn get_frames(&self) -> &[Frame] {
        self.frames()
    }

    /// フレーム数
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// 現在のフレームインデックス
    pub fn current_frame_idx(&self) -> usize {
        self.current_frame_idx
    }

    /// 再生状態
    pub fn playback_state(&self) -> PlaybackState {
        self.playback_state
    }

    /// ループ設定
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    /// ループ設定を取得
    pub fn is_looping(&self) -> bool {
        self.looping
    }

    /// 再生速度を設定（1.0 = 通常、クランプ: 0.1～4.0）
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.1, 4.0);
    }

    /// 再生を開始
    pub fn play(&mut self) {
        self.playback_state = PlaybackState::Playing;
        self.elapsed_ms = 0.0;
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

    /// フレームを特定のインデックスに進める
    pub fn set_frame(&mut self, idx: usize) -> bool {
        if idx < self.frames.len() {
            self.current_frame_idx = idx;
            self.elapsed_ms = 0.0;
            true
        } else {
            false
        }
    }

    /// 次のフレームに進める
    pub fn next_frame(&mut self) {
        if self.current_frame_idx + 1 < self.frames.len() {
            self.current_frame_idx += 1;
            self.elapsed_ms = 0.0;
        } else if self.looping {
            self.current_frame_idx = 0;
            self.elapsed_ms = 0.0;
        }
    }

    /// 前のフレームに戻る
    pub fn prev_frame(&mut self) {
        if self.current_frame_idx > 0 {
            self.current_frame_idx -= 1;
            self.elapsed_ms = 0.0;
        } else if self.looping {
            self.current_frame_idx = self.frames.len() - 1;
            self.elapsed_ms = 0.0;
        }
    }

    /// 時間を進める（delta_ms ミリ秒）
    ///
    /// 自動的に次フレームに進み、ループを処理します。
    pub fn update(&mut self, delta_ms: f32) {
        if self.playback_state != PlaybackState::Playing || self.frames.is_empty() {
            return;
        }

        self.elapsed_ms += delta_ms * self.speed;

        let current_duration = self.frames[self.current_frame_idx].duration_ms as f32;

        while self.elapsed_ms >= current_duration {
            self.elapsed_ms -= current_duration;

            if self.current_frame_idx + 1 < self.frames.len() {
                self.current_frame_idx += 1;
            } else if self.looping {
                self.current_frame_idx = 0;
            } else {
                // ループなし、最後のフレーム
                self.playback_state = PlaybackState::Stopped;
                break;
            }
        }
    }

    /// 全フレームの継続時間を計算（ミリ秒）
    pub fn total_duration_ms(&self) -> u32 {
        self.frames.iter().map(|f| f.duration_ms).sum()
    }

    /// クリップをリセット（先頭フレーム、停止）
    pub fn reset(&mut self) {
        self.current_frame_idx = 0;
        self.playback_state = PlaybackState::Stopped;
        self.elapsed_ms = 0.0;
    }
}

/// アニメーションコントローラ：複数クリップの管理
#[derive(Clone, Debug)]
pub struct AnimationController {
    /// クリップ辞書
    clips: Vec<AnimationClip>,
    /// アクティブクリップのインデックス
    active_clip_idx: usize,
}

impl AnimationController {
    /// 新しいコントローラを作成（初期クリップ 1 個）
    pub fn new(clip: AnimationClip) -> Self {
        Self {
            clips: vec![clip],
            active_clip_idx: 0,
        }
    }

    /// クリップを追加
    pub fn add_clip(&mut self, clip: AnimationClip) {
        self.clips.push(clip);
    }

    /// アクティブクリップを取得
    pub fn active_clip(&self) -> &AnimationClip {
        &self.clips[self.active_clip_idx]
    }

    /// アクティブクリップを可変参照で取得
    pub fn active_clip_mut(&mut self) -> &mut AnimationClip {
        &mut self.clips[self.active_clip_idx]
    }

    /// 全クリップを取得
    pub fn clips(&self) -> &[AnimationClip] {
        &self.clips
    }

    /// クリップを名前で選択
    pub fn select_clip(&mut self, name: &str) -> bool {
        if let Some(idx) = self.clips.iter().position(|c| c.name == name) {
            self.active_clip_idx = idx;
            self.clips[idx].stop();
            true
        } else {
            false
        }
    }

    /// クリップをインデックスで選択
    pub fn select_clip_by_idx(&mut self, idx: usize) -> bool {
        if idx < self.clips.len() {
            self.active_clip_idx = idx;
            self.clips[idx].stop();
            true
        } else {
            false
        }
    }

    /// アクティブクリップを更新
    pub fn update(&mut self, delta_ms: f32) {
        self.clips[self.active_clip_idx].update(delta_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{ColorMode, SpriteData};

    fn create_test_frame(num: u32) -> Frame {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let layer_stack = LayerStack::new(sprite);
        Frame::new(num, 100, layer_stack)
    }

    #[test]
    fn test_frame_creation() {
        let frame = create_test_frame(0);
        assert_eq!(frame.frame_num, 0);
        assert_eq!(frame.duration_ms, 100);
    }

    #[test]
    fn test_animation_clip_creation() {
        let frame = create_test_frame(0);
        let clip = AnimationClip::new("Test", frame);
        assert_eq!(clip.name, "Test");
        assert_eq!(clip.frame_count(), 1);
        assert_eq!(clip.playback_state(), PlaybackState::Stopped);
    }

    #[test]
    fn test_animation_playback() {
        let frame0 = create_test_frame(0);
        let frame1 = create_test_frame(1);

        let mut clip = AnimationClip::new("Test", frame0);
        clip.add_frame(frame1);

        assert_eq!(clip.current_frame_idx(), 0);

        clip.play();
        assert_eq!(clip.playback_state(), PlaybackState::Playing);

        clip.pause();
        assert_eq!(clip.playback_state(), PlaybackState::Paused);

        clip.stop();
        assert_eq!(clip.playback_state(), PlaybackState::Stopped);
        assert_eq!(clip.current_frame_idx(), 0);
    }

    #[test]
    fn test_frame_navigation() {
        let frame0 = create_test_frame(0);
        let frame1 = create_test_frame(1);
        let frame2 = create_test_frame(2);

        let mut clip = AnimationClip::new("Test", frame0);
        clip.add_frame(frame1);
        clip.add_frame(frame2);

        clip.set_looping(false);

        assert_eq!(clip.current_frame_idx(), 0);
        clip.next_frame();
        assert_eq!(clip.current_frame_idx(), 1);
        clip.next_frame();
        assert_eq!(clip.current_frame_idx(), 2);

        clip.prev_frame();
        assert_eq!(clip.current_frame_idx(), 1);
    }

    #[test]
    fn test_animation_update() {
        let frame0 = create_test_frame(0);
        let frame1 = create_test_frame(1);

        let mut clip = AnimationClip::new("Test", frame0);
        clip.add_frame(frame1);
        clip.set_looping(false);

        clip.play();
        assert_eq!(clip.current_frame_idx(), 0);

        // フレーム継続時間（100ms）未満
        clip.update(50.0);
        assert_eq!(clip.current_frame_idx(), 0);

        // フレーム継続時間を超える
        clip.update(60.0);
        assert_eq!(clip.current_frame_idx(), 1);

        // 最後のフレーム、ループなし → 停止
        clip.update(150.0);
        assert_eq!(clip.playback_state(), PlaybackState::Stopped);
    }

    #[test]
    fn test_speed_control() {
        let frame0 = create_test_frame(0);
        let frame1 = create_test_frame(1);

        let mut clip = AnimationClip::new("Test", frame0);
        clip.add_frame(frame1);

        clip.set_speed(2.0);
        assert_eq!(clip.speed, 2.0);

        clip.set_speed(0.05); // クランプされる
        assert_eq!(clip.speed, 0.1);

        clip.set_speed(5.0); // クランプされる
        assert_eq!(clip.speed, 4.0);
    }

    #[test]
    fn test_animation_controller() {
        let frame = create_test_frame(0);
        let clip1 = AnimationClip::new("Walk", frame.clone());
        let clip2 = AnimationClip::new("Run", frame);

        let mut controller = AnimationController::new(clip1);
        controller.add_clip(clip2);

        assert_eq!(controller.clips().len(), 2);

        assert!(controller.select_clip("Run"));
        assert_eq!(controller.active_clip().name, "Run");

        assert!(!controller.select_clip("NonExistent"));
    }
}
