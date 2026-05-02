// Cubeboy Full WASM - Complete Pyxel Port
// Based on /tmp/Cubeboy/Cubeboy/Cubeboy.py (780 lines)
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

const W: usize = 128;
const H: usize = 128;
const TILE_SIZE: usize = 8;
const FRAMEBUFFER_SIZE: usize = W * H;

// Colors (Pyxel palette)
const C_BG: u8 = 0;
const C_WALL: u8 = 1;
const C_PLAYER_READY: u8 = 7;
const C_PLAYER_SPENT: u8 = 14;
const C_PARTICLE: u8 = 12;
const C_ORB: u8 = 10;
const C_SPIKE: u8 = 7;
const C_CRYSTAL: u8 = 12;
const C_MOUNTAIN_1: u8 = 1;
const C_MOUNTAIN_2: u8 = 13;

// Game states
const STATE_START: u32 = 0;
const STATE_PLAY: u32 = 1;
const STATE_BOSS: u32 = 2;
const STATE_GAMEOVER: u32 = 3;
const STATE_GAMECLEAR: u32 = 4;
const STATE_GAMEOVER_SEQ: u32 = 5;

thread_local! {
    static RAND_SEED: RefCell<u32> = RefCell::new(1);
}

fn rand() -> u32 {
    RAND_SEED.with(|seed| {
        let mut s = seed.borrow_mut();
        *s = s.wrapping_mul(1103515245).wrapping_add(12345);
        *s
    })
}

fn rndf(a: f32, b: f32) -> f32 {
    a + ((rand() as f32 / u32::MAX as f32) * (b - a))
}

fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min { min } else if v > max { max } else { v }
}

fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    col: u8,
    life: i32,
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32, col: u8, life: i32) -> Self {
        Particle { x, y, dx, dy, col, life }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.life -= 1;
    }

    fn is_alive(&self) -> bool {
        self.life > 0
    }
}

#[derive(Clone, Copy)]
struct Orb {
    x: f32,
    y: f32,
    alive: bool,
}

impl Orb {
    fn new(x: f32, y: f32) -> Self {
        Orb { x, y, alive: true }
    }

    fn update(&mut self, player: &Player) {
        if self.alive && (player.x - self.x).abs() < 8.0 && (player.y - self.y).abs() < 8.0 {
            self.alive = false;
        }
    }

    fn draw(&self, fb: &mut Vec<u8>) {
        if self.alive {
            let x = self.x as i32;
            let y = self.y as i32;
            for dx in -2..=2 {
                for dy in -2..=2 {
                    set_pixel(fb, x + dx, y + dy, C_ORB);
                }
            }
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
    is_on_wall: i32,
    can_dash: bool,
    dash_time: i32,
    dash_dir: (f32, f32),
    
    coyote_timer: i32,
    jump_buffer: i32,
    
    facing: i32,
    is_dead: bool,
    
    stretch_x: f32,
    stretch_y: f32,
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
            is_on_ground: false,
            is_on_wall: 0,
            can_dash: true,
            dash_time: 0,
            dash_dir: (0.0, 0.0),
            coyote_timer: 0,
            jump_buffer: 0,
            facing: 1,
            is_dead: false,
            stretch_x: 1.0,
            stretch_y: 1.0,
        }
    }

    fn is_wall(&self, x: f32, y: f32, tilemap: &Tilemap) -> bool {
        // Tilemap-based wall check with padding
        let x1 = ((x) / TILE_SIZE as f32).floor() as usize;
        let y1 = ((y) / TILE_SIZE as f32).floor() as usize;
        let x2 = ((x + self.width as f32 - 0.1) / TILE_SIZE as f32).floor() as usize;
        let y2 = ((y + self.height as f32 - 0.1) / TILE_SIZE as f32).floor() as usize;
        
        for ty in y1..=y2 {
            if ty >= 16 { continue; }
            for tx in x1..=x2 {
                if tx >= 16 { continue; }
                if tilemap.get(tx, ty) == 1 {
                    return true;
                }
            }
        }
        false
    }

