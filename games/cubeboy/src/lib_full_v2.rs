use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Mutex;

thread_local! {
    static FRAMEBUFFER: RefCell<Vec<u32>> = RefCell::new(vec![0; 128 * 128]);
}

// Game Constants
const W: u32 = 128;
const H: u32 = 128;
const TILE_SIZE: u32 = 8;
const GRID_W: u32 = W / TILE_SIZE;
const GRID_H: u32 = H / TILE_SIZE;

// Colors (from Pyxel palette index)
const COLOR_BG: u32 = 0x000000;
const COLOR_WALL: u32 = 0x1D2B53;
const COLOR_PLAYER_READY: u32 = 0x00FF00;
const COLOR_PLAYER_SPENT: u32 = 0xFF0000;
const COLOR_PARTICLE: u32 = 0xFF0000;
const COLOR_ORB: u32 = 0xFFFF00;
const COLOR_SPIKE: u32 = 0xFF6600;
const COLOR_SPIKE_2: u32 = 0xFF0000;
const COLOR_BOSS: u32 = 0xFF00FF;

// Game States
const STATE_START: i32 = 0;
const STATE_PLAY: i32 = 1;
const STATE_BOSS: i32 = 2;
const STATE_GAMECLEAR: i32 = 3;
const STATE_GAMEOVER: i32 = 4;
const STATE_GAMEOVER_SEQ: i32 = 5;

// Physics
const GRAVITY: f32 = 1.6;
const MOVE_SPEED: f32 = 8.0;
const MAX_FALL: f32 = 20.0;
const JUMP_POWER: f32 = -40.0;

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: u32,
    lifetime: i32,
}

impl Particle {
    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 0.2;
        self.lifetime -= 1;
    }

    fn is_alive(&self) -> bool {
        self.lifetime > 0
    }
}

#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    can_jump: bool,
    dash_count: i32,
    wall_slide_x: Option<f32>,
    jump_pressed_last: bool,
    last_direction: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            x: 64.0,
            y: 64.0,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            can_jump: true,
            dash_count: 0,
            wall_slide_x: None,
            jump_pressed_last: false,
            last_direction: 1.0,
        }
    }

    fn reset(&mut self) {
        self.x = 64.0;
        self.y = 64.0;
        self.vx = 0.0;
        self.vy = 0.0;
        self.on_ground = false;
        self.dash_count = 0;
        self.wall_slide_x = None;
    }

    fn update(&mut self, tilemap: &Tilemap, particles: &mut Vec<Particle>, input: &InputState) {
        let old_x = self.x;
        let old_y = self.y;

        // Input
        if input.left {
            self.vx = -MOVE_SPEED;
            self.last_direction = -1.0;
        } else if input.right {
            self.vx = MOVE_SPEED;
            self.last_direction = 1.0;
        } else {
            self.vx = 0.0;
        }

        if input.jump && !self.jump_pressed_last && self.can_jump {
            self.vy = JUMP_POWER;
            self.can_jump = false;
            self.jump_pressed_last = true;
        } else if !input.jump {
            self.jump_pressed_last = false;
        }

        self.jump_pressed_last = input.jump;

        // Gravity & Fall
        self.vy = (self.vy + GRAVITY).min(MAX_FALL);

        // X Movement & Collision
        self.x += self.vx;
        if self.collides_tile(tilemap) {
            self.x = old_x;
        }

        // Y Movement & Collision
        self.y += self.vy;
        if self.collides_tile(tilemap) {
            self.y = old_y;
            if self.vy > 0.0 {
                self.on_ground = true;
                self.can_jump = true;
                self.vy = 0.0;
            } else {
                self.vy = 0.0;
            }
        } else {
            self.on_ground = false;
        }

        // Spike collision (damage)
        if self.collides_spike(tilemap) {
            // Emit particles
            for _ in 0..8 {
                let r1 = Self::rand_f32() * 628.0;
                let r2 = Self::rand_f32() * 100.0;
                let angle = r1 / 100.0;
                let speed = 0.1 + r2 / 200.0;
                particles.push(Particle {
                    x: self.x + 8.0,
                    y: self.y + 8.0,
                    vx: angle.cos() * speed,
                    vy: angle.sin() * speed,
                    color: COLOR_PARTICLE,
                    lifetime: 300,
                });
            }
            self.reset();
        }
    }

    fn collides_tile(&self, tilemap: &Tilemap) -> bool {
        let x1 = (self.x as u32) / TILE_SIZE;
        let y1 = (self.y as u32) / TILE_SIZE;
        let x2 = ((self.x + 15.0) as u32) / TILE_SIZE;
        let y2 = ((self.y + 15.0) as u32) / TILE_SIZE;

        for ty in y1..=y2.min(15) {
            for tx in x1..=x2.min(15) {
                if tilemap.get(tx as u32, ty as u32) != 0 {
                    return true;
                }
            }
        }
        false
    }

    fn collides_spike(&self, tilemap: &Tilemap) -> bool {
        // Spike check: collision zone y=416-440 (in 128-scaled down: y>104)
        // Simplified: check if near bottom
        self.y > 110.0
    }

    fn rand_f32() -> f32 {
        unsafe { (libc::rand() as f32).abs() }
    }

    fn render(&self, color: u32) {
        let x = self.x as u32;
        let y = self.y as u32;
        for dy in 0..16 {
            for dx in 0..16 {
                if x + dx < W && y + dy < H {
                    set_pixel(x + dx, y + dy, color);
                }
            }
        }

        // Eyes
        let eye_color = 0x000000;
        let eye_offset = if self.last_direction > 0.0 { 8 } else { 4 };
        set_pixel(x + eye_offset, y + 4, eye_color);
        set_pixel(x + eye_offset + 2, y + 4, eye_color);
    }
}

