//! Game Framework - Unified game loop and trait-based architecture
//!
//! This module provides a reusable framework for building games with Nantaraquad.
//! Games implement the `GameApp` trait and use `GameEngine` for unified input/rendering.

use super::drawing::{DrawingContext, PYXEL_PALETTE};
use super::input::InputState;
use super::game::GameEngine;
use std::time::Instant;

/// Trait for game applications
///
/// Implement this trait to create a game that works with the game framework.
pub trait GameApp {
    /// Update game logic (called each frame)
    fn update(&mut self, input: &InputState, dt: f32);

    /// Draw the game (called each frame)
    fn draw(&self, ctx: &mut DrawingContext);

    /// Check if game should continue running
    fn should_exit(&self) -> bool {
        false
    }
}

/// Game framework runner
pub struct GameRunner {
    engine: GameEngine,
    last_frame_time: Instant,
}

impl GameRunner {
    /// Create a new game runner
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        GameRunner {
            engine: GameEngine::new(width, height, fps),
            last_frame_time: Instant::now(),
        }
    }

    /// Run the game loop
    ///
    /// This function blocks until the game exits. It handles:
    /// - Input processing
    /// - Game update
    /// - Drawing
    /// - Frame rate limiting
    pub fn run<T: GameApp>(&mut self, mut app: T) {
        loop {
            let now = Instant::now();
            let dt = (now - self.last_frame_time).as_secs_f32().min(0.033); // Cap at ~30fps
            self.last_frame_time = now;

            // Input
            self.engine.input.update_frame();

            // Update
            app.update(&self.engine.input, dt);

            // Check exit condition
            if app.should_exit() {
                break;
            }

            // Draw
            let mut ctx = DrawingContext::new(
                self.engine.width,
                self.engine.height,
                PYXEL_PALETTE.to_vec(),
            );
            app.draw(&mut ctx);

            // Frame timing
            let frame_time_ms = self.engine.frame_time_ms();
            let elapsed_ms = now.elapsed().as_secs_f32() * 1000.0;
            let sleep_ms = (frame_time_ms - elapsed_ms).max(0.0);
            if sleep_ms > 0.0 {
                std::thread::sleep(std::time::Duration::from_millis(sleep_ms as u64));
            }
        }
    }

    /// Get the underlying game engine
    pub fn engine(&self) -> &GameEngine {
        &self.engine
    }

    /// Get mutable reference to the game engine
    pub fn engine_mut(&mut self) -> &mut GameEngine {
        &mut self.engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGame {
        update_count: usize,
        draw_count: usize,
    }

    impl TestGame {
        fn new() -> Self {
            TestGame {
                update_count: 0,
                draw_count: 0,
            }
        }
    }

    impl GameApp for TestGame {
        fn update(&mut self, _input: &InputState, _dt: f32) {
            self.update_count += 1;
        }

        fn draw(&self, _ctx: &mut DrawingContext) {
            // Test draws don't produce output
        }

        fn should_exit(&self) -> bool {
            self.update_count >= 3
        }
    }

    #[test]
    fn test_game_runner_creation() {
        let runner = GameRunner::new(160, 120, 60);
        assert_eq!(runner.engine.width, 160);
        assert_eq!(runner.engine.height, 120);
        assert_eq!(runner.engine.fps, 60);
    }

    #[test]
    fn test_game_app_execution() {
        let mut runner = GameRunner::new(160, 120, 60);
        let app = TestGame::new();
        runner.run(app);
        // If we get here without panicking, test passed
    }

    #[test]
    fn test_frame_time_calculation() {
        let runner = GameRunner::new(160, 120, 60);
        let frame_time = runner.engine.frame_time_ms();
        assert!((frame_time - 16.666666667).abs() < 0.1);
    }
}
