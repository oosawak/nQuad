//! Cubeboy - Complete Rust Implementation
//! 
//! A challenging precision platformer with advanced movement mechanics:
//! - Coyote Time: Brief window to jump after leaving ground
//! - Jump Buffer: Early jump input is buffered
//! - Wall Slide: Slide down walls smoothly
//! - Dash: Quick directional movement (limited uses per level)
//! - Particles: Visual feedback for movement and effects

use nantaraquad::api::drawing::{DrawingContext, PYXEL_PALETTE};
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::api::particles::{Particle, ParticleSystem};
use nantaraquad::audio::AudioManager;
use std::time::Instant;

const COYOTE_TIMER_FRAMES: u32 = 6;
const JUMP_BUFFER_FRAMES: u32 = 4;
const DASH_COOLDOWN_FRAMES: u32 = 10;
const PARTICLE_LIFETIME_FRAMES: u32 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Start,
    Playing,
    Boss,
    GameOver,
    Clear,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,   // 6
    height: u32,  // 6
    is_on_ground: bool,
    is_on_wall: i32,      // -1 (left), 0 (none), 1 (right)
    can_dash: bool,
    coyote_timer: u32,    // Frames after leaving ground
    jump_buffer: u32,     // Frames of buffered jump input
    dash_cooldown: u32,   // Frames until next dash available
    jumped: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 6,
            height: 6,
            is_on_ground: true,
            is_on_wall: 0,
            can_dash: true,
            coyote_timer: COYOTE_TIMER_FRAMES,
            jump_buffer: 0,
            dash_cooldown: 0,
            jumped: false,
        }
    }

    fn update(
        &mut self,
        input: &InputState,
        dt: f32,
        world_width: u32,
        world_height: u32,
        tiles: &[Vec<bool>],
    ) -> Vec<Particle> {
        let mut particles = Vec::new();
        
        // Reset jumped flag
        self.jumped = false;

        // Horizontal input
        let mut move_x = 0.0;
        if input.btn(Key::Left) || input.btn(Key::A) {
            move_x = -1.0;
        }
        if input.btn(Key::Right) || input.btn(Key::D) {
            move_x = 1.0;
        }

        const MOVE_SPEED: f32 = 100.0;
        self.vx = move_x * MOVE_SPEED;

        // Jumping
        if input.btnp(Key::Space) {
            self.jump_buffer = JUMP_BUFFER_FRAMES;
        }

        const GRAVITY: f32 = 300.0;
        const MAX_FALL_SPEED: f32 = 200.0;
        const JUMP_FORCE: f32 = -150.0;
        const WALL_SLIDE_SPEED: f32 = 30.0;

        // Apply gravity
        if self.is_on_wall != 0 {
            // Wall slide reduces fall speed
            self.vy += GRAVITY * 0.5 * dt;
            if self.vy > WALL_SLIDE_SPEED {
                self.vy = WALL_SLIDE_SPEED;
            }
        } else {
            self.vy += GRAVITY * dt;
            if self.vy > MAX_FALL_SPEED {
                self.vy = MAX_FALL_SPEED;
            }
        }

        // Perform jump if buffer has input and coyote/ground available
        if self.jump_buffer > 0 && (self.is_on_ground || self.coyote_timer > 0 || self.is_on_wall != 0)
        {
            self.vy = JUMP_FORCE;
            self.jump_buffer = 0;
            self.is_on_ground = false;
            self.coyote_timer = 0;
            self.jumped = true;

            // Wall jump gives horizontal boost away from wall
            if self.is_on_wall != 0 {
                self.vx = -self.is_on_wall as f32 * 100.0;
            }
        }

        // Dash
        if input.btnp(Key::GamepadButtonB) || input.btnp(Key::Enter) {
            if self.can_dash && self.dash_cooldown == 0 {
                let dash_speed = 150.0;
                if move_x != 0.0 {
                    self.vx = move_x * dash_speed;
                } else if self.vx != 0.0 {
                    self.vx = if self.vx > 0.0 { dash_speed } else { -dash_speed };
                }
                self.can_dash = false;
                self.dash_cooldown = DASH_COOLDOWN_FRAMES;

                // Create dash particles
                for _ in 0..3 {
                    let angle = (rand::random::<f32>() * 6.28) as f32;
                    let speed = 50.0;
                    particles.push(Particle::new(
                        self.x + 3.0,
                        self.y + 3.0,
                        angle.cos() * speed,
                        angle.sin() * speed,
                        6,
                        PARTICLE_LIFETIME_FRAMES,
                    ));
                }
            }
        }

        // Update timers
        self.jump_buffer = self.jump_buffer.saturating_sub(1);
        self.dash_cooldown = self.dash_cooldown.saturating_sub(1);

        if !self.is_on_ground {
            self.coyote_timer = self.coyote_timer.saturating_sub(1);
        }

        // Movement
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Collision detection with tiles (simplified grid-based)
        let tile_size = 8;
        let grid_x = (self.x / tile_size as f32) as i32;
        let grid_y = (self.y / tile_size as f32) as i32;

        self.is_on_ground = false;
        self.is_on_wall = 0;

        // Check ground
        if grid_y + 1 < tiles.len() as i32 {
            if (grid_x >= 0 && grid_x < tiles[0].len() as i32)
                && tiles[(grid_y + 1) as usize][grid_x as usize]
            {
                self.is_on_ground = true;
                self.can_dash = true;
                self.coyote_timer = COYOTE_TIMER_FRAMES;
                self.vy = 0.0;
                self.y = ((grid_y + 1) as f32 * tile_size as f32) - self.height as f32;
            }
        }

        // Check walls
        if grid_x - 1 >= 0 && tiles[grid_y as usize][(grid_x - 1) as usize] && self.vx < 0.0 {
            self.is_on_wall = -1;
            self.vy *= 0.8;
        }
        if grid_x + 1 < tiles[0].len() as i32
            && tiles[grid_y as usize][(grid_x + 1) as usize]
            && self.vx > 0.0
        {
            self.is_on_wall = 1;
            self.vy *= 0.8;
        }

        // Bounds
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.width as f32 > world_width as f32 {
            self.x = world_width as f32 - self.width as f32;
        }

        particles
    }

    fn collides_with(&self, x: f32, y: f32, w: u32, h: u32) -> bool {
        self.x < x + w as f32
            && self.x + self.width as f32 > x
            && self.y < y + h as f32
            && self.y + self.height as f32 > y
    }
}