struct Orb {
    x: f32,
    y: f32,
    collected: bool,
}

impl Orb {
    fn new(x: f32, y: f32) -> Self {
        Orb {
            x,
            y,
            collected: false,
        }
    }

    fn check_collect(&mut self, player: &Player) {
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        if dx * dx + dy * dy < 256.0 {
            self.collected = true;
        }
    }

    fn render(&self) {
        if !self.collected {
            draw_rect(self.x as u32, self.y as u32, 8, 8, COLOR_ORB);
        }
    }
}

struct Boss {
    x: f32,
    y: f32,
    vy: f32,
    phase: i32,
    attack_timer: i32,
}

impl Boss {
    fn new() -> Self {
        Boss {
            x: 56.0,
            y: 20.0,
            vy: 0.0,
            phase: 0,
            attack_timer: 0,
        }
    }

    fn update(&mut self, _player: &Player, _tilemap: &Tilemap) {
        // Simple boss movement pattern
        self.attack_timer += 1;
        self.vy = (self.vy + 0.5).min(5.0);
        self.y += self.vy;

        if self.y > 110.0 {
            self.vy = -10.0;
        }
    }

    fn render(&self) {
        draw_rect(self.x as u32, self.y as u32, 16, 16, COLOR_BOSS);
    }
}

struct Tilemap {
    tiles: Vec<u32>,
}

impl Tilemap {
    fn new() -> Self {
        Tilemap {
            tiles: vec![0; (GRID_W * GRID_H) as usize],
        }
    }

    fn clear(&mut self) {
        for tile in &mut self.tiles {
            *tile = 0;
        }
    }

    fn set(&mut self, x: u32, y: u32, value: u32) {
        if x < GRID_W && y < GRID_H {
            self.tiles[(y * GRID_W + x) as usize] = value;
        }
    }

    fn get(&self, x: u32, y: u32) -> u32 {
        if x < GRID_W && y < GRID_H {
            self.tiles[(y * GRID_W + x) as usize]
        } else {
            1 // Borders are solid
        }
    }

    fn render(&self) {
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if self.get(x, y) != 0 {
                    draw_rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE, COLOR_WALL);
                }
            }
        }
    }
}

struct InputState {
    left: bool,
    right: bool,
    jump: bool,
}

impl InputState {
    fn new() -> Self {
        InputState {
            left: false,
            right: false,
            jump: false,
        }
    }
}

