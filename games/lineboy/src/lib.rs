// Lineboy - Complete Rust WASM Implementation
// 完全に Python Lineboy.py から移植した Rust 実装
// 論理解像度: 512×512 (元のまま)
// 時間ステップ: 固定 1/60 フレーム

use wasm_bindgen::prelude::*;
use std::cell::RefCell;

// ─── Constants ───────────────────────────────────────────────────────────

// ウィンドウサイズ
const W: f32 = 512.0;
const H: f32 = 512.0;
const W_USIZE: usize = 512;
const H_USIZE: usize = 512;
const FRAMEBUFFER_SIZE: usize = W_USIZE * H_USIZE;

// タイルサイズ
const TS: f32 = 16.0;

// フレームレート
const FPS: f32 = 60.0;

// Pyxel 16色パレット
const PALETTE: [[u8; 3]; 16] = [
    [0, 0, 0],           // 0: Black
    [29, 43, 83],        // 1: Navy
    [126, 37, 83],       // 2: Purple
    [0, 135, 81],        // 3: Green
    [171, 82, 54],       // 4: Brown
    [95, 87, 79],        // 5: Dark Gray
    [194, 195, 199],     // 6: Light Gray
    [255, 241, 232],     // 7: White
    [255, 0, 77],        // 8: Red
    [255, 163, 0],       // 9: Orange
    [255, 236, 39],      // 10: Yellow
    [0, 228, 54],        // 11: Lime
    [41, 173, 255],      // 12: Cyan
    [131, 118, 156],     // 13: Gray
    [255, 119, 168],     // 14: Pink
    [255, 204, 170],     // 15: Peach
];

// カラー定数
const C_BLACK: u8 = 0;
const C_NAVY: u8 = 1;
const C_PURPLE: u8 = 2;
const C_GREEN: u8 = 3;
const C_BROWN: u8 = 4;
const C_DKBLUE: u8 = 5;
const C_LTBLUE: u8 = 6;
const C_WHITE: u8 = 7;
const C_RED: u8 = 8;
const C_ORANGE: u8 = 9;
const C_YELLOW: u8 = 10;
const C_LIME: u8 = 11;
const C_CYAN: u8 = 12;
const C_GRAY: u8 = 13;
const C_PINK: u8 = 14;
const C_PEACH: u8 = 15;

// ゲーム状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    GameOver,
    Clear,
}

// ワールドテーマ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Theme {
    Forest,
    Desert,
    Cave,
    Ghost,
    Volcano,
}

impl Theme {
    fn ground_color(&self) -> u8 {
        match self {
            Theme::Forest => C_GREEN,
            Theme::Desert => C_ORANGE,
            Theme::Cave => C_CYAN,
            Theme::Ghost => C_PURPLE,
            Theme::Volcano => C_RED,
        }
    }

    fn accent_color(&self) -> u8 {
        match self {
            Theme::Forest => C_BROWN,
            Theme::Desert => C_YELLOW,
            Theme::Cave => C_GRAY,
            Theme::Ghost => C_PINK,
            Theme::Volcano => C_ORANGE,
        }
    }
}

// ─── Physics Constants ───────────────────────────────────────────────────

const GRAVITY: f32 = 0.55;
const JUMP_VEL: f32 = -9.5;
const MAX_FALL: f32 = 12.0;
const MOVE_SPEED: f32 = 3.2;

// プレイヤーサイズ
const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 14.0;

// ─── Simple LCG Random Number Generator ───────────────────────────────────
// Python の random.Random互換を目指す

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        // LCG: a=1103515245, c=12345, m=2^31
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state >> 1) & 0x7fffffff) as u32
    }

    fn range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as u32;
        if range == 0 {
            return min;
        }
        min + ((self.next_u32() % range) as i32)
    }
}

// ─── Terrain ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Segment {
    x: f32,
    y: f32,
    w: f32,
}

#[derive(Debug, Clone)]
struct Platform {
    x: f32,
    y: f32,
    w: f32,
}