struct Boss {
    x: f32,
    y: f32,
    health: u32,
    direction: i32,
}

impl Boss {
    fn new() -> Self {
        Boss {
            x: 70.0,
            y: 30.0,
            health: 10,
            direction: 1,
        }
    }

    fn update(&mut self, _player: &Player) {
        const BOSS_SPEED: f32 = 30.0;
        let dt = 0.016;

        self.x += self.direction as f32 * BOSS_SPEED * dt;

        if self.x <= 10.0 || self.x >= 140.0 {
            self.direction = -self.direction;
        }
    }

    fn collides_with_player(&self, player: &Player) -> bool {
        player.collides_with(self.x, self.y, 12, 12)
    }
}

struct Cubeboy {
    player: Player,
    particles: ParticleSystem,
    boss: Option<Boss>,
    tiles: Vec<Vec<bool>>,
    score: u32,
    state: GameState,
    level: u32,
    start_time: Instant,
    last_frame_time: Instant,
    audio: AudioManager,
}

impl Cubeboy {
    fn new() -> Self {
        let player = Player::new(10.0, 100.0);
        let mut audio = AudioManager::new();
        
        // BGM と SFX の読み込みを試みる（ファイルが存在しない場合は無視）
        let _ = audio.load_bgm("dungeon", "assets/bgm_dungeon.wav");
        let _ = audio.load_sfx("jump", "assets/sfx_jump.wav");
        let _ = audio.load_sfx("hit", "assets/sfx_hit.wav");
        let _ = audio.load_sfx("power_up", "assets/sfx_power_up.wav");

        // Simple tilemap (8x15 grid, 8px tiles)
        let mut tiles = vec![vec![false; 20]; 15];

        // Floor
        for x in 0..20 {
            tiles[14][x] = true;
        }

        // Platforms
        tiles[12][3..6].iter_mut().for_each(|t| *t = true);
        tiles[10][8..12].iter_mut().for_each(|t| *t = true);
        tiles[8][14..18].iter_mut().for_each(|t| *t = true);

        Cubeboy {
            player,
            particles: ParticleSystem::new(256),
            boss: None,
            tiles,
            score: 0,
            state: GameState::Start,
            level: 1,
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            audio,
        }
    }

