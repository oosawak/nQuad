mod pyxel_api;

use pyxel_api::*;

// Cubeboy.py from Pyxel directly ported
const W: u32 = 128;
const H: u32 = 128;
const TILE_SIZE: u32 = 8;

const COLOR_BG: u32 = 0;
const COLOR_WALL: u32 = 1;
const COLOR_PLAYER_READY: u32 = 7;
const COLOR_PLAYER_SPENT: u32 = 14;
const COLOR_PARTICLE: u32 = 12;
const COLOR_ORB: u32 = 10;
const COLOR_SPIKE: u32 = 7;

const STATE_PLAY: i32 = 1;
const STATE_BOSS: i32 = 2;
const STATE_GAMECLEAR: i32 = 3;

static mut GAME: Option<Game> = None;
static mut TILEMAP: Tilemap = Tilemap { tiles: [0; 256] };

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    col: u32,
    life: i32,
}

impl Particle {
    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.life -= 1;
    }

    fn draw(&self) {
        pset(self.x as i32, self.y as i32, self.col);
    }
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: f32,
    height: f32,
    is_on_ground: bool,
    is_on_wall: i32,
    can_dash: bool,
    dash_time: i32,
    dash_dir: (f32, f32),
    coyote_timer: i32,
    jump_buffer: i32,
    stretch_x: f32,
    stretch_y: f32,
    facing: i32,
    is_dead: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x, y, vx: 0.0, vy: 0.0,
            width: 6.0, height: 6.0,
            is_on_ground: false, is_on_wall: 0, can_dash: true,
            dash_time: 0, dash_dir: (0.0, 0.0),
            coyote_timer: 0, jump_buffer: 0,
            stretch_x: 1.0, stretch_y: 1.0,
            facing: 1, is_dead: false,
        }
    }

    fn is_wall(&self, x: f32, y: f32) -> bool {
        unsafe {
            let x1 = (x / 8.0).floor() as i32;
            let y1 = (y / 8.0).floor() as i32;
            let x2 = ((x + self.width - 0.1) / 8.0).floor() as i32;
            let y2 = ((y + self.height - 0.1) / 8.0).floor() as i32;

            for ty in y1..=y2 {
                if !(0..16).contains(&ty) { continue; }
                for tx in x1..=x2 {
                    if !(0..16).contains(&tx) { continue; }
                    if TILEMAP.pget(tx, ty).0 == 1 {
                        return true;
                    }
                }
            }
            false
        }
    }

    fn resolve_overlap(&mut self) {
        if self.is_wall(self.x, self.y) {
            for r in 1..16 {
                for dx in -r..=r {
                    for dy in -r..=r {
                        if !self.is_wall(self.x + dx as f32, self.y + dy as f32) {
                            self.x += dx as f32;
                            self.y += dy as f32;
                            return;
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, particles: &mut Vec<Particle>, left: i32, right: i32, jump: i32) {
        self.resolve_overlap();

        if self.coyote_timer > 0 { self.coyote_timer -= 1; }
        if self.jump_buffer > 0 { self.jump_buffer -= 1; }

        if self.dash_time > 0 {
            self.vx = self.dash_dir.0 * 5.0;
            self.vy = self.dash_dir.1 * 5.0;
            self.dash_time -= 1;

            particles.push(Particle {
                x: self.x + 3.0, y: self.y + 3.0,
                dx: Pyxel::rndf(-1.0, 1.0), dy: Pyxel::rndf(-1.0, 1.0),
                col: COLOR_PARTICLE, life: 10,
            });

            if self.dash_time == 0 {
                self.vx *= 0.5;
                self.vy *= 0.5;
            }
        } else {
            if left != 0 {
                let target_vx = left as f32 * 2.5;
                self.vx += (target_vx - self.vx) * 0.2;
                self.facing = left;
            } else if right != 0 {
                let target_vx = right as f32 * 2.5;
                self.vx += (target_vx - self.vx) * 0.2;
                self.facing = right;
            } else {
                self.vx *= 0.7;
            }

            if self.is_on_wall != 0 && self.vy > 0.0 {
                self.vy = Pyxel::clamp(self.vy + 0.1, 0.0, 0.8);
            } else {
                let grav = if jump != 0 && self.vy < 0.0 { 0.3 } else { 0.5 };
                self.vy += grav;
            }

            if jump != 0 { self.jump_buffer = 4; }

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

        let steps_x = ((self.vx.abs() / 0.5).floor() as i32 + 1).max(1);
        let step_x = self.vx / steps_x as f32;
        for _ in 0..steps_x {
            if !self.is_wall(self.x + step_x, self.y) {
                self.x += step_x;
            } else {
                self.vx = 0.0;
                break;
            }
        }

        let steps_y = ((self.vy.abs() / 0.5).floor() as i32 + 1).max(1);
        let step_y = self.vy / steps_y as f32;
        let old_y = self.y;
        for _ in 0..steps_y {
            if !self.is_wall(self.x, self.y + step_y) {
                self.y += step_y;
            } else {
                self.vy = 0.0;
                break;
            }
        }

        self.is_on_ground = self.is_wall(self.x, self.y + 1.0);
        if self.is_on_ground {
            self.coyote_timer = 5;
            self.can_dash = true;
            if old_y < self.y {
                self.stretch_x = 1.4;
                self.stretch_y = 0.6;
            }
        }

        if self.is_wall(self.x + 1.0, self.y) { self.is_on_wall = 1; }
        else if self.is_wall(self.x - 1.0, self.y) { self.is_on_wall = -1; }
        else { self.is_on_wall = 0; }

        self.stretch_x += (1.0 - self.stretch_x) * 0.2;
        self.stretch_y += (1.0 - self.stretch_y) * 0.2;

        unsafe {
            if TILEMAP.pget((self.x as i32 + 3) / 8, (self.y as i32 + 3) / 8).0 == 2 {
                self.is_dead = true;
            }
        }
    }

    fn draw(&self) {
        let color = if self.can_dash { COLOR_PLAYER_READY } else { COLOR_PLAYER_SPENT };
        let sw = 8.0 * self.stretch_x;
        let sh = 8.0 * self.stretch_y;
        let ox = (self.width - sw) / 2.0;
        rect((self.x + ox) as i32, (self.y + self.height - sh) as i32, sw as i32, sh as i32, color);

        let eyex = (self.x + if self.facing > 0 { 4.0 } else { 0.0 }) as i32;
        pset(eyex, (self.y + 1.0) as i32, 0);
    }
}

struct Orb {
    x: f32,
    y: f32,
    active: bool,
    timer: i32,
}

impl Orb {
    fn new(x: f32, y: f32) -> Self { Orb { x, y, active: true, timer: 0 } }

    fn update(&mut self, player: &mut Player) -> bool {
        if !self.active {
            self.timer += 1;
            if self.timer > 90 {
                self.active = true;
                self.timer = 0;
            }
        } else {
            let dx = (player.x + 3.0) - (self.x + 4.0);
            let dy = (player.y + 3.0) - (self.y + 4.0);
            let dist = Pyxel::sqrt(dx * dx + dy * dy);
            if dist < 10.0 {
                if !player.can_dash {
                    player.can_dash = true;
                    self.active = false;
                    self.timer = 0;
                    return true;
                }
            }
        }
        false
    }

    fn draw(&self) {
        let fc = Pyxel::frame_count();
        if self.active {
            let t = fc / 4;
            let off = Pyxel::sin((t as f32) * 20.0) * 2.0;
            circ((self.x + 4.0) as i32, (self.y + 4.0 + off) as i32, 3, COLOR_ORB);
            circb((self.x + 4.0) as i32, (self.y + 4.0 + off) as i32, 3, 7);
        } else {
            circb((self.x + 4.0) as i32, (self.y + 4.0) as i32, 2, 5);
        }
    }
}

struct Boss {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Boss {
    fn new(x: f32, y: f32) -> Self { Boss { x, y, w: 24.0, h: 24.0, speed: 0.8 } }

    fn update(&mut self, player: &Player) {
        let dx = (player.x + 3.0) - (self.x + self.w / 2.0);
        let dy = (player.y + 3.0) - (self.y + self.h / 2.0);
        let dist = Pyxel::sqrt(dx * dx + dy * dy);
        if dist > 0.0 {
            self.x += (dx / dist) * self.speed;
            self.y += (dy / dist) * self.speed;
        }

        unsafe {
            let tx_start = (self.x / 8.0).floor() as i32;
            let ty_start = (self.y / 8.0).floor() as i32;
            let tx_end = ((self.x + self.w) / 8.0).floor() as i32;
            let ty_end = ((self.y + self.h) / 8.0).floor() as i32;

            for ty in ty_start..=ty_end {
                for tx in tx_start..=tx_end {
                    if (0..16).contains(&tx) && (0..16).contains(&ty) {
                        if TILEMAP.pget(tx, ty).0 == 1 {
                            TILEMAP.pset(tx, ty, (0, 0));
                        }
                    }
                }
            }
        }
    }

    fn draw(&self) {
        rect(self.x as i32, self.y as i32, self.w as i32, self.h as i32, 8);
        rectb(self.x as i32, self.y as i32, self.w as i32, self.h as i32, 7);
        rect((self.x + 4.0) as i32, (self.y + 6.0) as i32, 4, 4, 7);
        rect((self.x + 16.0) as i32, (self.y + 6.0) as i32, 4, 4, 7);
    }
}

struct Game {
    player: Player,
    particles: Vec<Particle>,
    orbs: Vec<Orb>,
    boss: Boss,
    state: i32,
    collected_orbs: i32,
    room_x: i32,
    room_y: i32,
    collected_rooms: std::collections::HashSet<(i32, i32)>,
    boss_countdown: i32,
}

impl Game {
    fn new() -> Self {
        unsafe {
            TILEMAP.cls((0, 0));
            Self::generate_room(0, 0);
        }

        let mut orbs = Vec::new();
        orbs.push(Orb::new(64.0, 24.0));

        Game {
            player: Player::new(64.0, 64.0),
            particles: Vec::new(),
            orbs,
            boss: Boss::new(-100.0, -100.0),
            state: STATE_PLAY,
            collected_orbs: 0,
            room_x: 0,
            room_y: 0,
            collected_rooms: std::collections::HashSet::new(),
            boss_countdown: 0,
        }
    }

    fn generate_room(rx: i32, ry: i32) {
        unsafe {
            TILEMAP.cls((0, 0));

            // Borders
            for x in 0..16 {
                for y in 0..16 {
                    let is_edge = x == 0 || x == 15 || y == 0 || y == 15;
                    let is_exit = (7..=8).contains(&x) && (y == 0 || y == 15)
                        || (7..=8).contains(&y) && (x == 0 || x == 15);
                    if is_edge && !is_exit {
                        TILEMAP.pset(x as i32, y as i32, (1, 0));
                    }
                }
            }

            // Simple platform generation (seeded)
            let seed = Self::hash_seed(rx, ry);
            let density = 0.8;

            for gy in 1..7 {
                for gx in 1..7 {
                    let rnd = Self::seeded_rand(seed, gx * 100 + gy);
                    if rnd < density {
                        let px = gx * 2;
                        let py = gy * 2;
                        let typ = (Self::seeded_rand(seed, gx * 1000 + gy * 10) * 4.0) as u32 % 4;
                        let size = ((Self::seeded_rand(seed, gx * 10000 + gy * 100) * 2.0) + 2.0) as u32;

                        match typ {
                            0 => { // Horizontal
                                for i in 0..size {
                                    if 0 < px + i && px + i < 15 {
                                        TILEMAP.pset((px + i) as i32, py as i32, (1, 0));
                                    }
                                }
                            }
                            1 => { // Vertical
                                for i in 0..size {
                                    if 0 < py + i && py + i < 15 {
                                        TILEMAP.pset(px as i32, (py + i) as i32, (1, 0));
                                    }
                                }
                            }
                            2 => { // L-Shape
                                for i in 0..size {
                                    if 0 < px + i && px + i < 15 {
                                        TILEMAP.pset((px + i) as i32, py as i32, (1, 0));
                                    }
                                }
                                for i in 0..size {
                                    if 0 < py + i && py + i < 15 {
                                        TILEMAP.pset(px as i32, (py + i) as i32, (1, 0));
                                    }
                                }
                            }
                            _ => { TILEMAP.pset(px as i32, py as i32, (1, 0)); }
                        }
                    }
                }
            }

            // Clear exits
            for i in 7..=8 {
                for j in 0..2 {
                    TILEMAP.pset(i, j as i32, (0, 0));
                    TILEMAP.pset(i, (15 - j) as i32, (0, 0));
                }
            }
            for j in 7..=8 {
                for i in 0..2 {
                    TILEMAP.pset(i as i32, j, (0, 0));
                    TILEMAP.pset((15 - i) as i32, j, (0, 0));
                }
            }

            // Safe zone
            for i in 6..10 {
                for j in 6..10 {
                    TILEMAP.pset(i, j, (0, 0));
                }
            }

            // Spikes
            for i in 0..15 {
                let tx = ((seed ^ (i as u32)) % 14 + 1) as i32;
                let ty = ((seed ^ ((i as u32) * 2)) % 14 + 1) as i32;
                if (tx == 7 || tx == 8) || (ty == 7 || ty == 8) || (6..=9).contains(&tx) && (6..=9).contains(&ty) {
                    continue;
                }
                if TILEMAP.pget(tx, ty).0 == 0 {
                    let mut adj = false;
                    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        if TILEMAP.pget(tx + dx, ty + dy).0 == 1 {
                            adj = true;
                            break;
                        }
                    }
                    if adj {
                        TILEMAP.pset(tx, ty, (2, 0));
                    }
                }
            }
        }
    }

    fn hash_seed(rx: i32, ry: i32) -> u32 {
        let s = format!("{}_{}_cubeboy", rx, ry);
        let mut h = 5381u32;
        for c in s.as_bytes() {
            h = ((h << 5).wrapping_add(h)).wrapping_add(*c as u32);
        }
        h
    }

    fn seeded_rand(seed: u32, offset: u32) -> f32 {
        let x = ((seed ^ (offset.wrapping_mul(2654435761))) ^ (seed >> 15)) as u32;
        (x % 1000) as f32 / 1000.0
    }

    fn update(&mut self, left: i32, right: i32, jump: i32) {
        Pyxel::tick_frame();

        match self.state {
            STATE_PLAY => {
                self.player.update(&mut self.particles, left, right, jump);

                if self.player.is_dead {
                    self.player.x = 64.0;
                    self.player.y = 64.0;
                    self.player.vx = 0.0;
                    self.player.vy = 0.0;
                    self.player.is_dead = false;
                }

                const MARGIN: f32 = 4.0;
                if self.player.x < -MARGIN {
                    self.room_x -= 1;
                    self.player.x = 128.0 - self.player.width - 12.0;
                    Self::generate_room(self.room_x, self.room_y);
                } else if self.player.x > 128.0 + MARGIN {
                    self.room_x += 1;
                    self.player.x = self.player.width + 12.0;
                    Self::generate_room(self.room_x, self.room_y);
                } else if self.player.y < -MARGIN {
                    self.room_y -= 1;
                    self.player.y = 128.0 - self.player.height - 12.0;
                    Self::generate_room(self.room_x, self.room_y);
                } else if self.player.y > 128.0 + MARGIN {
                    self.room_y += 1;
                    self.player.y = self.player.height + 12.0;
                    Self::generate_room(self.room_x, self.room_y);
                }

                for orb in &mut self.orbs {
                    if orb.update(&mut self.player) {
                        self.collected_orbs += 1;
                        self.collected_rooms.insert((self.room_x, self.room_y));
                    }
                }
                self.orbs.retain(|o| o.active);

                if self.collected_orbs >= 3 {
                    self.state = STATE_BOSS;
                    self.boss_countdown = 60;
                }
            }
            STATE_BOSS => {
                self.player.update(&mut self.particles, left, right, jump);

                if self.boss_countdown > 0 {
                    self.boss_countdown -= 1;
                    if self.boss_countdown == 0 {
                        self.boss.x = self.player.x - 64.0;
                        self.boss.y = self.player.y - 64.0;
                    }
                } else {
                    self.boss.update(&self.player);

                    if self.player.x < self.boss.x + self.boss.w &&
                       self.player.x + self.player.width > self.boss.x &&
                       self.player.y < self.boss.y + self.boss.h &&
                       self.player.y + self.player.height > self.boss.y {
                        if self.room_x == 0 && self.room_y == 0 {
                            self.state = STATE_GAMECLEAR;
                        }
                    }
                }
            }
            STATE_GAMECLEAR => {
                if jump != 0 {
                    *self = Self::new();
                }
            }
            _ => {}
        }

        for p in &mut self.particles {
            p.update();
        }
        self.particles.retain(|p| p.life > 0);
    }

    fn draw(&self) {
        // Clear background
        unsafe {
            for i in 0..(128*128) {
                pyxel_api::FRAMEBUFFER.lock().unwrap()[i] = 0;
            }
        }

        // Background
        for x in 0..128 {
            let h = ((x as i32 * 3) / 4) % 30 + 10;
            rect(x as i32, (128 - h) as i32, 1, h, 13);
        }
        circ(110, 15, 10, 7);

        // Tilemap
        unsafe {
            for y in 0..16 {
                for x in 0..16 {
                    let tile = TILEMAP.pget(x as i32, y as i32).0;
                    if tile == 1 {
                        rect(x as i32 * 8, y as i32 * 8, 8, 8, 1);
                    } else if tile == 2 {
                        rect(x as i32 * 8, y as i32 * 8, 8, 8, 7);
                    }
                }
            }
        }

        for orb in &self.orbs {
            orb.draw();
        }

        self.player.draw();

        if self.state == STATE_BOSS {
            self.boss.draw();
        }

        for p in &self.particles {
            p.draw();
        }

        if self.state == STATE_GAMECLEAR {
            for x in 30..98 {
                rect(x as i32, 60, 1, 8, 10);
                rect(x as i32, 68, 1, 8, 10);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn init() {
    unsafe { GAME = Some(Game::new()); }
}

#[no_mangle]
pub extern "C" fn update(left: i32, right: i32, jump: i32) {
    unsafe {
        if let Some(ref mut game) = GAME {
            game.update(left, right, jump);
        }
    }
}

#[no_mangle]
pub extern "C" fn render() {
    unsafe {
        if let Some(ref game) = GAME {
            game.draw();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_framebuffer() -> *const u32 {
    unsafe {
        let fb = pyxel_api::FRAMEBUFFER.lock().unwrap();
        fb.as_ptr()
    }
}
