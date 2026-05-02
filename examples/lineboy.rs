//! Lineboy - Complete Rust Implementation
//! 
//! A classic action platformer where you must jump and dodge enemies
//! to progress through five different themed worlds.

use nantaraquad::api::drawing::{DrawingContext, PYXEL_PALETTE};
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::audio::AudioManager;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    GameOver,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Theme {
    Forest,    // Green
    Desert,    // Yellow/Brown
    Cave,      // Gray/Blue
    Ghost,     // Purple/White
    Volcano,   // Red/Orange
}

impl Theme {
    fn bg_color(&self) -> u8 {
        match self {
            Theme::Forest => 2,   // Green
            Theme::Desert => 4,   // Yellow
            Theme::Cave => 1,     // Blue
            Theme::Ghost => 13,   // Purple
            Theme::Volcano => 8,  // Red
        }
    }

    fn enemy_color(&self) -> u8 {
        match self {
            Theme::Forest => 3,   // Dark green
            Theme::Desert => 5,   // Brown
            Theme::Cave => 5,     // Dark blue
            Theme::Ghost => 7,    // White
            Theme::Volcano => 9,  // Dark red
        }
    }

    fn next(&self) -> Option<Theme> {
        match self {
            Theme::Forest => Some(Theme::Desert),
            Theme::Desert => Some(Theme::Cave),
            Theme::Cave => Some(Theme::Ghost),
            Theme::Ghost => Some(Theme::Volcano),
            Theme::Volcano => None,
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,
    height: u32,
    is_on_ground: bool,
    was_on_ground: bool,
    jumped: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 4,
            height: 4,
            is_on_ground: false,
            was_on_ground: false,
            jumped: false,
        }
    }

    fn update(&mut self, input: &InputState, dt: f32, world_width: u32, world_height: u32) {
        // Reset jumped flag at start of frame
        self.jumped = false;
        self.was_on_ground = self.is_on_ground;
        
        // Horizontal movement
        const MOVE_SPEED: f32 = 80.0;
        let mut move_x = 0.0;

        if input.btn(Key::Left) || input.btn(Key::A) {
            move_x = -MOVE_SPEED;
        }
        if input.btn(Key::Right) || input.btn(Key::D) {
            move_x = MOVE_SPEED;
        }

        self.vx = move_x;
        self.x += self.vx * dt;

        // Gravity
        const GRAVITY: f32 = 300.0;
        const MAX_FALL_SPEED: f32 = 200.0;
        const JUMP_FORCE: f32 = -150.0;

        self.vy += GRAVITY * dt;
        if self.vy > MAX_FALL_SPEED {
            self.vy = MAX_FALL_SPEED;
        }

        self.y += self.vy * dt;

        // Jump
        if input.btnp(Key::Space) && self.is_on_ground {
            self.vy = JUMP_FORCE;
            self.is_on_ground = false;
            self.jumped = true;
        }

        // Ground collision
        const GROUND_Y: f32 = 110.0;
        if self.y + self.height as f32 >= GROUND_Y {
            self.y = GROUND_Y - self.height as f32;
            self.vy = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }

        // Bounds checking
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.width as f32 > world_width as f32 {
            self.x = world_width as f32 - self.width as f32;
        }
    }

    fn collides_with(&self, other_x: f32, other_y: f32, other_w: u32, other_h: u32) -> bool {
        self.x < other_x + other_w as f32
            && self.x + self.width as f32 > other_x
            && self.y < other_y + other_h as f32
            && self.y + self.height as f32 > other_y
    }
}

struct Enemy {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    direction: i32, // -1 or 1
}

impl Enemy {
    fn new(x: f32, y: f32, direction: i32) -> Self {
        Enemy {
            x,
            y,
            width: 4,
            height: 4,
            direction,
        }
    }

    fn update(&mut self, dt: f32) {
        const ENEMY_SPEED: f32 = 40.0;
        self.x += self.direction as f32 * ENEMY_SPEED * dt;
    }

    fn should_bounce(&self, world_width: u32) -> bool {
        (self.direction == -1 && self.x <= 0.0)
            || (self.direction == 1 && self.x + self.width as f32 >= world_width as f32)
    }
}

struct Lineboy {
    player: Player,
    enemies: Vec<Enemy>,
    score: u32,
    state: GameState,
    current_theme_idx: usize,
    enemies_defeated: u32,
    start_time: Instant,
    last_frame_time: Instant,
    audio: AudioManager,
}