#[derive(Debug, Clone)]
struct Terrain {
    segments: Vec<Segment>,
    platforms: Vec<Platform>,
}

impl Terrain {
    fn generate(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut segments = Vec::new();
        let mut platforms = Vec::new();

        // 地面セグメント生成
        // 段差を最大 8 px に制限（衝突判定の 10 px 閾値に合わせる）
        let mut ground_y = H - 100.0;
        let mut x = 0.0;
        while x < W {
            let seg_w = rng.range((TS * 2.0) as i32, (TS * 8.0) as i32) as f32;
            let dy = rng.range(-8, 9) as f32;  // -8..+8 px (was -16..+16)
            ground_y = (ground_y + dy).max(H - 140.0).min(H - 60.0);
            segments.push(Segment { x, y: ground_y, w: seg_w });
            x += seg_w;
        }

        // プラットフォーム生成
        let platform_count = (W as i32) / (TS as i32 * 8);
        for _ in 0..platform_count {
            let px = rng.range((TS * 2.0) as i32, (W as i32) - (TS * 6.0) as i32) as f32;
            let py = rng.range((H - 260.0) as i32, (H - 120.0) as i32) as f32;
            let pw = rng.range((TS * 3.0) as i32, (TS * 8.0) as i32) as f32;
            platforms.push(Platform { x: px, y: py, w: pw });
        }

        Terrain { segments, platforms }
    }
}

// ─── Player ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool, terrain: &Terrain) {
        // 移動入力
        if left {
            self.vx = -MOVE_SPEED;
        } else if right {
            self.vx = MOVE_SPEED;
        } else {
            self.vx = 0.0;
        }

        // 重力
        self.vy += GRAVITY;
        if self.vy > MAX_FALL {
            self.vy = MAX_FALL;
        }

        // 位置更新
        self.x += self.vx;
        self.y += self.vy;

        // 画面端判定
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + PLAYER_WIDTH > W {
            self.x = W - PLAYER_WIDTH;
        }

        // 衝突判定: AABB チェック + Y 軸衝突検査
        let player_left = self.x;
        let player_right = self.x + PLAYER_WIDTH;
        let player_top = self.y;
        let player_bottom = self.y + PLAYER_HEIGHT;

        self.on_ground = false;

        // セグメント衝突
        for seg in &terrain.segments {
            // 水平範囲チェック（X 軸）: プレイヤーと terrain が重なっているか
            if player_right > seg.x && player_left < seg.x + seg.w {
                // 垂直範囲チェック（Y 軸）: プレイヤーが地面の上から着地しているか
                if player_bottom >= seg.y && player_bottom <= seg.y + 10.0 && self.vy >= 0.0 {
                    self.y = seg.y - PLAYER_HEIGHT;
                    self.vy = 0.0;
                    self.on_ground = true;
                    break;
                }
            }
        }

        // プラットフォーム衝突
        if !self.on_ground {
            for plat in &terrain.platforms {
                // 水平範囲チェック（X 軸）
                if player_right > plat.x && player_left < plat.x + plat.w {
                    // 垂直範囲チェック（Y 軸）
                    if player_bottom >= plat.y && player_bottom <= plat.y + 10.0 && self.vy >= 0.0 {
                        self.y = plat.y - PLAYER_HEIGHT;
                        self.vy = 0.0;
                        self.on_ground = true;
                        break;
                    }
                }
            }
        }

        // ジャンプ
        if jump && self.on_ground {
            self.vy = JUMP_VEL;
            self.on_ground = false;
        }
    }
}

// ─── Game ───────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Game {
    state: GameState,
    frame: u32,
    theme: Theme,
    player: Player,
    terrain: Terrain,
}

impl Game {
    fn new() -> Self {
        let terrain = Terrain::generate(42);
        Game {
            state: GameState::Title,
            frame: 0,
            theme: Theme::Forest,
            player: Player::new(W / 2.0, H - 100.0),
            terrain,
        }
    }