struct Game {
    player: Player,
    tilemap: Tilemap,
    particles: Vec<Particle>,
    orbs: Vec<Orb>,
    boss: Boss,
    state: i32,
    collected_orbs: i32,
    current_room: (i32, i32),
    rooms_cache: HashMap<(i32, i32), Vec<u32>>,
    input: InputState,
    death_timer: i32,
    clear_timer: i32,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            player: Player::new(),
            tilemap: Tilemap::new(),
            particles: Vec::new(),
            orbs: Vec::new(),
            boss: Boss::new(),
            state: STATE_START,
            collected_orbs: 0,
            current_room: (0, 0),
            rooms_cache: HashMap::new(),
            input: InputState::new(),
            death_timer: 0,
            clear_timer: 0,
        };
        game.load_room(0, 0);
        game
    }

    fn generate_room(&mut self, rx: i32, ry: i32) {
        self.tilemap.clear();
        self.orbs.clear();

        // Borders
        for x in 0..GRID_W {
            for y in 0..GRID_H {
                let is_edge = x == 0 || x == GRID_W - 1 || y == 0 || y == GRID_H - 1;
                let is_exit = (7..=8).contains(&x) && (y == 0 || y == GRID_H - 1)
                    || (7..=8).contains(&y) && (x == 0 || x == GRID_W - 1);
                if is_edge && !is_exit {
                    self.tilemap.set(x, y, 1);
                }
            }
        }

        // Clear exits
        for i in 7..=8 {
            self.tilemap.set(i, 0, 0);
            self.tilemap.set(i, 15, 0);
            self.tilemap.set(0, i, 0);
            self.tilemap.set(15, i, 0);
        }

        // Clear center
        for i in 6..=9 {
            for j in 6..=9 {
                self.tilemap.set(i, j, 0);
            }
        }

        // Add Orb (except start room)
        if rx != 0 || ry != 0 {
            self.orbs.push(Orb::new(56.0, 56.0));
        }
    }

    fn load_room(&mut self, rx: i32, ry: i32) {
        self.current_room = (rx, ry);
        self.generate_room(rx, ry);
        self.player.reset();
    }

    fn update(&mut self) {
        match self.state {
            STATE_START => {
                if self.input.jump {
                    self.state = STATE_PLAY;
                }
            }
            STATE_PLAY => {
                self.player.update(&self.tilemap, &mut self.particles, &self.input);

                // Check Orb collection
                for orb in &mut self.orbs {
                    orb.check_collect(&self.player);
                    if orb.collected {
                        self.collected_orbs += 1;
                    }
                }
                self.orbs.retain(|o| !o.collected);

                // Check room transition
                if self.player.y < 0.0 {
                    self.load_room(self.current_room.0, self.current_room.1 - 1);
                } else if self.player.y > H as f32 {
                    self.load_room(self.current_room.0, self.current_room.1 + 1);
                } else if self.player.x < 0.0 {
                    self.load_room(self.current_room.0 - 1, self.current_room.1);
                } else if self.player.x > W as f32 {
                    self.load_room(self.current_room.0 + 1, self.current_room.1);
                }

                // Check boss spawn
                if self.collected_orbs >= 3 && self.state == STATE_PLAY {
                    self.state = STATE_BOSS;
                    self.boss = Boss::new();
                }
            }
            STATE_BOSS => {
                self.player.update(&self.tilemap, &mut self.particles, &self.input);
                self.boss.update(&self.player, &self.tilemap);

                // Boss collision (simplified)
                let dx = self.boss.x - self.player.x;
                let dy = self.boss.y - self.player.y;
                if dx * dx + dy * dy < 256.0 {
                    self.state = STATE_GAMEOVER_SEQ;
                    self.death_timer = 60;
                }
            }
            STATE_GAMEOVER_SEQ => {
                self.death_timer -= 1;
                if self.death_timer <= 0 {
                    self.state = STATE_GAMEOVER;
                }
            }
            STATE_GAMEOVER | STATE_GAMECLEAR => {
                if self.input.jump {
                    self.reset_game();
                }
            }
            _ => {}
        }

        // Update particles
        for p in &mut self.particles {
            p.update();
        }
        self.particles.retain(|p| p.is_alive());
    }

    fn reset_game(&mut self) {
        self.state = STATE_PLAY;
        self.collected_orbs = 0;
        self.load_room(0, 0);
    }

    fn render(&self) {
        // Background
        FRAMEBUFFER.with(|fb| {
            let mut buf = fb.borrow_mut();
            for pixel in buf.iter_mut() {
                *pixel = COLOR_BG;
            }
        });

        // Tilemap
        self.tilemap.render();

        // Orbs
        for orb in &self.orbs {
            orb.render();
        }

        // Player
        let player_color = if self.player.on_ground {
            COLOR_PLAYER_READY
        } else {
            COLOR_PLAYER_SPENT
        };
        self.player.render(player_color);

        // Boss (in BOSS state)
        if self.state == STATE_BOSS {
            self.boss.render();
        }

        // Particles
        for p in &self.particles {
            set_pixel(p.x as u32, p.y as u32, p.color);
        }

        // UI Text (simple)
        if self.state == STATE_START {
            // Draw "START" indicator
        }
    }
}

static mut GAME: Option<Game> = None;

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        GAME = Some(Game::new());
    }
}

#[no_mangle]
pub extern "C" fn update(left: i32, right: i32, jump: i32) {
    unsafe {
        if let Some(ref mut game) = GAME {
            game.input.left = left != 0;
            game.input.right = right != 0;
            game.input.jump = jump != 0;
            game.update();
        }
    }
}

#[no_mangle]
pub extern "C" fn render() {
    unsafe {
        if let Some(ref game) = GAME {
            game.render();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_framebuffer() -> *const u32 {
    FRAMEBUFFER.with(|fb| {
        let buf = fb.borrow();
        buf.as_ptr()
    })
}

fn set_pixel(x: u32, y: u32, color: u32) {
    if x < W && y < H {
        FRAMEBUFFER.with(|fb| {
            let mut buf = fb.borrow_mut();
            buf[(y * W + x) as usize] = color;
        });
    }
}

fn draw_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    for dy in 0..h {
        for dx in 0..w {
            if x + dx < W && y + dy < H {
                set_pixel(x + dx, y + dy, color);
            }
        }
    }
}
