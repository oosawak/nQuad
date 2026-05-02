// Mario 64 - Simplified WASM Implementation
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

const W: usize = 512;
const H: usize = 512;
const FRAMEBUFFER_SIZE: usize = W * H;

// Colors (Pyxel 16-color palette)
const C_BLACK: u8 = 0;
const C_NAVY: u8 = 1;
const C_PURPLE: u8 = 2;
const C_GREEN: u8 = 3;
const C_BROWN: u8 = 4;
const C_DKGRAY: u8 = 5;
const C_LTGRAY: u8 = 6;
const C_WHITE: u8 = 7;
const C_RED: u8 = 8;
const C_ORANGE: u8 = 9;
const C_YELLOW: u8 = 10;
const C_LIME: u8 = 11;
const C_CYAN: u8 = 12;
const C_GRAY: u8 = 13;
const C_PINK: u8 = 14;
const C_PEACH: u8 = 15;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn project_iso(&self, angle: f32) -> (i32, i32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let rx = self.x * cos_a - self.y * sin_a;
        let ry = self.x * sin_a + self.y * cos_a;

        let sx = (rx - ry) as i32 + 256;
        let sy = ((rx + ry) * 0.5 - self.z) as i32 + 256;

        (sx, sy)
    }
}

struct Mario {
    pos: Vec3,
    vel: Vec3,
}

impl Mario {
    fn new() -> Self {
        Mario {
            pos: Vec3::new(0.0, 0.0, 0.0),
            vel: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        const SPEED: f32 = 1.5;
        const GRAVITY: f32 = 0.25;
        const FRICTION: f32 = 0.88;

        if left {
            self.vel.x -= SPEED;
        }
        if right {
            self.vel.x += SPEED;
        }

        self.vel.z -= GRAVITY;

        if jump && self.pos.z < 0.5 {
            self.vel.z = 6.0;
        }

        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;

        if self.pos.z < 0.0 {
            self.pos.z = 0.0;
            self.vel.z = 0.0;
            self.vel.x *= FRICTION;
            self.vel.y *= FRICTION;
        }

        let bounds = 150.0;
        if self.pos.x < -bounds || self.pos.x > bounds {
            self.pos.x = self.pos.x.clamp(-bounds, bounds);
            self.vel.x *= -0.3;
        }
        if self.pos.y < -bounds || self.pos.y > bounds {
            self.pos.y = self.pos.y.clamp(-bounds, bounds);
            self.vel.y *= -0.3;
        }
    }

    fn draw(&self, fb: &mut Vec<u8>, angle: f32) {
        let (sx, sy) = self.pos.project_iso(angle);

        let color = if self.pos.z > 5.0 { C_CYAN } else { C_RED };

        for dy in -3..4 {
            for dx in -3..4 {
                set_pixel(fb, sx + dx, sy + dy, color);
            }
        }

        if sx >= 0 && sx < W as i32 && sy >= 0 && sy < H as i32 {
            set_pixel(fb, sx - 2, sy - 1, C_WHITE);
            set_pixel(fb, sx + 2, sy - 1, C_WHITE);
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

// WASM Global State
thread_local! {
    static MARIO: RefCell<Mario> = RefCell::new(Mario::new());
    static CAMERA_ANGLE: RefCell<f32> = RefCell::new(0.0);
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BLACK; FRAMEBUFFER_SIZE]);
}

#[wasm_bindgen]
pub fn init() {
    MARIO.with(|m| {
        *m.borrow_mut() = Mario::new();
    });
}

#[wasm_bindgen]
pub fn update(left: bool, right: bool, jump: bool) {
    MARIO.with(|m| {
        m.borrow_mut().update(left, right, jump);
    });
}

#[wasm_bindgen]
pub fn render() {
    MARIO.with(|mario_cell| {
        CAMERA_ANGLE.with(|angle_cell| {
            let mario = mario_cell.borrow();
            let angle = *angle_cell.borrow();

            let mut fb = vec![C_NAVY; FRAMEBUFFER_SIZE];

            // Ground
            for py in 380..H {
                for px in 0..W {
                    fb[py * W + px] = C_GREEN;
                }
            }

            mario.draw(&mut fb, angle);

            FRAMEBUFFER.with(|fbuf| {
                *fbuf.borrow_mut() = fb;
            });
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
