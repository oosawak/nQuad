// 3D Demo - Isometric Cube Rotation
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

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn project_iso(&self, center_x: f32, center_y: f32) -> (i32, i32) {
        let iso_x = (self.x - self.z) * 0.707;
        let iso_y = (self.x + self.z) * 0.35 - self.y;
        let screen_x = (center_x + iso_x) as i32;
        let screen_y = (center_y + iso_y) as i32;
        (screen_x, screen_y)
    }

    fn rotate_y(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vec3 {
            x: self.x * cos_a - self.z * sin_a,
            y: self.y,
            z: self.x * sin_a + self.z * cos_a,
        }
    }

    fn rotate_x(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vec3 {
            x: self.x,
            y: self.y * cos_a - self.z * sin_a,
            z: self.y * sin_a + self.z * cos_a,
        }
    }

    fn rotate_z(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vec3 {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
            z: self.z,
        }
    }
}

struct Cube {
    x: f32,
    y: f32,
    z: f32,
    size: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
}

impl Cube {
    fn new(x: f32, y: f32, z: f32, size: f32) -> Self {
        Cube {
            x,
            y,
            z,
            size,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
        }
    }

    fn get_vertex(&self, i: usize) -> Vec3 {
        let h = self.size / 2.0;
        let mut v = match i {
            0 => Vec3::new(-h, -h, -h),
            1 => Vec3::new(h, -h, -h),
            2 => Vec3::new(h, h, -h),
            3 => Vec3::new(-h, h, -h),
            4 => Vec3::new(-h, -h, h),
            5 => Vec3::new(h, -h, h),
            6 => Vec3::new(h, h, h),
            7 => Vec3::new(-h, h, h),
            _ => Vec3::new(0.0, 0.0, 0.0),
        };

        v = v.rotate_x(self.rot_x);
        v = v.rotate_y(self.rot_y);
        v = v.rotate_z(self.rot_z);

        Vec3 {
            x: v.x + self.x,
            y: v.y + self.y,
            z: v.z + self.z,
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        if left {
            self.rot_y += 0.05;
        }
        if right {
            self.rot_y -= 0.05;
        }
        if jump {
            self.rot_x += 0.05;
        }
    }

    fn draw(&self, fb: &mut Vec<u8>) {
        let center_x = 256.0;
        let center_y = 256.0;

        let verts: Vec<(i32, i32)> = (0..8)
            .map(|i| self.get_vertex(i).project_iso(center_x, center_y))
            .collect();

        // Bottom face edges
        draw_line(fb, verts[0].0, verts[0].1, verts[1].0, verts[1].1, C_WHITE);
        draw_line(fb, verts[1].0, verts[1].1, verts[5].0, verts[5].1, C_WHITE);
        draw_line(fb, verts[5].0, verts[5].1, verts[4].0, verts[4].1, C_WHITE);
        draw_line(fb, verts[4].0, verts[4].1, verts[0].0, verts[0].1, C_WHITE);

        // Top face edges
        draw_line(fb, verts[3].0, verts[3].1, verts[2].0, verts[2].1, C_YELLOW);
        draw_line(fb, verts[2].0, verts[2].1, verts[6].0, verts[6].1, C_YELLOW);
        draw_line(fb, verts[6].0, verts[6].1, verts[7].0, verts[7].1, C_YELLOW);
        draw_line(fb, verts[7].0, verts[7].1, verts[3].0, verts[3].1, C_YELLOW);

        // Vertical edges
        draw_line(fb, verts[0].0, verts[0].1, verts[3].0, verts[3].1, C_CYAN);
        draw_line(fb, verts[1].0, verts[1].1, verts[2].0, verts[2].1, C_CYAN);
        draw_line(fb, verts[5].0, verts[5].1, verts[6].0, verts[6].1, C_CYAN);
        draw_line(fb, verts[4].0, verts[4].1, verts[7].0, verts[7].1, C_CYAN);

        // Vertices
        for v in verts.iter() {
            set_pixel(fb, v.0, v.1, C_LIME);
            set_pixel(fb, v.0 + 1, v.1, C_LIME);
            set_pixel(fb, v.0, v.1 + 1, C_LIME);
            set_pixel(fb, v.0 + 1, v.1 + 1, C_LIME);
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

fn draw_line(fb: &mut Vec<u8>, x0: i32, y0: i32, x1: i32, y1: i32, color: u8) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = (dx as f32 - dy as f32) / 2.0;
    let mut x = x0;
    let mut y = y0;

    loop {
        set_pixel(fb, x, y, color);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = err;
        if e2 > -(dx as f32) / 2.0 {
            err -= dy as f32;
            x += sx;
        }
        if e2 < (dx as f32) / 2.0 {
            err += dx as f32;
            y += sy;
        }
    }
}

thread_local! {
    static CUBE: RefCell<Cube> = RefCell::new(Cube::new(0.0, 0.0, 0.0, 80.0));
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BLACK; FRAMEBUFFER_SIZE]);
}

#[wasm_bindgen]
pub fn init() {
    CUBE.with(|c| {
        *c.borrow_mut() = Cube::new(0.0, 0.0, 0.0, 80.0);
    });
}

#[wasm_bindgen]
pub fn update(left: bool, right: bool, jump: bool) {
    CUBE.with(|c| {
        c.borrow_mut().update(left, right, jump);
    });
}

#[wasm_bindgen]
pub fn render() {
    CUBE.with(|cube_cell| {
        let cube = cube_cell.borrow();
        let mut fb = vec![C_NAVY; FRAMEBUFFER_SIZE];

        for y in (0..H).step_by(32) {
            draw_line(&mut fb, 0, y as i32, W as i32, y as i32, C_DKGRAY);
        }
        for x in (0..W).step_by(32) {
            draw_line(&mut fb, x as i32, 0, x as i32, H as i32, C_DKGRAY);
        }

        cube.draw(&mut fb);

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
