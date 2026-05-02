// Mario Kart - Simple Racing Game
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

const W: usize = 512;
const H: usize = 512;
const FRAMEBUFFER_SIZE: usize = W * H;

// Colors
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

struct Kart {
    x: f32,
    y: f32,
    speed: f32,
    angle: f32,
}

struct Obstacle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Kart {
    fn new(x: f32, y: f32) -> Self {
        Kart {
            x,
            y,
            speed: 0.0,
            angle: 0.0,
        }
    }

    fn update(&mut self, left: bool, right: bool, accelerate: bool) {
        const MAX_SPEED: f32 = 2.5;
        const ACCEL: f32 = 0.1;
        const FRICTION: f32 = 0.94;
        const TRACK_MIN: f32 = 60.0;
        const TRACK_MAX: f32 = 452.0;
        const KART_RADIUS: f32 = 3.0;

        // Turn
        if left {
            self.angle -= 0.08;
        }
        if right {
            self.angle += 0.08;
        }

        // Accelerate
        if accelerate && self.speed < MAX_SPEED {
            self.speed += ACCEL;
        } else {
            self.speed *= FRICTION;
        }

        // Move forward in angle direction
        self.x += self.speed * self.angle.cos();
        self.y += self.speed * self.angle.sin();

        // Collision with track boundaries
        if self.x < TRACK_MIN + KART_RADIUS {
            self.x = TRACK_MIN + KART_RADIUS;
            self.speed *= -0.3;
        }
        if self.x > TRACK_MAX - KART_RADIUS {
            self.x = TRACK_MAX - KART_RADIUS;
            self.speed *= -0.3;
        }
        if self.y < TRACK_MIN + KART_RADIUS {
            self.y = TRACK_MIN + KART_RADIUS;
            self.speed *= -0.3;
        }
        if self.y > TRACK_MAX - KART_RADIUS {
            self.y = TRACK_MAX - KART_RADIUS;
            self.speed *= -0.3;
        }

        // Collision with obstacles - push out completely
        let obstacles = get_obstacles();
        for obs in obstacles.iter() {
            if self.collides_with_obstacle(obs, KART_RADIUS) {
                // Push kart outside obstacle
                let obs_cx = obs.x + obs.w / 2.0;
                let obs_cy = obs.y + obs.h / 2.0;
                let dx = self.x - obs_cx;
                let dy = self.y - obs_cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 0.0 {
                    let obs_radius = (obs.w.max(obs.h)) / 2.0;
                    let push_dist = obs_radius + KART_RADIUS + 1.0;
                    self.x = obs_cx + (dx / dist) * push_dist;
                    self.y = obs_cy + (dy / dist) * push_dist;
                    // Bounce back with force
                    self.speed = -2.0 * (dx / dist).abs().max((dy / dist).abs());
                }
            }
        }
    }

    fn collides_with_obstacle(&self, obs: &Obstacle, kart_radius: f32) -> bool {
        let closest_x = self.x.max(obs.x).min(obs.x + obs.w);
        let closest_y = self.y.max(obs.y).min(obs.y + obs.h);
        let dx = self.x - closest_x;
        let dy = self.y - closest_y;
        (dx * dx + dy * dy) < (kart_radius * kart_radius)
    }

    fn draw(&self, fb: &mut Vec<u8>) {
        let x = self.x as i32;
        let y = self.y as i32;

        // Draw kart (red square)
        for dy in -2..3 {
            for dx in -2..3 {
                set_pixel(fb, x + dx, y + dy, C_RED);
            }
        }

        // Draw direction indicator (yellow)
        let dir_x = (self.x + self.angle.cos() * 5.0) as i32;
        let dir_y = (self.y + self.angle.sin() * 5.0) as i32;
        set_pixel(fb, dir_x, dir_y, C_YELLOW);
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

fn draw_rect_filled(fb: &mut Vec<u8>, x: i32, y: i32, w: i32, h: i32, color: u8) {
    for py in y..y + h {
        for px in x..x + w {
            set_pixel(fb, px, py, color);
        }
    }
}

fn draw_rect_border(fb: &mut Vec<u8>, x: i32, y: i32, w: i32, h: i32, color: u8, thick: i32) {
    // Top border
    draw_rect_filled(fb, x, y, w, thick, color);
    // Bottom border
    draw_rect_filled(fb, x, y + h - thick, w, thick, color);
    // Left border
    draw_rect_filled(fb, x, y, thick, h, color);
    // Right border
    draw_rect_filled(fb, x + w - thick, y, thick, h, color);
}

fn get_obstacles() -> Vec<Obstacle> {
    vec![
        Obstacle { x: 100.0, y: 100.0, w: 50.0, h: 50.0 },
        Obstacle { x: 362.0, y: 100.0, w: 50.0, h: 50.0 },
        Obstacle { x: 100.0, y: 362.0, w: 50.0, h: 50.0 },
        Obstacle { x: 362.0, y: 362.0, w: 50.0, h: 50.0 },
    ]
}

thread_local! {
    static KART: RefCell<Kart> = RefCell::new(Kart::new(256.0, 256.0));
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BLACK; FRAMEBUFFER_SIZE]);
}

#[wasm_bindgen]
pub fn init() {
    KART.with(|k| {
        *k.borrow_mut() = Kart::new(256.0, 256.0);
    });
}

#[wasm_bindgen]
pub fn update(left: bool, right: bool, jump: bool) {
    KART.with(|k| {
        k.borrow_mut().update(left, right, jump);
    });
}

#[wasm_bindgen]
pub fn render() {
    KART.with(|kart_cell| {
        let kart = kart_cell.borrow();

        let mut fb = vec![C_NAVY; FRAMEBUFFER_SIZE];

        // Draw outer track border (gray)
        draw_rect_border(&mut fb, 40, 40, 432, 432, C_GRAY, 10);

        // Draw inner green track
        draw_rect_filled(&mut fb, 60, 60, 392, 392, C_GREEN);

        // Draw obstacles (brown boxes in 4 corners)
        draw_rect_filled(&mut fb, 100, 100, 50, 50, C_BROWN);
        draw_rect_filled(&mut fb, 362, 100, 50, 50, C_BROWN);
        draw_rect_filled(&mut fb, 100, 362, 50, 50, C_BROWN);
        draw_rect_filled(&mut fb, 362, 362, 50, 50, C_BROWN);

        // Draw kart
        kart.draw(&mut fb);

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