impl Lineboy {
    fn new() -> Self {
        let player = Player::new(75.0, 100.0);
        let mut audio = AudioManager::new();
        
        // BGM と SFX の読み込みを試みる（ファイルが存在しない場合は無視）
        let _ = audio.load_bgm("forest", "assets/bgm_forest.wav");
        let _ = audio.load_sfx("jump", "assets/sfx_jump.wav");
        let _ = audio.load_sfx("hit", "assets/sfx_hit.wav");
        let _ = audio.load_sfx("coin", "assets/sfx_coin.wav");

        Lineboy {
            player,
            enemies: vec![
                Enemy::new(30.0, 100.0, 1),
                Enemy::new(130.0, 100.0, -1),
                Enemy::new(60.0, 80.0, 1),
            ],
            score: 0,
            state: GameState::Title,
            current_theme_idx: 0,
            enemies_defeated: 0,
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            audio,
        }
    }

    fn current_theme(&self) -> Theme {
        match self.current_theme_idx {
            0 => Theme::Forest,
            1 => Theme::Desert,
            2 => Theme::Cave,
            3 => Theme::Ghost,
            _ => Theme::Volcano,
        }
    }

    fn update(&mut self, input: &InputState) {
        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32().min(0.016); // Cap at ~60fps
        self.last_frame_time = now;

        match self.state {
            GameState::Title => {
                if input.btnp(Key::Space) {
                    self.state = GameState::Playing;
                    self.start_time = Instant::now();
                }
            }
            GameState::Playing => {
                const WORLD_WIDTH: u32 = 160;
                const WORLD_HEIGHT: u32 = 120;
                
                self.player.update(input, dt, WORLD_WIDTH, WORLD_HEIGHT);
                
                // Play jump sound if player jumped
                if self.player.jumped {
                    let _ = self.audio.play_sfx("jump");
                }

                for enemy in &mut self.enemies {
                    enemy.update(dt);
                    if enemy.should_bounce(WORLD_WIDTH) {
                        enemy.direction = -enemy.direction;
                    }
                }

                // Collision detection with enemies
                for enemy in &self.enemies {
                    if self.player.collides_with(enemy.x, enemy.y, enemy.width, enemy.height) {
                        // Play hit sound on collision
                        let _ = self.audio.play_sfx("hit");
                        self.state = GameState::GameOver;
                        break;
                    }
                }

                // Advance to next level
                if self.player.y < 10.0 {
                    if let Some(next_theme) = self.current_theme().next() {
                        self.current_theme_idx += 1;
                        self.enemies_defeated += 1;
                        self.score += 100;
                        
                        // Play coin sound on advance
                        let _ = self.audio.play_sfx("coin");

                        // Reset player position
                        self.player = Player::new(75.0, 100.0);

                        // Increase difficulty
                        let enemy_count = 3 + self.enemies_defeated as usize;
                        self.enemies.clear();
                        for i in 0..enemy_count {
                            let x = 20.0 + (i as f32) * 30.0;
                            let direction = if i % 2 == 0 { 1 } else { -1 };
                            self.enemies.push(Enemy::new(x, 100.0, direction));
                        }
                    } else {
                        self.state = GameState::Clear;
                        self.score += 500;
                    }
                }
            }
            GameState::GameOver => {
                if input.btnp(Key::Space) {
                    *self = Lineboy::new();
                }
            }
            GameState::Clear => {
                if input.btnp(Key::Space) {
                    *self = Lineboy::new();
                }
            }
        }
    }

    fn draw(&self, ctx: &mut DrawingContext) {
        let theme = self.current_theme();

        // Background
        ctx.cls(theme.bg_color());

        match self.state {
            GameState::Title => {
                self.draw_title(ctx);
            }
            GameState::Playing => {
                self.draw_playing(ctx);
            }
            GameState::GameOver => {
                self.draw_game_over(ctx);
            }
            GameState::Clear => {
                self.draw_clear(ctx);
            }
        }
    }

    fn draw_title(&self, ctx: &mut DrawingContext) {
        // Title background
        ctx.rectfill(40, 30, 80, 20, 7);
        ctx.rectfill(42, 32, 76, 16, 0);
        
        // Title text
        ctx.print("LINEBOY", 62, 35, 7);

        // Instructions background
        ctx.rectfill(20, 70, 120, 10, 7);
        
        // Instructions text
        ctx.print("Press Z or SPACE to Start", 22, 73, 0);

        // Score background
        ctx.rectfill(50, 100, 60, 8, 7);
        
        // Best score text
        let best_score = format!("BEST: {}", self.score);
        ctx.print(&best_score, 52, 103, 0);
    }

