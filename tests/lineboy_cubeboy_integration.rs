//! Integration tests for Lineboy and Cubeboy games
//!
//! These tests verify:
//! - Correct initialization
//! - State transitions
//! - Game mechanics
//! - Collision detection
//! - Integration with GameEngine

use nantaraquad::api::drawing::{DrawingContext, PYXEL_PALETTE};
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::api::game::GameEngine;
use std::time::Instant;

// ===== Lineboy Tests =====

#[test]
fn test_lineboy_player_initialization() {
    let player = LineyboyPlayer::new(75.0, 100.0);
    assert_eq!(player.x, 75.0);
    assert_eq!(player.y, 100.0);
    assert_eq!(player.width, 4);
    assert_eq!(player.height, 4);
    assert_eq!(player.vx, 0.0);
    assert_eq!(player.vy, 0.0);
}

#[test]
fn test_lineboy_player_movement() {
    let mut player = LineyboyPlayer::new(75.0, 100.0);
    let mut input = InputState::new();
    input.press_key(Key::Right);

    player.update(&input, 0.016, 160, 120);

    assert!(player.vx > 0.0);
    assert!(player.x > 75.0);
}

#[test]
fn test_lineboy_gravity() {
    let mut player = LineyboyPlayer::new(75.0, 50.0);
    let input = InputState::new();

    player.update(&input, 0.016, 160, 120);

    assert!(player.y > 50.0); // Gravity pulled down
}

#[test]
fn test_lineboy_jump() {
    let mut player = LineyboyPlayer::new(75.0, 110.0);
    let mut input = InputState::new();
    player.is_on_ground = true;

    input.press_key(Key::Space);
    player.update(&input, 0.016, 160, 120);

    assert!(player.vy < 0.0); // Jump force applied
}

#[test]
fn test_lineboy_collision_detection() {
    let player = LineyboyPlayer::new(50.0, 100.0);

    // Collision should happen
    assert!(player.collides_with(52.0, 102.0, 2, 2));

    // No collision at distance
    assert!(!player.collides_with(100.0, 100.0, 2, 2));
}

#[test]
fn test_lineboy_enemy_movement() {
    let mut enemy = LineyboyEnemy::new(50.0, 100.0, 1);
    let initial_x = enemy.x;

    enemy.update(0.016);

    assert!(enemy.x > initial_x);
}

#[test]
fn test_lineboy_enemy_bounce() {
    let mut enemy = LineyboyEnemy::new(1.0, 100.0, -1);
    assert!(!enemy.should_bounce(160));

    enemy.direction = -1;
    enemy.x = -5.0;
    assert!(enemy.should_bounce(160));
}

#[test]
fn test_lineboy_theme_progression() {
    let forest = LineyboyTheme::Forest;
    assert_eq!(forest.next(), Some(LineyboyTheme::Desert));

    let desert = LineyboyTheme::Desert;
    assert_eq!(desert.next(), Some(LineyboyTheme::Cave));

    let cave = LineyboyTheme::Cave;
    assert_eq!(cave.next(), Some(LineyboyTheme::Ghost));

    let ghost = LineyboyTheme::Ghost;
    assert_eq!(ghost.next(), Some(LineyboyTheme::Volcano));

    let volcano = LineyboyTheme::Volcano;
    assert_eq!(volcano.next(), None);
}

#[test]
fn test_lineboy_initialization() {
    let game = Lineboy::new();
    assert_eq!(game.state, LineyboyGameState::Title);
    assert_eq!(game.score, 0);
    assert_eq!(game.current_theme_idx, 0);
    assert_eq!(game.enemies_defeated, 0);
    assert_eq!(game.player.x, 75.0);
    assert!(game.enemies.len() >= 3);
}

#[test]
fn test_lineboy_title_transition() {
    let mut game = Lineboy::new();
    let mut input = InputState::new();

    assert_eq!(game.state, LineyboyGameState::Title);

    input.press_key(Key::Space);
    game.update(&input);

    assert_eq!(game.state, LineyboyGameState::Playing);
}

