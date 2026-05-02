// Cubeboy WASM - Full Pyxel Port
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
const C_PARTICLE: u8 = 8;  // Changed from 12 (cyan) to 8 (red) for visibility
const C_ORB: u8 = 10;
const C_SPIKE: u8 = 7;
const C_GREEN: u8 = 3;
const C_WHITE: u8 = 7;

// Game states
const STATE_START: u32 = 0;
const STATE_PLAY: u32 = 1;
const STATE_BOSS: u32 = 2;
const STATE_GAMEOVER: u32 = 3;
const STATE_GAMECLEAR: u32 = 4;
const STATE_GAMEOVER_SEQ: u32 = 5;

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
    dash_time: u32,
    coyote_timer: u32,
    jump_buffer: u32,
    is_dead: bool,
    last_direction: f32,  // Track last move direction for eyes
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 24,   // 6 → 24 (4x)
            height: 24,  // 6 → 24 (4x)
            is_on_ground: false,
            is_on_wall: 0,
            can_dash: true,
            dash_time: 0,
            coyote_timer: 0,
            jump_buffer: 0,
            is_dead: false,
            last_direction: 1.0,  // Default: looking right
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        const GRAVITY: f32 = 1.6;         // 0.4 * 4
        const MOVE_SPEED: f32 = 8.0;      // 2.0 * 4
        const MAX_FALL: f32 = 20.0;       // 5.0 * 4
        const JUMP_POWER: f32 = -40.0;    // -10.0 * 4
        const COYOTE_FRAMES: u32 = 6;
        const JUMP_BUFFER_FRAMES: u32 = 4;

        // Input
        let mut dx = 0.0;
        if left {
            dx -= MOVE_SPEED;
        }
        if right {
            dx += MOVE_SPEED;
        }

        // Jump buffer
        if jump {
            self.jump_buffer = JUMP_BUFFER_FRAMES;
        } else if self.jump_buffer > 0 {
            self.jump_buffer -= 1;
        }

        // Apply input
        self.vx = dx;
        
        // Track direction for eyes
        if self.vx > 0.0 {
            self.last_direction = 1.0;  // Right
        } else if self.vx < 0.0 {
            self.last_direction = -1.0;  // Left
        }
        // If vx == 0.0, keep last_direction

        // Coyote time (decrement first, before applying gravity)
        if self.is_on_ground {
            self.coyote_timer = COYOTE_FRAMES;
        } else if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }

        // Jump logic
        if self.jump_buffer > 0 && (self.is_on_ground || self.coyote_timer > 0) {
            self.vy = JUMP_POWER;
            self.jump_buffer = 0;
            self.coyote_timer = 0;
            self.is_on_ground = false;
        }

        // Apply gravity only if not on ground
        if !self.is_on_ground {
            self.vy += GRAVITY;
            if self.vy > MAX_FALL {
                self.vy = MAX_FALL;
            }
        }

        // Move
        self.x += self.vx;
        self.y += self.vy;

        // Bounds
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.width as f32 > W as f32 {
            self.x = W as f32 - self.width as f32;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y > H as f32 {
            self.is_dead = true;
        }

        // Ground collision (floor at y=440)
        const FLOOR_Y: f32 = 440.0;
        
        if self.y + self.height as f32 >= FLOOR_Y {
            self.y = FLOOR_Y - self.height as f32;
            self.vy = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }

        // Spike collision (on LEFT side: x=40-160, y=416-440) - 24px tall like player
        if self.x < 160.0 && self.x + self.width as f32 > 40.0 &&
           self.y < 440.0 && self.y + self.height as f32 > 416.0 {
            self.is_dead = true;
        }
    }

    fn draw(&self, fb: &mut Vec<u8>, color: u8) {
        let x = self.x as i32;
        let y = self.y as i32;
        for dy in 0..(self.height as i32) {
            for dx in 0..(self.width as i32) {
                set_pixel(fb, x + dx, y + dy, color);
            }
        }
        
        // Draw eyes horizontally based on last direction
        let eye_color = 0;  // black
        let (left_eye_x, right_eye_x) = if self.last_direction > 0.0 {
            (14, 20)  // Right: eyes on right side
        } else {
            (2, 8)    // Left: eyes on left side
        };
        
        // Left eye (2x2 pixels)
        for dy in 0..2 {
            for dx in 0..2 {
                set_pixel(fb, x + left_eye_x + dx, y + 10 + dy, eye_color);
            }
        }
        
        // Right eye (2x2 pixels)
        for dy in 0..2 {
            for dx in 0..2 {
                set_pixel(fb, x + right_eye_x + dx, y + 10 + dy, eye_color);
            }
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

fn draw_triangle(fb: &mut Vec<u8>, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: u8) {
    // Simple triangle fill using horizontal lines
    let min_y = y1.min(y2).min(y3);
    let max_y = y1.max(y2).max(y3);
    
    for y in min_y..=max_y {
        let fy = y as f32;
        
        // Find left and right intersections
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        
        // Edge 1-2
        if (y1 <= y && y <= y2) || (y2 <= y && y <= y1) {
            if y1 != y2 {
                let x = x1 as f32 + (x2 as f32 - x1 as f32) * (fy - y1 as f32) / (y2 as f32 - y1 as f32);
                left = left.min(x as i32);
                right = right.max(x as i32);
            }
        }
        // Edge 2-3
        if (y2 <= y && y <= y3) || (y3 <= y && y <= y2) {
            if y2 != y3 {
                let x = x2 as f32 + (x3 as f32 - x2 as f32) * (fy - y2 as f32) / (y3 as f32 - y2 as f32);
                left = left.min(x as i32);
                right = right.max(x as i32);
            }
        }
        // Edge 3-1
        if (y3 <= y && y <= y1) || (y1 <= y && y <= y3) {
            if y3 != y1 {
                let x = x3 as f32 + (x1 as f32 - x3 as f32) * (fy - y3 as f32) / (y1 as f32 - y3 as f32);
                left = left.min(x as i32);
                right = right.max(x as i32);
            }
        }
        
        if left <= right {
            for x in left..=right {
                set_pixel(fb, x, y, color);
            }
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

struct Game {
    player: Player,
    particles: Vec<Particle>,
    state: u32,
    shake: i32,
    death_counter: i32,
    debug_death_x: i32,
    debug_death_y: i32,
}

impl Game {
    fn new() -> Self {
        Game {
            player: Player::new(256.0, 200.0),  // 64 → 256, 50 → 200 (4x)
            particles: Vec::new(),
            state: STATE_START,
            shake: 0,
            death_counter: 0,
            debug_death_x: -1,
            debug_death_y: -1,
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        match self.state {
            STATE_START => {
                self.state = STATE_PLAY;
            }
            STATE_PLAY => {
                self.player.update(left, right, jump);
                
                // Update particles
                if !self.player.is_dead {
                    self.particles.retain_mut(|p| {
                        p.update();
                        p.is_alive()
                    });
                }

                if self.player.is_dead {
                    self.shake = 10;
                    self.debug_death_x = self.player.x as i32;
                    self.debug_death_y = self.player.y as i32;
                    // Generate particles at death location with randomized velocity
                    for _ in 0..20 {
                        let r1 = rand() as f32;
                        let r2 = rand() as f32;
                        let angle = (r1 % 628.0) / 100.0;  // 0 to 2π
                        let speed = 0.1 + (r2 % 100.0) / 200.0;  // 0.1 to 0.6 pixels/frame
                        let vx = angle.cos() * speed;
                        let vy = angle.sin() * speed;
                        
                        self.particles.push(Particle::new(
                            self.player.x + 12.0,  // Center of player
                            self.player.y + 12.0,
                            vx,
                            vy,
                            C_PARTICLE,
                            300,
                        ));
                    }
                    // Transition to death animation state
                    self.state = STATE_GAMEOVER_SEQ;
                    self.death_counter = 300;  // Match particle lifetime
                } else {
                    // Only update particles if NOT dead (avoid deletion on spawn)
                    self.particles.retain_mut(|p| {
                        p.update();
                        p.is_alive()
                    });
                }
            }
            STATE_GAMEOVER_SEQ => {
                // Show particles and keep player dead
                self.death_counter -= 1;
                
                // Update particles during death animation
                self.particles.retain_mut(|p| {
                    p.update();
                    p.is_alive()
                });
                
                // After animation, reset and go back to PLAY state
                if self.death_counter <= 0 {
                    self.player = Player::new(256.0, 200.0);  // 4x scale
                    self.particles.clear();
                    self.state = STATE_PLAY;
                }
            }
            _ => {}
        }

        if self.shake > 0 {
            self.shake -= 1;
        }
    }

    fn render(&self, fb: &mut Vec<u8>) {
        // Clear with background color
        fb.fill(C_BG);

        // Draw solid ground (4x scale)
        draw_rect(fb, 0, 440, W as i32, 72, C_GREEN);

        // Draw spikes on LEFT side (12px wide, paired)
        for x in (40..160).step_by(24) {
            // Left spike of pair (12px wide)
            draw_triangle(fb, x, 440, x + 6, 416, x + 12, 440, C_SPIKE);
            // Right spike of pair (12px wide)
            draw_triangle(fb, x + 12, 440, x + 18, 416, x + 24, 440, C_SPIKE);
        }

        // Draw player (only if not dead)
        if self.state != STATE_GAMEOVER_SEQ {
            let player_color = if self.player.can_dash {
                C_PLAYER_READY
            } else {
                C_PLAYER_SPENT
            };
            self.player.draw(fb, player_color);
        }

        // Draw particles (larger for visibility)
        // TEST: Draw particles at 10x10 size for better visibility
        for (i, p) in self.particles.iter().enumerate() {
            if p.is_alive() {
                // Draw 10x10 particles
                for dy in 0..10 {
                    for dx in 0..10 {
                        set_pixel(fb, (p.x as i32) + dx, (p.y as i32) + dy, C_PARTICLE);
                    }
                }
            }
        }

        // Draw simple UI at top
        draw_rect(fb, 2, 2, 30, 8, C_WHITE); // Title area
    }
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BG; FRAMEBUFFER_SIZE]);
}

static mut RAND_SEED: u32 = 1;

fn rand() -> u32 {
    unsafe {
        RAND_SEED = RAND_SEED.wrapping_mul(1103515245).wrapping_add(12345);
        RAND_SEED
    }
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
    GAME.with(|game_cell| {
        let game = game_cell.borrow();
        let mut fb = vec![C_BG; FRAMEBUFFER_SIZE];
        game.render(&mut fb);

        FRAMEBUFFER.with(|fbuf| {
            *fbuf.borrow_mut() = fb;
        });
    });
}

#[wasm_bindgen]
pub fn get_framebuffer() -> Vec<u8> {
    FRAMEBUFFER.with(|fbuf| {
        fbuf.borrow().clone()
    })
}

#[wasm_bindgen]
pub fn reset() {
    init();
}