    fn draw_playing(&self, ctx: &mut DrawingContext) {
        let theme = self.current_theme();

        // Draw player
        ctx.rectfill(
            self.player.x as i32,
            self.player.y as i32,
            self.player.width,
            self.player.height,
            7,
        );

        // Draw enemies
        for enemy in &self.enemies {
            ctx.rectfill(
                enemy.x as i32,
                enemy.y as i32,
                enemy.width,
                enemy.height,
                theme.enemy_color(),
            );
        }

        // Draw ground
        ctx.rectfill(0, 115, 160, 5, 5);

        // HUD - Level and Score
        ctx.rectfill(2, 2, 30, 8, 0);
        ctx.rectfill(130, 2, 28, 8, 0);
        
        // Level text
        let level_str = format!("L:{}", self.level);
        ctx.print(&level_str, 4, 4, 7);
        
        // Score text
        let score_str = format!("S:{}", self.score);
        ctx.print(&score_str, 132, 4, 7);
    }

    fn draw_game_over(&self, ctx: &mut DrawingContext) {
        // Game Over screen
        ctx.rectfill(40, 40, 80, 20, 0);
        ctx.rectfill(42, 42, 76, 16, 8);
        
        // Game Over text
        ctx.print("GAME OVER", 51, 45, 7);

        ctx.rectfill(35, 70, 90, 10, 7);
        ctx.rectfill(37, 72, 86, 6, 0);
        
        // Continue text
        ctx.print("Press Z to Retry", 41, 74, 7);

        let score_str = format!("SCORE: {}", self.score);
        ctx.rectfill(50, 100, 60, 8, 7);
        
        // Score text
        ctx.print(&score_str, 52, 103, 0);
    }

    fn draw_clear(&self, ctx: &mut DrawingContext) {
        // Clear screen
        ctx.rectfill(30, 35, 100, 30, 7);
        ctx.rectfill(32, 37, 96, 26, 0);
        
        // Clear text
        ctx.print("STAGE CLEAR", 49, 40, 7);

        ctx.rectfill(35, 75, 90, 10, 7);
        ctx.rectfill(37, 77, 86, 6, 0);
        
        // Continue text
        ctx.print("Press Z for Next", 43, 78, 0);

        let final_score = format!("FINAL: {}", self.score);
        ctx.rectfill(45, 100, 70, 8, 7);
        
        // Score text
        ctx.print(&final_score, 47, 103, 0);
    }
}

fn main() {
    let mut game = Lineboy::new();
    let mut input = InputState::new();
    const WORLD_WIDTH: u32 = 160;
    const WORLD_HEIGHT: u32 = 120;

    loop {
        let _now = Instant::now();

        // Update input
        input.update_frame();
        
        // Update game
        game.update(&input);

        // Create drawing context
        let mut ctx = DrawingContext::new(
            WORLD_WIDTH,
            WORLD_HEIGHT,
            PYXEL_PALETTE.to_vec(),
        );

        game.draw(&mut ctx);

        // In a real game loop with macroquad, we'd render ctx to screen
        // For now, this serves as a framework that can be integrated with macroquad
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(75.0, 100.0);
        assert_eq!(player.x, 75.0);
        assert_eq!(player.y, 100.0);
        assert_eq!(player.width, 4);
        assert_eq!(player.height, 4);
    }

    #[test]
    fn test_player_gravity() {
        let mut player = Player::new(75.0, 50.0);
        let input = InputState::new();
        player.update(&input, 0.016, 160, 120);
        assert!(player.y > 50.0); // Gravity should pull down
    }

    #[test]
    fn test_player_jump() {
        let mut player = Player::new(75.0, 110.0);
        let mut input = InputState::new();
        player.is_on_ground = true;
        input.press_key(Key::Space);
        player.update(&input, 0.016, 160, 120);
        assert!(player.vy < 0.0); // Jump force applied
    }

    #[test]
    fn test_collision_detection() {
        let player = Player::new(50.0, 100.0);
        let collides = player.collides_with(52.0, 102.0, 2, 2);
        assert!(collides);

        let no_collide = player.collides_with(100.0, 100.0, 2, 2);
        assert!(!no_collide);
    }

    #[test]
    fn test_enemy_movement() {
        let mut enemy = Enemy::new(50.0, 100.0, 1);
        let initial_x = enemy.x;
        enemy.update(0.016);
        assert!(enemy.x > initial_x);
    }

    #[test]
    fn test_theme_progression() {
        let forest = Theme::Forest;
        assert_eq!(forest.next(), Some(Theme::Desert));

        let volcano = Theme::Volcano;
        assert_eq!(volcano.next(), None);
    }

    #[test]
    fn test_lineboy_initialization() {
        let game = Lineboy::new();
        assert_eq!(game.state, GameState::Title);
        assert_eq!(game.score, 0);
        assert_eq!(game.current_theme_idx, 0);
        assert_eq!(game.player.x, 75.0);
        assert!(game.enemies.len() >= 3);
    }
}