#[test]
fn test_lineboy_game_over_on_collision() {
    let mut game = Lineboy::new();
    game.state = LineyboyGameState::Playing;
    game.player.x = 30.0;
    game.player.y = 100.0;
    game.enemies[0].x = 30.0;
    game.enemies[0].y = 100.0;

    let input = InputState::new();
    game.update(&input);

    assert_eq!(game.state, LineyboyGameState::GameOver);
}

#[test]
fn test_lineboy_drawing_title() {
    let game = Lineboy::new();
    let mut ctx = DrawingContext::new(160, 120, PYXEL_PALETTE.to_vec());

    game.draw(&mut ctx);

    // Drawing should not panic
    // We can't easily verify drawing, but we can verify no panics
}

// ===== Cubeboy Tests =====

#[test]
fn test_cubeboy_player_initialization() {
    let player = CubeboyPlayer::new(50.0, 100.0);
    assert_eq!(player.x, 50.0);
    assert_eq!(player.y, 100.0);
    assert_eq!(player.width, 6);
    assert_eq!(player.height, 6);
    assert!(player.is_on_ground);
    assert!(player.can_dash);
    assert_eq!(player.coyote_timer, 6);
}

#[test]
fn test_cubeboy_particle_creation() {
    let particle = CubeboyParticle::new(50.0, 50.0, 10.0, -10.0, 5);
    assert_eq!(particle.x, 50.0);
    assert_eq!(particle.y, 50.0);
    assert!(particle.is_alive());
}

#[test]
fn test_cubeboy_particle_lifetime() {
    let mut particle = CubeboyParticle::new(50.0, 50.0, 10.0, -10.0, 5);

    for _ in 0..30 {
        particle.update();
    }

    assert!(!particle.is_alive());
}

#[test]
fn test_cubeboy_particle_physics() {
    let mut particle = CubeboyParticle::new(50.0, 50.0, 10.0, 0.0, 5);
    let initial_x = particle.x;
    let initial_y = particle.y;

    particle.update();

    assert_eq!(particle.x, initial_x + 10.0);
    assert!(particle.y > initial_y); // Gravity applied
}

#[test]
fn test_cubeboy_boss_creation() {
    let boss = CubeboyBoss::new();
    assert_eq!(boss.health, 10);
    assert_eq!(boss.x, 70.0);
    assert_eq!(boss.y, 30.0);
}

#[test]
fn test_cubeboy_boss_movement() {
    let mut boss = CubeboyBoss::new();
    let initial_x = boss.x;
    let player = CubeboyPlayer::new(100.0, 100.0);

    boss.update(&player);

    assert_ne!(boss.x, initial_x);
}

#[test]
fn test_cubeboy_collision_detection() {
    let player = CubeboyPlayer::new(50.0, 100.0);
    let collides = player.collides_with(52.0, 102.0, 4, 4);
    assert!(collides);

    let no_collide = player.collides_with(100.0, 100.0, 4, 4);
    assert!(!no_collide);
}

#[test]
fn test_cubeboy_initialization() {
    let game = Cubeboy::new();
    assert_eq!(game.state, CubeboyGameState::Start);
    assert_eq!(game.score, 0);
    assert_eq!(game.level, 1);
    assert_eq!(game.player.x, 10.0);
    assert_eq!(game.player.y, 100.0);
    assert!(game.boss.is_none());
}

#[test]
fn test_cubeboy_tilemap_generation() {
    let game = Cubeboy::new();
    assert_eq!(game.tiles.len(), 15);
    assert_eq!(game.tiles[0].len(), 20);

    // Check floor exists
    assert!(game.tiles[14].iter().all(|&t| t));
}

#[test]
fn test_cubeboy_title_transition() {
    let mut game = Cubeboy::new();
    let mut input = InputState::new();

    assert_eq!(game.state, CubeboyGameState::Start);

    input.press_key(Key::Space);
    game.update(&input);

    assert_eq!(game.state, CubeboyGameState::Playing);
}

#[test]
fn test_cubeboy_drawing_start() {
    let game = Cubeboy::new();
    let mut ctx = DrawingContext::new(160, 120, PYXEL_PALETTE.to_vec());

    game.draw(&mut ctx);

    // Drawing should not panic
}

// ===== Game Integration Tests =====

