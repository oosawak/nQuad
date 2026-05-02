// Pyxel互換API層
use std::sync::Mutex;

static FRAME_COUNT: Mutex<i32> = Mutex::new(0);
pub static FRAMEBUFFER: Mutex<[u32; 128 * 128]> = Mutex::new([0; 128 * 128]);
static RANDOM_SEED_STATE: Mutex<u32> = Mutex::new(12345);

pub mod input {
    pub struct InputState {
        pub keys: [bool; 256],
    }

    thread_local! {
        static INPUT: InputState = InputState { keys: [false; 256] };
    }
}

pub struct Pyxel;

impl Pyxel {
    /// Get current frame count
    pub fn frame_count() -> i32 {
        *FRAME_COUNT.lock().unwrap()
    }

    /// Increment frame count
    pub fn tick_frame() {
        if let Ok(mut count) = FRAME_COUNT.lock() {
            *count += 1;
        }
    }

    /// Get framebuffer
    pub fn framebuffer() -> [u32; 128 * 128] {
        *FRAMEBUFFER.lock().unwrap()
    }

    /// Set framebuffer
    pub fn set_framebuffer(fb: [u32; 128 * 128]) {
        if let Ok(mut buffer) = FRAMEBUFFER.lock() {
            *buffer = fb;
        }
    }

    /// Math: clamp value
    pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
        if val < min { min } else if val > max { max } else { val }
    }

    /// Math: square root
    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }

    /// Math: sine (degrees to radians conversion)
    pub fn sin(deg: f32) -> f32 {
        (deg * std::f32::consts::PI / 180.0).sin()
    }

    /// Math: cosine
    pub fn cos(deg: f32) -> f32 {
        (deg * std::f32::consts::PI / 180.0).cos()
    }

    /// Random: float in range [min, max)
    pub fn rndf(min: f32, max: f32) -> f32 {
        if let Ok(mut seed) = RANDOM_SEED_STATE.lock() {
            *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let val = (*seed as f32 / u32::MAX as f32).abs();
            min + (max - min) * val
        } else {
            min
        }
    }

    /// Random: integer in range [min, max]
    pub fn rnd(min: i32, max: i32) -> i32 {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        let range = (max - min + 1) as f32;
        min + (Self::rndf(0.0, range) as i32)
    }
}

/// Drawing primitives
pub fn pset(x: i32, y: i32, col: u32) {
    if x >= 0 && x < 128 && y >= 0 && y < 128 {
        if let Ok(mut fb) = FRAMEBUFFER.lock() {
            fb[(y as usize * 128 + x as usize)] = col;
        }
    }
}

pub fn rect(x: i32, y: i32, w: i32, h: i32, col: u32) {
    for dy in 0..h {
        for dx in 0..w {
            pset(x + dx, y + dy, col);
        }
    }
}

pub fn rectb(x: i32, y: i32, w: i32, h: i32, col: u32) {
    // Top and bottom
    for dx in 0..w {
        pset(x + dx, y, col);
        pset(x + dx, y + h - 1, col);
    }
    // Left and right
    for dy in 0..h {
        pset(x, y + dy, col);
        pset(x + w - 1, y + dy, col);
    }
}

/// Midpoint circle algorithm
pub fn circ(x: i32, y: i32, r: i32, col: u32) {
    let mut ox = r;
    let mut oy = 0;
    let mut d = 3 - 2 * r;

    while ox >= oy {
        // 8 symmetric points
        pset(x + ox, y + oy, col);
        pset(x - ox, y + oy, col);
        pset(x + ox, y - oy, col);
        pset(x - ox, y - oy, col);
        pset(x + oy, y + ox, col);
        pset(x - oy, y + ox, col);
        pset(x + oy, y - ox, col);
        pset(x - oy, y - ox, col);

        if d < 0 {
            d += 4 * oy + 6;
        } else {
            d += 4 * (oy - ox) + 10;
            ox -= 1;
        }
        oy += 1;
    }
}

pub fn circb(x: i32, y: i32, r: i32, col: u32) {
    // Circle boundary - same as filled for outline effect
    circ(x, y, r, col);
}

/// Tilemap simulation
pub struct Tilemap {
    pub tiles: [u32; 16 * 16],
}

impl Tilemap {
    pub fn new() -> Self {
        Tilemap { tiles: [0; 256] }
    }

    pub fn pget(&self, x: i32, y: i32) -> (u32, u32) {
        if x >= 0 && x < 16 && y >= 0 && y < 16 {
            let val = self.tiles[(y as usize * 16 + x as usize)];
            (val, 0)
        } else {
            (0, 0)
        }
    }

    pub fn pset(&mut self, x: i32, y: i32, val: (u32, u32)) {
        if x >= 0 && x < 16 && y >= 0 && y < 16 {
            self.tiles[(y as usize * 16 + x as usize)] = val.0;
        }
    }

    pub fn cls(&mut self, val: (u32, u32)) {
        for i in 0..256 {
            self.tiles[i] = val.0;
        }
    }
}