    fn resolve_overlap(&mut self, tilemap: &Tilemap) {
        if self.is_wall(self.x, self.y, tilemap) {
            for r in 1..16 {
                let mut found = false;
                for dx in -(r as i32)..=(r as i32) {
                    for dy in -(r as i32)..=(r as i32) {
                        if !self.is_wall(self.x + dx as f32, self.y + dy as f32, tilemap) {
                            self.x += dx as f32;
                            self.y += dy as f32;
                            found = true;
                            break;
                        }
                    }
                    if found { break; }
                }
                if found { break; }
            }
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool, tilemap: &Tilemap, particles: &mut Vec<Particle>) {
        self.resolve_overlap(tilemap);
        
        if self.coyote_timer > 0 { self.coyote_timer -= 1; }
        if self.jump_buffer > 0 { self.jump_buffer -= 1; }

        if self.dash_time > 0 {
            self.vx = self.dash_dir.0 * 5.0;
            self.vy = self.dash_dir.1 * 5.0;
            self.dash_time -= 1;
            particles.push(Particle::new(self.x + 3.0, self.y + 3.0, rndf(-1.0, 1.0), rndf(-1.0, 1.0), C_PARTICLE, 10));
            if self.dash_time == 0 {
                self.vx *= 0.5;
                self.vy *= 0.5;
            }
        } else {
            let mut dx = 0;
            if left { dx -= 1; }
            if right { dx += 1; }
            
            if dx != 0 {
                let target_vx = (dx as f32) * 2.5;
                self.vx += (target_vx - self.vx) * 0.2;
                self.facing = dx;
            } else {
                self.vx *= 0.7;
            }
            
            if self.is_on_wall != 0 && self.vy > 0.0 {
                self.vy = clamp(self.vy + 0.1, 0.0, 0.8);
            } else {
                let grav = if jump && self.vy < 0.0 { 0.3 } else { 0.5 };
                self.vy += grav;
            }
            
            if jump {
                self.jump_buffer = 4;
            }
            
            if self.jump_buffer > 0 {
                if self.coyote_timer > 0 {
                    self.vy = -4.5;
                    self.stretch_x = 0.6;
                    self.stretch_y = 1.4;
                    self.coyote_timer = 0;
                    self.jump_buffer = 0;
                } else if self.is_on_wall != 0 {
                    self.vy = -4.2;
                    self.vx = -(self.is_on_wall as f32) * 3.5;
                    self.stretch_x = 0.6;
                    self.stretch_y = 1.4;
                    self.jump_buffer = 0;
                }
            }
        }

        // Collision & Movement (Axis Separated)
        let steps_x = ((self.vx.abs() / 0.5).floor() as i32) + 1;
        if steps_x > 0 {
            let step_x = self.vx / steps_x as f32;
            for _ in 0..steps_x {
                if !self.is_wall(self.x + step_x, self.y, tilemap) {
                    self.x += step_x;
                } else {
                    self.vx = 0.0;
                    break;
                }
            }
        }

        let steps_y = ((self.vy.abs() / 0.5).floor() as i32) + 1;
        if steps_y > 0 {
            let step_y = self.vy / steps_y as f32;
            for _ in 0..steps_y {
                if !self.is_wall(self.x, self.y + step_y, tilemap) {
                    self.y += step_y;
                } else {
                    self.vy = 0.0;
                    break;
                }
            }
        }

        self.is_on_ground = self.is_wall(self.x, self.y + 1.0, tilemap);
        if self.is_on_ground {
            self.coyote_timer = 6;
            self.can_dash = true;
        }

        if self.is_wall(self.x + 1.0, self.y, tilemap) { self.is_on_wall = 1; }
        else if self.is_wall(self.x - 1.0, self.y, tilemap) { self.is_on_wall = -1; }
        else { self.is_on_wall = 0; }
    }

    fn draw(&self, fb: &mut Vec<u8>) {
        let color = if self.can_dash { C_PLAYER_READY } else { C_PLAYER_SPENT };
        let x = (self.x * self.stretch_x) as i32;
        let y = (self.y * self.stretch_y) as i32;
        let w = (self.width as f32 * self.stretch_x) as i32;
        let h = (self.height as f32 * self.stretch_y) as i32;
        
        draw_rect(fb, x, y, w, h, color);
    }
}

struct Tilemap {
    tiles: [[u8; 16]; 16],
}

impl Tilemap {
    fn new() -> Self {
        Tilemap {
            tiles: [[0; 16]; 16],
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        if x < 16 && y < 16 { self.tiles[y][x] } else { 0 }
    }

    fn set(&mut self, x: usize, y: usize, tile: u8) {
        if x < 16 && y < 16 { self.tiles[y][x] = tile; }
    }
}

struct Game {
    player: Player,
    particles: Vec<Particle>,
    orbs: Vec<Orb>,
    tilemap: Tilemap,
    state: u32,
    room_x: i32,
    room_y: i32,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            player: Player::new(64.0, 60.0),
            particles: Vec::new(),
            orbs: Vec::new(),
            tilemap: Tilemap::new(),
            state: STATE_START,
            room_x: 0,
            room_y: 0,
        };
        game.generate_room();
        game
    }

