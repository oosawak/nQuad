use crate::resource::SpriteData;
use crate::audio::AudioManager;
use super::camera::Camera;
use super::drawing::DrawingContext;
use super::input::InputState;
use super::particles::ParticleSystem;

/// ゲームシーンのトレイト
pub trait Scene {
    fn update(&mut self, delta_ms: f32);
    fn render(&self) -> Vec<SpriteData>;
}

/// ゲームエンジン統合
pub struct GameEngine {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub drawing: DrawingContext,
    pub input: InputState,
    pub camera: Camera,
    pub audio: AudioManager,
    pub particles: ParticleSystem,
}

impl GameEngine {
    /// 新規ゲームエンジンを作成
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        let palette = crate::api::drawing::PYXEL_PALETTE.to_vec();
        GameEngine {
            width,
            height,
            fps,
            drawing: DrawingContext::new(width, height, palette),
            input: InputState::new(),
            camera: Camera::new(width, height),
            audio: AudioManager::new(),
            particles: ParticleSystem::new(256),
        }
    }

    /// フレーム更新（入力フレーム更新 + パーティクル更新）
    pub fn update(&mut self, _delta_ms: f32) {
        self.input.update_frame();
        self.particles.update();
    }

    /// 描画をリセット（クリア）
    pub fn clear(&mut self, color: u8) {
        self.drawing.cls(color);
    }

    /// フレームタイムを計算（FPS ベース）
    pub fn frame_time_ms(&self) -> f32 {
        1000.0 / self.fps as f32
    }

    /// 指定時間のフレームカウント
    pub fn frames_for_duration(&self, duration_ms: f32) -> u32 {
        (duration_ms / self.frame_time_ms()) as u32
    }

    /// AudioManager を設定
    pub fn with_audio(mut self, audio: AudioManager) -> Self {
        self.audio = audio;
        self
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new(160, 120, 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_engine_creation() {
        let engine = GameEngine::new(160, 120, 60);
        assert_eq!(engine.width, 160);
        assert_eq!(engine.height, 120);
        assert_eq!(engine.fps, 60);
    }

    #[test]
    fn test_frame_time_calculation() {
        let engine = GameEngine::new(160, 120, 60);
        let frame_time = engine.frame_time_ms();
        assert!((frame_time - 16.666666667).abs() < 0.1);
    }

    #[test]
    fn test_frames_for_duration() {
        let engine = GameEngine::new(160, 120, 60);
        let frames = engine.frames_for_duration(1000.0);
        assert_eq!(frames, 60);
    }

    #[test]
    fn test_input_state_update() {
        let mut engine = GameEngine::new(160, 120, 60);
        use super::super::input::Key;
        engine.input.press_key(Key::Up);
        engine.update(16.667);
        assert!(engine.input.btn(Key::Up));
        assert!(!engine.input.btnp(Key::Up));
    }
}