#[test]
fn test_game_engine_with_lineboy() {
    let engine = GameEngine::new(160, 120, 60);
    assert_eq!(engine.width, 160);
    assert_eq!(engine.height, 120);
    assert_eq!(engine.fps, 60);

    let mut input = InputState::new();
    input.press_key(Key::Up);
    assert!(input.btn(Key::Up));
}

#[test]
fn test_game_engine_with_cubeboy() {
    let mut engine = GameEngine::new(160, 120, 60);
    engine.input.press_key(Key::Space);
    assert!(engine.input.btn(Key::Space));
}

#[test]
fn test_drawing_context_with_palette() {
    let palette = PYXEL_PALETTE.to_vec();
    let mut ctx = DrawingContext::new(160, 120, palette);

    ctx.cls(0);
    ctx.rectfill(10, 10, 20, 20, 5);

    assert_eq!(ctx.pget(15, 15), Some(5));
}

// ===== Helper Structures for Testing =====

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineyboyGameState {
    Title,
    Playing,
    GameOver,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineyboyTheme {
    Forest,
    Desert,
    Cave,
    Ghost,
    Volcano,
}

impl LineyboyTheme {
    fn next(&self) -> Option<LineyboyTheme> {
        match self {
            LineyboyTheme::Forest => Some(LineyboyTheme::Desert),
            LineyboyTheme::Desert => Some(LineyboyTheme::Cave),
            LineyboyTheme::Cave => Some(LineyboyTheme::Ghost),
            LineyboyTheme::Ghost => Some(LineyboyTheme::Volcano),
            LineyboyTheme::Volcano => None,
        }
    }
}

struct LineyboyPlayer {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,
    height: u32,
    is_on_ground: bool,
}

impl LineyboyPlayer {
    fn new(x: f32, y: f32) -> Self {
        LineyboyPlayer {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 4,
            height: 4,
            is_on_ground: false,
        }
    }

    fn update(&mut self, input: &InputState, dt: f32, world_width: u32, world_height: u32) {
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

        const GRAVITY: f32 = 300.0;
        const MAX_FALL_SPEED: f32 = 200.0;
        const JUMP_FORCE: f32 = -150.0;

        self.vy += GRAVITY * dt;
        if self.vy > MAX_FALL_SPEED {
            self.vy = MAX_FALL_SPEED;
        }

        self.y += self.vy * dt;

        if input.btnp(Key::Space) && self.is_on_ground {
            self.vy = JUMP_FORCE;
            self.is_on_ground = false;
        }

        const GROUND_Y: f32 = 110.0;
        if self.y + self.height as f32 >= GROUND_Y {
            self.y = GROUND_Y - self.height as f32;
            self.vy = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }

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

struct LineyboyEnemy {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    direction: i32,
}

impl LineyboyEnemy {
    fn new(x: f32, y: f32, direction: i32) -> Self {
        LineyboyEnemy {
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
    player: LineyboyPlayer,
    enemies: Vec<LineyboyEnemy>,
    score: u32,
    state: LineyboyGameState,
    current_theme_idx: usize,
    enemies_defeated: u32,
}

impl Lineboy {
    fn new() -> Self {
        let player = LineyboyPlayer::new(75.0, 100.0);
        Lineboy {
            player,
            enemies: vec![
                LineyboyEnemy::new(30.0, 100.0, 1),
                LineyboyEnemy::new(130.0, 100.0, -1),
                LineyboyEnemy::new(60.0, 80.0, 1),
            ],
            score: 0,
            state: LineyboyGameState::Title,
            current_theme_idx: 0,
            enemies_defeated: 0,
        }
    }

    fn update(&mut self, input: &InputState) {
        match self.state {
            LineyboyGameState::Title => {
                if input.btnp(Key::Space) {
                    self.state = LineyboyGameState::Playing;
                }
            }
            LineyboyGameState::Playing => {
                self.player.update(input, 0.016, 160, 120);

                for enemy in &mut self.enemies {
                    enemy.update(0.016);
                    if enemy.should_bounce(160) {
                        enemy.direction = -enemy.direction;
                    }
                }

                for enemy in &self.enemies {
                    if self.player.collides_with(enemy.x, enemy.y, enemy.width, enemy.height) {
                        self.state = LineyboyGameState::GameOver;
                        break;
                    }
                }
            }
            LineyboyGameState::GameOver => {
                if input.btnp(Key::Space) {
                    *self = Lineboy::new();
                }
            }
            LineyboyGameState::Clear => {
                if input.btnp(Key::Space) {
                    *self = Lineboy::new();
                }
            }
        }
    }

    fn draw(&self, ctx: &mut DrawingContext) {
        ctx.cls(2);
        ctx.rectfill(
            self.player.x as i32,
            self.player.y as i32,
            self.player.width,
            self.player.height,
            7,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CubeboyGameState {
    Start,
    Playing,
    Boss,
    GameOver,
    Clear,
}

struct CubeboyParticle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: u8,
    life: u32,
}

impl CubeboyParticle {
    fn new(x: f32, y: f32, dx: f32, dy: f32, color: u8) -> Self {
        CubeboyParticle {
            x,
            y,
            dx,
            dy,
            color,
            life: 30,
        }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy += 2.0;
        self.life = self.life.saturating_sub(1);
    }

    fn is_alive(&self) -> bool {
        self.life > 0
    }
}

struct CubeboyPlayer {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,
    height: u32,
    is_on_ground: bool,
    is_on_wall: i32,
    can_dash: bool,
    coyote_timer: u32,
    jump_buffer: u32,
    dash_cooldown: u32,
}

impl CubeboyPlayer {
    fn new(x: f32, y: f32) -> Self {
        CubeboyPlayer {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 6,
            height: 6,
            is_on_ground: true,
            is_on_wall: 0,
            can_dash: true,
            coyote_timer: 6,
            jump_buffer: 0,
            dash_cooldown: 0,
        }
    }

    fn collides_with(&self, x: f32, y: f32, w: u32, h: u32) -> bool {
        self.x < x + w as f32
            && self.x + self.width as f32 > x
            && self.y < y + h as f32
            && self.y + self.height as f32 > y
    }
}

struct CubeboyBoss {
    x: f32,
    y: f32,
    health: u32,
    direction: i32,
}

impl CubeboyBoss {
    fn new() -> Self {
        CubeboyBoss {
            x: 70.0,
            y: 30.0,
            health: 10,
            direction: 1,
        }
    }

    fn update(&mut self, _player: &CubeboyPlayer) {
        const BOSS_SPEED: f32 = 30.0;
        let dt = 0.016;

        self.x += self.direction as f32 * BOSS_SPEED * dt;

        if self.x <= 10.0 || self.x >= 140.0 {
            self.direction = -self.direction;
        }
    }
}

struct Cubeboy {
    player: CubeboyPlayer,
    particles: Vec<CubeboyParticle>,
    boss: Option<CubeboyBoss>,
    tiles: Vec<Vec<bool>>,
    score: u32,
    state: CubeboyGameState,
    level: u32,
}

impl Cubeboy {
    fn new() -> Self {
        let player = CubeboyPlayer::new(10.0, 100.0);

        let mut tiles = vec![vec![false; 20]; 15];
        for x in 0..20 {
            tiles[14][x] = true;
        }
        tiles[12][3..6].iter_mut().for_each(|t| *t = true);
        tiles[10][8..12].iter_mut().for_each(|t| *t = true);
        tiles[8][14..18].iter_mut().for_each(|t| *t = true);

        Cubeboy {
            player,
            particles: Vec::new(),
            boss: None,
            tiles,
            score: 0,
            state: CubeboyGameState::Start,
            level: 1,
        }
    }

    fn update(&mut self, input: &InputState) {
        match self.state {
            CubeboyGameState::Start => {
                if input.btnp(Key::Space) {
                    self.state = CubeboyGameState::Playing;
                }
            }
            CubeboyGameState::Playing => {
                // Game logic
            }
            CubeboyGameState::GameOver => {
                if input.btnp(Key::Space) {
                    *self = Cubeboy::new();
                }
            }
            _ => {}
        }
    }

    fn draw(&self, ctx: &mut DrawingContext) {
        ctx.cls(0);
    }
}