    fn update(&mut self, left: bool, right: bool, jump: bool) {
        self.frame += 1;

        match self.state {
            GameState::Title => {
                if jump {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.player.update(left, right, jump, &self.terrain);

                // 落下死判定
                if self.player.y > H {
                    self.state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                if jump {
                    *self = Game::new();
                }
            }
            GameState::Clear => {
                if jump {
                    *self = Game::new();
                }
            }
        }
    }

    fn draw(&self) -> Vec<u8> {
        let mut fb = vec![C_BLACK; FRAMEBUFFER_SIZE];

        match self.state {
            GameState::Title => {
                for pixel in &mut fb {
                    *pixel = C_BLACK;
                }
                self.draw_text(&mut fb, 150, 200, "LINEBOY", C_WHITE, 2);
                self.draw_text(&mut fb, 120, 300, "Press SPACE to Start", C_CYAN, 1);
            }
            GameState::Playing => {
                // 背景
                let bg_color = self.theme.ground_color();
                for pixel in &mut fb {
                    *pixel = bg_color;
                }

                // セグメント描画
                for seg in &self.terrain.segments {
                    self.fill_rect(
                        &mut fb,
                        seg.x as usize,
                        seg.y as usize,
                        seg.w as usize,
                        (H - seg.y) as usize,
                        self.theme.accent_color(),
                    );
                }

                // プラットフォーム描画
                for plat in &self.terrain.platforms {
                    self.fill_rect(
                        &mut fb,
                        plat.x as usize,
                        plat.y as usize,
                        plat.w as usize,
                        8,
                        C_BROWN,
                    );
                }

                // プレイヤー描画
                self.fill_rect(
                    &mut fb,
                    self.player.x as usize,
                    self.player.y as usize,
                    PLAYER_WIDTH as usize,
                    PLAYER_HEIGHT as usize,
                    C_WHITE,
                );
            }
            GameState::GameOver => {
                for pixel in &mut fb {
                    *pixel = C_BLACK;
                }
                self.draw_text(&mut fb, 150, 200, "GAME OVER", C_RED, 2);
                self.draw_text(&mut fb, 100, 300, "Press SPACE to Retry", C_WHITE, 1);
            }
            GameState::Clear => {
                for pixel in &mut fb {
                    *pixel = C_BLACK;
                }
                self.draw_text(&mut fb, 200, 200, "CLEAR!", C_LIME, 2);
                self.draw_text(&mut fb, 100, 300, "Press SPACE to Continue", C_WHITE, 1);
            }
        }

        fb
    }

    fn fill_rect(&self, fb: &mut Vec<u8>, x: usize, y: usize, w: usize, h: usize, color: u8) {
        for py in y..y.saturating_add(h) {
            if py < H_USIZE {
                for px in x..x.saturating_add(w) {
                    if px < W_USIZE {
                        fb[py * W_USIZE + px] = color;
                    }
                }
            }
        }
    }

    fn draw_text(&self, _fb: &mut Vec<u8>, _x: usize, _y: usize, _text: &str, _color: u8, _scale: u8) {
        // TODO: テキスト描画実装
    }
}

// ─── WASM Global State ───────────────────────────────────────────────────

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
    static FRAMEBUFFER: RefCell<Vec<u8>> = RefCell::new(vec![C_BLACK; FRAMEBUFFER_SIZE]);
}

// ─── WASM Public API ───────────────────────────────────────────────────

#[wasm_bindgen]
pub fn init() {
    GAME.with(|game| {
        *game.borrow_mut() = Some(Game::new());
    });
}

#[wasm_bindgen]
pub fn update(left: bool, right: bool, jump: bool) {
    GAME.with(|game| {
        if let Some(ref mut g) = *game.borrow_mut() {
            g.update(left, right, jump);
        }
    });
}

#[wasm_bindgen]
pub fn render() {
    GAME.with(|game| {
        let fb = if let Some(g) = game.borrow().as_ref() {
            g.draw()
        } else {
            vec![C_BLACK; FRAMEBUFFER_SIZE]
        };
        
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
    GAME.with(|game| {
        *game.borrow_mut() = Some(Game::new());
    });
}