    fn update(&mut self, input: &InputState) {
        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32().min(0.016);
        self.last_frame_time = now;

        const WORLD_WIDTH: u32 = 160;
        const WORLD_HEIGHT: u32 = 120;

        match self.state {
            GameState::Start => {
                if input.btnp(Key::Space) {
                    self.state = GameState::Playing;
                    self.start_time = Instant::now();
                }
            }
            GameState::Playing => {
                let new_particles = self.player.update(
                    input,
                    dt,
                    WORLD_WIDTH,
                    WORLD_HEIGHT,
                    &self.tiles,
                );
                // Convert Vec<Particle> to ParticleSystem emissions
                for particle in new_particles {
                    self.particles.emit_with_lifetime(
                        particle.x,
                        particle.y,
                        particle.dx,
                        particle.dy,
                        particle.color,
                        particle.life,
                    );
                }
                
                // Play jump sound if player jumped
                if self.player.jumped {
                    let _ = self.audio.play_sfx("jump");
                }

                // Update particles
                self.particles.update();

                // Check level clear
                if self.player.y < 10.0 {
                    self.level += 1;
                    if self.level > 3 {
                        self.state = GameState::Boss;
                        self.boss = Some(Boss::new());
                    } else {
                        // Play coin sound on level clear
                        let _ = self.audio.play_sfx("power_up");
                        self.player = Player::new(10.0, 100.0);
                        self.score += 100;
                    }
                }

                // Check death
                if self.player.y > WORLD_HEIGHT as f32 {
                    self.state = GameState::GameOver;
                }
            }
            GameState::Boss => {
                let new_particles = self.player.update(
                    input,
                    dt,
                    WORLD_WIDTH,
                    WORLD_HEIGHT,
                    &self.tiles,
                );
                // Convert Vec<Particle> to ParticleSystem emissions
                for particle in new_particles {
                    self.particles.emit_with_lifetime(
                        particle.x,
                        particle.y,
                        particle.dx,
                        particle.dy,
                        particle.color,
                        particle.life,
                    );
                }

                // Update particles
                self.particles.update();

                if let Some(boss) = &mut self.boss {
                    boss.update(&self.player);

                    if boss.collides_with_player(&self.player) {
                        boss.health = boss.health.saturating_sub(1);
                        if boss.health == 0 {
                            self.state = GameState::Clear;
                            self.score += 500;
                        }
                    }
                }

                if self.player.y > WORLD_HEIGHT as f32 {
                    self.state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                if input.btnp(Key::Space) {
                    *self = Cubeboy::new();
                }
            }
            GameState::Clear => {
                if input.btnp(Key::Space) {
                    *self = Cubeboy::new();
                }
            }
        }
    }

    fn draw(&self, ctx: &mut DrawingContext) {
        ctx.cls(0);

        match self.state {
            GameState::Start => {
                self.draw_start(ctx);
            }
            GameState::Playing | GameState::Boss => {
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

    fn draw_start(&self, ctx: &mut DrawingContext) {
        ctx.rectfill(30, 40, 100, 30, 0);
        ctx.rectfill(32, 42, 96, 26, 3);
        ctx.rectfill(30, 85, 100, 10, 7);
    }

    fn draw_playing(&self, ctx: &mut DrawingContext) {
        // Draw tiles
        let tile_size = 8;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &is_solid) in row.iter().enumerate() {
                if is_solid {
                    ctx.rectfill(
                        (x * tile_size) as i32,
                        (y * tile_size) as i32,
                        tile_size as u32,
                        tile_size as u32,
                        5,
                    );
                }
            }
        }

        // Draw particles
        self.particles.draw(ctx);

        // Draw player
        ctx.rectfill(
            self.player.x as i32,
            self.player.y as i32,
            self.player.width,
            self.player.height,
            7,
        );

        // Draw boss if present
        if let Some(boss) = &self.boss {
            ctx.rectfill(boss.x as i32, boss.y as i32, 12, 12, 8);
            // Health indicator
            ctx.rectfill(
                (boss.x - 6.0) as i32,
                (boss.y - 10.0) as i32,
                boss.health as u32 * 2,
                2,
                3,
            );
        }

        // HUD
        ctx.rectfill(2, 2, 30, 8, 0);
        ctx.rectfill(130, 2, 28, 8, 0);
    }

    fn draw_game_over(&self, ctx: &mut DrawingContext) {
        ctx.rectfill(40, 40, 80, 30, 0);
        ctx.rectfill(42, 42, 76, 26, 8);
        ctx.rectfill(35, 80, 90, 10, 7);
    }

    fn draw_clear(&self, ctx: &mut DrawingContext) {
        ctx.rectfill(30, 35, 100, 40, 0);
        ctx.rectfill(32, 37, 96, 36, 3);
        ctx.rectfill(35, 85, 90, 10, 7);
    }
}

fn main() {
    let mut game = Cubeboy::new();
    let mut input = InputState::new();
    const WORLD_WIDTH: u32 = 160;
    const WORLD_HEIGHT: u32 = 120;

    loop {
        let _now = Instant::now();

        input.update_frame();
        game.update(&input);

        let mut ctx = DrawingContext::new(
            WORLD_WIDTH,
            WORLD_HEIGHT,
            PYXEL_PALETTE.to_vec(),
        );

        game.draw(&mut ctx);

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

// Simple random number generator
mod rand {
    pub fn random<T>() -> T
    where
        T: From<f32>,
    {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let x = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) / 65536) % 32768;
        T::from((x as f32) / 32768.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(50.0, 100.0);
        assert_eq!(player.x, 50.0);
        assert_eq!(player.y, 100.0);
        assert_eq!(player.width, 6);
        assert_eq!(player.height, 6);
    }

    #[test]
    fn test_coyote_timer() {
        let player = Player::new(50.0, 100.0);
        assert_eq!(player.coyote_timer, COYOTE_TIMER_FRAMES);
    }

    #[test]
    fn test_particle_lifetime() {
        let mut particle = Particle::new(10.0, 10.0, 1.0, 1.0, 5, PARTICLE_LIFETIME_FRAMES);
        assert!(particle.is_alive());
        assert_eq!(particle.life, PARTICLE_LIFETIME_FRAMES);

        for _ in 0..PARTICLE_LIFETIME_FRAMES {
            particle.update();
        }
        assert!(!particle.is_alive());
    }

    #[test]
    fn test_particle_physics() {
        let mut particle = Particle::new(50.0, 50.0, 10.0, 0.0, 5, PARTICLE_LIFETIME_FRAMES);
        let initial_x = particle.x;
        let initial_y = particle.y;

        particle.update();

        assert!(particle.x > initial_x); // Moved by dx
        assert!(particle.y > initial_y); // Gravity applied
    }

    #[test]
    fn test_boss_creation() {
        let boss = Boss::new();
        assert_eq!(boss.health, 10);
        assert!(boss.x > 0.0 && boss.x < 160.0);
    }

    #[test]
    fn test_boss_movement() {
        let mut boss = Boss::new();
        let initial_x = boss.x;
        let player = Player::new(100.0, 100.0);

        boss.update(&player);
        assert!(boss.x != initial_x);
    }

    #[test]
    fn test_collision_detection() {
        let player = Player::new(50.0, 100.0);
        let collides = player.collides_with(52.0, 102.0, 4, 4);
        assert!(collides);

        let no_collide = player.collides_with(100.0, 100.0, 4, 4);
        assert!(!no_collide);
    }

    #[test]
    fn test_cubeboy_initialization() {
        let game = Cubeboy::new();
        assert_eq!(game.state, GameState::Start);
        assert_eq!(game.score, 0);
        assert_eq!(game.level, 1);
        assert_eq!(game.player.x, 10.0);
        assert_eq!(game.player.y, 100.0);
    }

    #[test]
    fn test_tilemap_generation() {
        let game = Cubeboy::new();
        assert_eq!(game.tiles.len(), 15);
        assert_eq!(game.tiles[0].len(), 20);
        // Check floor exists
        assert!(game.tiles[14].iter().all(|&t| t));
    }
}
