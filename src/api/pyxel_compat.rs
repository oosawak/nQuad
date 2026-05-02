use crate::api::drawing::{DrawingContext, PYXEL_PALETTE};
use crate::api::input::{InputState, Key};
use crate::api::camera::Camera;
use crate::api::game::GameEngine;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Test 1: Drawing API - rect/circle/line
    // ============================================================

    #[test]
    fn test_drawing_api_rect_and_circle() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);

        // Clear to black
        ctx.cls(0);

        // Draw rect outline
        ctx.rect(10, 10, 20, 20, 7); // white
        assert_eq!(ctx.pget(10, 10), Some(7));
        assert_eq!(ctx.pget(29, 10), Some(7));

        // Draw filled rect
        ctx.rectfill(50, 50, 20, 20, 8); // red
        assert_eq!(ctx.pget(50, 50), Some(8));
        assert_eq!(ctx.pget(60, 60), Some(8));

        // Draw circle
        ctx.circle(100, 100, 10, 3); // green
        let color = ctx.pget(100, 100);
        assert!(color.is_some());

        // Draw filled circle
        ctx.circfill(100, 100, 5, 11); // lime
        assert_eq!(ctx.pget(100, 100), Some(11));
    }

    #[test]
    fn test_drawing_api_line() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);

        ctx.cls(0);

        // Draw horizontal line
        ctx.line(10, 50, 60, 50, 7); // white
        assert_eq!(ctx.pget(10, 50), Some(7));
        assert_eq!(ctx.pget(35, 50), Some(7));

        // Draw vertical line
        ctx.line(80, 20, 80, 80, 8); // red
        assert_eq!(ctx.pget(80, 20), Some(8));
        assert_eq!(ctx.pget(80, 50), Some(8));

        // Draw diagonal line
        ctx.line(100, 100, 130, 130, 3); // green
        let color = ctx.pget(115, 115);
        assert!(color.is_some());
    }

    #[test]
    fn test_drawing_api_pset_pget() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);

        ctx.cls(0);

        // Set individual pixels
        ctx.pset(50, 50, 7); // white
        ctx.pset(60, 60, 8); // red
        ctx.pset(70, 70, 3); // green

        assert_eq!(ctx.pget(50, 50), Some(7));
        assert_eq!(ctx.pget(60, 60), Some(8));
        assert_eq!(ctx.pget(70, 70), Some(3));

        // Test out of bounds
        assert_eq!(ctx.pget(-1, 0), None);
        assert_eq!(ctx.pget(160, 120), None);
    }

    // ============================================================
    // Test 2: Input API
    // ============================================================

    #[test]
    fn test_input_api_btn() {
        let mut input = InputState::new();

        // No keys pressed initially
        assert!(!input.btn(Key::Up));
        assert!(!input.btn(Key::Down));

        // Press keys
        input.press_key(Key::Up);
        input.press_key(Key::Left);

        assert!(input.btn(Key::Up));
        assert!(input.btn(Key::Left));
        assert!(!input.btn(Key::Down));
    }

    #[test]
    fn test_input_api_btnp() {
        let mut input = InputState::new();

        // No keys pressed
        assert!(!input.btnp(Key::Up));

        // Press a key - should detect as new press
        input.press_key(Key::Up);
        assert!(input.btnp(Key::Up));

        // Update frame - btnp should clear
        input.update_frame();
        assert!(!input.btnp(Key::Up));

        // But btn should still return true
        assert!(input.btn(Key::Up));

        // Press again in new frame
        input.press_key(Key::Down);
        assert!(input.btnp(Key::Down));
    }

    #[test]
    fn test_input_api_multiple_keys() {
        let mut input = InputState::new();

        // Simulate a complex input sequence
        input.press_key(Key::W);
        input.press_key(Key::A);
        assert!(input.btn(Key::W));
        assert!(input.btn(Key::A));
        assert!(input.btnp(Key::W));
        assert!(input.btnp(Key::A));

        input.update_frame();
        assert!(input.btn(Key::W));
        assert!(input.btn(Key::A));
        assert!(!input.btnp(Key::W));
        assert!(!input.btnp(Key::A));

        // Release one key
        input.release_key(Key::W);
        assert!(!input.btn(Key::W));
        assert!(input.btn(Key::A));
    }

    // ============================================================
    // Test 3: Camera
    // ============================================================

    #[test]
    fn test_camera_world_to_screen() {
        let camera = Camera::new(160, 120);

        // At origin, world and screen are the same
        let (sx, sy) = camera.world_to_screen(0.0, 0.0);
        assert_eq!(sx, 0);
        assert_eq!(sy, 0);

        let (sx, sy) = camera.world_to_screen(100.0, 100.0);
        assert_eq!(sx, 100);
        assert_eq!(sy, 100);
    }

    #[test]
    fn test_camera_world_to_screen_with_offset() {
        let mut camera = Camera::new(160, 120);
        camera.x = 50.0;
        camera.y = 50.0;

        let (sx, sy) = camera.world_to_screen(100.0, 100.0);
        assert_eq!(sx, 50);
        assert_eq!(sy, 50);

        let (sx, sy) = camera.world_to_screen(50.0, 50.0);
        assert_eq!(sx, 0);
        assert_eq!(sy, 0);
    }

    #[test]
    fn test_camera_screen_to_world() {
        let camera = Camera::new(160, 120);

        let (wx, wy) = camera.screen_to_world(100, 100);
        assert_eq!(wx, 100.0);
        assert_eq!(wy, 100.0);

        let (wx, wy) = camera.screen_to_world(0, 0);
        assert_eq!(wx, 0.0);
        assert_eq!(wy, 0.0);
    }

    #[test]
    fn test_camera_follow() {
        let mut camera = Camera::new(160, 120);

        // Follow a target at (200, 200) with speed 0.1
        let old_x = camera.x;
        let old_y = camera.y;

        camera.follow(200.0, 200.0, 0.1);

        // Camera should have moved towards the target
        assert!(camera.x > old_x);
        assert!(camera.y > old_y);

        // Multiple follows should move it closer
        let old_x2 = camera.x;
        let old_y2 = camera.y;
        camera.follow(200.0, 200.0, 0.1);
        assert!(camera.x > old_x2);
        assert!(camera.y > old_y2);
    }

    #[test]
    fn test_camera_zoom() {
        let mut camera = Camera::new(160, 120);

        assert_eq!(camera.scale, 1.0);

        camera.set_zoom(2.0);
        assert_eq!(camera.scale, 2.0);

        camera.zoom_in(2.0);
        assert_eq!(camera.scale, 4.0);

        camera.zoom_out(2.0);
        assert_eq!(camera.scale, 2.0);
    }

    // ============================================================
    // Test 4: GameEngine Integration
    // ============================================================

    #[test]
    fn test_game_engine_creation() {
        let engine = GameEngine::new(160, 120, 60);

        assert_eq!(engine.width, 160);
        assert_eq!(engine.height, 120);
        assert_eq!(engine.fps, 60);
    }

    #[test]
    fn test_game_engine_drawing_integration() {
        let mut engine = GameEngine::new(160, 120, 60);

        // Clear canvas
        engine.clear(0);

        // Draw some shapes
        engine.drawing.rectfill(10, 10, 30, 30, 8); // red
        engine.drawing.circle(100, 60, 15, 7); // white

        // Verify drawing was done
        assert_eq!(engine.drawing.pget(10, 10), Some(8));
        let center_pixel = engine.drawing.pget(100, 60);
        assert!(center_pixel.is_some());
    }

    #[test]
    fn test_game_engine_input_integration() {
        let mut engine = GameEngine::new(160, 120, 60);

        // Simulate input
        engine.input.press_key(Key::Up);
        engine.input.press_key(Key::Left);

        assert!(engine.input.btn(Key::Up));
        assert!(engine.input.btn(Key::Left));

        // Update frame
        engine.update(16.667);

        // Keys should still be pressed
        assert!(engine.input.btn(Key::Up));
        assert!(engine.input.btn(Key::Left));

        // But btnp should be false (cleared after update_frame)
        assert!(!engine.input.btnp(Key::Up));
    }

    #[test]
    fn test_game_engine_camera_integration() {
        let mut engine = GameEngine::new(160, 120, 60);

        // Move camera
        engine.camera.x = 100.0;
        engine.camera.y = 100.0;

        let (sx, sy) = engine.camera.world_to_screen(150.0, 150.0);
        assert_eq!(sx, 50);
        assert_eq!(sy, 50);
    }

    #[test]
    fn test_game_engine_frame_time() {
        let engine = GameEngine::new(160, 120, 60);

        let frame_time = engine.frame_time_ms();
        assert!((frame_time - 16.666666667).abs() < 0.1);

        let frames_per_second = engine.frames_for_duration(1000.0);
        assert_eq!(frames_per_second, 60);

        let frames_half_second = engine.frames_for_duration(500.0);
        assert_eq!(frames_half_second, 30);
    }

    // ============================================================
    // Test 5: Simple Lineboy Demo
    // ============================================================

    struct SimpleLineboy {
        player_x: i32,
        player_y: i32,
        enemy_x: i32,
        enemy_y: i32,
        speed: i32,
    }

    impl SimpleLineboy {
        fn new() -> Self {
            SimpleLineboy {
                player_x: 80,
                player_y: 60,
                enemy_x: 20,
                enemy_y: 20,
                speed: 2,
            }
        }

        fn update(&mut self, input: &InputState) {
            if input.btn(Key::Up) && self.player_y > 0 {
                self.player_y -= self.speed;
            }
            if input.btn(Key::Down) && self.player_y < 110 {
                self.player_y += self.speed;
            }
            if input.btn(Key::Left) && self.player_x > 0 {
                self.player_x -= self.speed;
            }
            if input.btn(Key::Right) && self.player_x < 150 {
                self.player_x += self.speed;
            }

            // Simple AI: enemy follows player
            if self.enemy_x < self.player_x {
                self.enemy_x += 1;
            } else if self.enemy_x > self.player_x {
                self.enemy_x -= 1;
            }

            if self.enemy_y < self.player_y {
                self.enemy_y += 1;
            } else if self.enemy_y > self.player_y {
                self.enemy_y -= 1;
            }
        }

        fn render(&self, ctx: &mut DrawingContext) {
            ctx.cls(0); // black background

            // Draw player (white square)
            ctx.rectfill(self.player_x, self.player_y, 8, 8, 7);

            // Draw enemy (red square)
            ctx.rectfill(self.enemy_x, self.enemy_y, 6, 6, 8);

            // Draw connection line
            ctx.line(
                self.player_x + 4,
                self.player_y + 4,
                self.enemy_x + 3,
                self.enemy_y + 3,
                3, // green
            );
        }
    }

    #[test]
    fn test_lineboy_demo() {
        let mut engine = GameEngine::new(160, 120, 60);
        let mut game = SimpleLineboy::new();

        // Simulate game loop for 10 frames
        for _ in 0..10 {
            // Input
            engine.input.press_key(Key::Up);
            engine.input.press_key(Key::Right);

            // Update
            engine.update(16.667);
            game.update(&engine.input);

            // Render
            game.render(&mut engine.drawing);

            // Frame update
            engine.input.update_frame();
        }

        // Verify player moved
        assert!(game.player_x > 80);
        assert!(game.player_y < 60);

        // Verify enemy is chasing
        assert!(game.enemy_x > 20);
        assert!(game.enemy_y < 20);

        // Verify drawing happened
        assert_eq!(engine.drawing.pget(game.player_x as i32, game.player_y as i32), Some(7));
    }

    #[test]
    fn test_lineboy_collision_detection() {
        let mut engine = GameEngine::new(160, 120, 60);
        let mut game = SimpleLineboy::new();

        // Move player and enemy to same position
        game.player_x = 50;
        game.player_y = 50;
        game.enemy_x = 50;
        game.enemy_y = 50;

        game.render(&mut engine.drawing);

        // At collision point, should see enemy pixel
        let collision_pixel = engine.drawing.pget(50, 50);
        assert!(collision_pixel.is_some());
    }

    // ============================================================
    // Palette Tests
    // ============================================================

    #[test]
    fn test_pyxel_palette() {
        assert_eq!(PYXEL_PALETTE.len(), 16);

        // Verify key colors
        assert_eq!(PYXEL_PALETTE[0], [0, 0, 0, 255]); // black
        assert_eq!(PYXEL_PALETTE[7], [255, 255, 255, 255]); // white
        assert_eq!(PYXEL_PALETTE[8], [255, 0, 77, 255]); // red
        assert_eq!(PYXEL_PALETTE[3], [0, 135, 81, 255]); // green
    }

    #[test]
    fn test_palette_customization() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);

        // Customize a color
        ctx.set_palette(0, 255, 0, 0, 255); // Change black to red
        assert_eq!(ctx.palette[0], [255, 0, 0, 255]);

        // Draw with customized palette
        ctx.pset(50, 50, 0);
        let color = ctx.pget(50, 50);
        assert_eq!(color, Some(0)); // Should still be index 0
    }
}