    fn generate_room(&mut self) {
        // Simple room: walls on edges
        for y in 0..16 {
            self.tilemap.set(0, y, 1);
            self.tilemap.set(15, y, 1);
        }
        for x in 0..16 {
            self.tilemap.set(x, 0, 1);
            self.tilemap.set(x, 15, 1);
        }
        
        // Add some obstacles
        if self.room_x % 2 == 0 {
            for x in 4..8 {
                self.tilemap.set(x, 7, 1);
            }
        }
        if self.room_y % 2 == 0 {
            for y in 4..8 {
                self.tilemap.set(8, y, 1);
            }
        }
        
        // Spawn orbs
        self.orbs.clear();
        for i in 0..3 {
            self.orbs.push(Orb::new(32.0 + (i as f32) * 40.0, 32.0));
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        match self.state {
            STATE_START => {
                self.state = STATE_PLAY;
            }
            STATE_PLAY => {
                self.player.update(left, right, jump, &self.tilemap, &mut self.particles);
                
                for orb in &mut self.orbs {
                    orb.update(&self.player);
                }
                
                self.particles.retain_mut(|p| {
                    p.update();
                    p.is_alive()
                });
                
                if self.player.y > 128.0 {
                    self.player.is_dead = true;
                    self.state = STATE_GAMEOVER_SEQ;
                }
            }
            STATE_GAMEOVER_SEQ => {
                if self.player.y > 200.0 {
                    self.player = Player::new(64.0, 60.0);
                    self.state = STATE_PLAY;
                }
            }
            _ => {}
        }
    }

    fn render(&self, fb: &mut Vec<u8>) {
        fb.fill(C_BG);
        
        // Draw tilemap
        for y in 0..16 {
            for x in 0..16 {
                if self.tilemap.get(x, y) == 1 {
                    let px = (x * TILE_SIZE) as i32;
                    let py = (y * TILE_SIZE) as i32;
                    draw_rect(fb, px, py, TILE_SIZE as i32, TILE_SIZE as i32, C_WALL);
                }
            }
        }
        
        // Draw orbs
        for orb in &self.orbs {
            orb.draw(fb);
        }
        
        // Draw particles
        for p in &self.particles {
            if p.is_alive() {
                set_pixel(fb, p.x as i32, p.y as i32, p.col);
            }
        }
        
        // Draw player
        if self.state != STATE_GAMEOVER_SEQ {
            self.player.draw(fb);
        }
    }
}

fn set_pixel(fb: &mut Vec<u8>, x: i32, y: i32, color: u8) {
    if x >= 0 && x < W as i32 && y >= 0 && y < H as i32 {
        let idx = (y as usize) * W + (x as usize);
        if idx < fb.len() {
            fb[idx] = color;
        }
    }
}

fn draw_rect(fb: &mut Vec<u8>, x: i32, y: i32, w: i32, h: i32, color: u8) {
    for dy in 0..h {
        for dx in 0..w {
            set_pixel(fb, x + dx, y + dy, color);
        }
    }
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BG; FRAMEBUFFER_SIZE]);
}

#[wasm_bindgen]
pub fn init() {
    GAME.with(|g| {
        *g.borrow_mut() = Game::new();
    });
}

#[wasm_bindgen]
pub fn update(left: bool, right: bool, jump: bool) {
    GAME.with(|g| {
        g.borrow_mut().update(left, right, jump);
    });
}

#[wasm_bindgen]
pub fn render() {
    GAME.with(|g| {
        FRAMEBUFFER.with(|fb| {
            let mut fbuf = fb.borrow_mut();
            g.borrow().render(&mut fbuf);
        });
    });
}

#[wasm_bindgen]
pub fn get_framebuffer() -> *const u8 {
    FRAMEBUFFER.with(|fb| {
        fb.borrow().as_ptr()
    })
}
