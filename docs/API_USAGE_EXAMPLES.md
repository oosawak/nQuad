# Nantaraquad API Usage Examples

各 API の詳細な使用例とパターン集です。

---

## 📚 Table of Contents

1. [Drawing API](#drawing-api)
2. [Input API](#input-api)
3. [Particle System](#particle-system)
4. [Camera & Viewport](#camera--viewport)
5. [Game Engine](#game-engine)
6. [Resource Management](#resource-management)
7. [Advanced Patterns](#advanced-patterns)

---

## Drawing API

### 基本ピクセル操作

```rust
use nantaraquad::api::drawing::DrawingContext;

fn draw_pixels(drawing: &mut DrawingContext) {
    // 単一ピクセル描画
    drawing.pset(50, 50, 7);  // x=50, y=50, 白色
    
    // ピクセル読み取り
    let color = drawing.pget(50, 50);
    println!("Color at (50, 50): {}", color);
}
```

### 矩形描画

```rust
// 枠線矩形
drawing.rect(10, 10, 50, 30, 4);  // 赤い枠

// 塗りつぶし矩形
drawing.rectfill(20, 20, 40, 40, 2);  // 緑の四角
```

### 円描画

```rust
// 円の枠線
drawing.circle(80, 60, 15, 1);  // 青い円

// 塗りつぶし円
drawing.circfill(120, 80, 20, 3);  // 水色の円
```

### 線描画

```rust
// 直線
drawing.line(0, 0, 160, 120, 7);  // 左上から右下へ白い線

// グラデーション風の線
for i in 0..10 {
    let y = i * 10;
    drawing.line(0, y, 160, y, i as u8 % 8);
}
```

### テキスト描画

```rust
// シンプルテキスト
drawing.print("Hello World", 40, 50, 7);

// スコア表示
let score = 12345;
drawing.print(&format!("SCORE: {}", score), 10, 10, 7);

// マルチラインテキスト
drawing.print("Line 1", 10, 20, 7);
drawing.print("Line 2", 10, 30, 7);
drawing.print("Line 3", 10, 40, 7);
```

### 色パレットの理解

```rust
// Pyxel互換パレット（0-15）
const COLORS: &[&str] = &[
    "Black",       // 0
    "Navy",        // 1
    "Green",       // 2
    "Cyan",        // 3
    "Red",         // 4
    "Purple",      // 5
    "Yellow",      // 6
    "White",       // 7
    "Dark Gray",   // 8
    "Light Blue",  // 9
    "Light Green", // 10
    "Light Cyan",  // 11
    "Light Red",   // 12
    "Pink",        // 13
    "Light Yellow",// 14
    "Pale",        // 15
];

fn print_color_palette(drawing: &mut DrawingContext) {
    for (i, color_name) in COLORS.iter().enumerate() {
        let y = (i as u32) * 10;
        drawing.rectfill(0, y, 160, 10, i as u8);
        drawing.print(color_name, 20, y + 2, 7);
    }
}
```

---

## Input API

### キー入力の基本

```rust
use nantaraquad::api::input::{InputState, Key};

fn handle_input(input: &InputState) {
    // 押し続けている（連続判定）
    if input.btn(Key::Left) {
        println!("Left key held");
    }
    
    // キーが押された瞬間（エッジトリガー）
    if input.btnp(Key::Space) {
        println!("Space pressed this frame");
    }
}
```

### プレイヤー移動

```rust
struct PlayerMovement {
    x: f32,
    y: f32,
    speed: f32,
}

impl PlayerMovement {
    fn handle_input(&mut self, input: &InputState) {
        // WASD キーで移動
        if input.btn(Key::W) || input.btn(Key::Up) {
            self.y -= self.speed;
        }
        if input.btn(Key::S) || input.btn(Key::Down) {
            self.y += self.speed;
        }
        if input.btn(Key::A) || input.btn(Key::Left) {
            self.x -= self.speed;
        }
        if input.btn(Key::D) || input.btn(Key::Right) {
            self.x += self.speed;
        }
    }
}
```

### メニュー操作

```rust
struct MenuState {
    selected: usize,
    items: Vec<String>,
}

impl MenuState {
    fn handle_input(&mut self, input: &InputState) {
        // 上下で選択移動
        if input.btnp(Key::Up) {
            if self.selected > 0 {
                self.selected -= 1;
            }
        }
        if input.btnp(Key::Down) {
            if self.selected < self.items.len() - 1 {
                self.selected += 1;
            }
        }
        
        // Space で決定
        if input.btnp(Key::Space) {
            println!("Selected: {}", self.items[self.selected]);
        }
    }
}
```

### ゲームパッド対応

```rust
fn handle_gamepad_input(input: &InputState) {
    // ゲームパッドボタン
    if input.btn(Key::GamepadButtonA) {
        println!("A button held");
    }
    
    if input.btnp(Key::GamepadButtonB) {
        println!("B button pressed");
    }
    
    // アナログ入力（十字キー相当）
    if input.btn(Key::Left) {
        println!("Left (works with both keyboard and gamepad)");
    }
}
```

---

## Particle System

### 基本的な放出

```rust
use nantaraquad::api::particles::ParticleSystem;

fn spawn_particles(particles: &mut ParticleSystem, x: f32, y: f32) {
    // 単一パーティクル
    particles.emit(
        x, y,           // 位置
        10.0, -30.0,    // 速度 (vx, vy)
        7,              // 色（白）
    );
}
```

### 爆発パターン

```rust
fn explosion(particles: &mut ParticleSystem, x: f32, y: f32, num_particles: usize) {
    for i in 0..num_particles {
        let angle = (i as f32 / num_particles as f32) * 6.28;  // 0 〜 2π
        let speed = 80.0;
        
        particles.emit(
            x, y,
            angle.cos() * speed,
            angle.sin() * speed,
            4,  // 赤
        );
    }
}
```

### 雨パターン

```rust
fn rain(particles: &mut ParticleSystem, screen_width: u32, screen_height: u32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for _ in 0..10 {
        let x = rng.gen_range(0..screen_width) as f32;
        let y = rng.gen_range(0..screen_height) as f32;
        
        particles.emit(
            x, y,
            0.0,      // 水平速度なし
            50.0,     // 下への速度
            1,        // 青
        );
    }
}
```

### パーティクルの更新と描画

```rust
use nantaraquad::api::drawing::DrawingContext;

fn update_and_draw(
    particles: &mut ParticleSystem,
    drawing: &mut DrawingContext,
    dt: f32,
) {
    // パーティクル更新（重力、寿命管理）
    particles.update(dt);
    
    // 描画
    particles.draw(drawing);
}
```

---

## Camera & Viewport

### カメラの移動

```rust
use nantaraquad::api::camera::Camera;

fn follow_player(camera: &mut Camera, player_x: f32, player_y: f32) {
    // カメラをプレイヤーの中央に配置
    camera.x = player_x - 80.0;  // 画面幅の半分
    camera.y = player_y - 60.0;  // 画面高さの半分
}
```

### ズーム機能

```rust
fn zoom_effect(camera: &mut Camera, scale: f32) {
    // ズーム（1.0 = 等倍、2.0 = 2倍）
    camera.zoom = scale;
}
```

### ワールド座標からスクリーン座標への変換

```rust
fn world_to_screen(camera: &Camera, world_x: f32, world_y: f32) -> (f32, f32) {
    let screen_x = (world_x - camera.x) * camera.zoom;
    let screen_y = (world_y - camera.y) * camera.zoom;
    (screen_x, screen_y)
}
```

---

## Game Engine

### エンジンの初期化と更新

```rust
use nantaraquad::api::game::GameEngine;

fn main() {
    // エンジン作成（幅、高さ、FPS）
    let mut engine = GameEngine::new(160, 120, 60);
    
    loop {
        // 画面クリア
        engine.clear(0);
        
        // ゲームロジック
        update(&mut engine);
        draw(&mut engine);
        
        // フレーム更新
        engine.update(engine.frame_time_ms());
    }
}

fn update(engine: &mut GameEngine) {
    // 入力処理
    if engine.input.btn(Key::Up) {
        println!("Moving up");
    }
    
    // パーティクル更新
    engine.particles.update(engine.frame_time_ms());
}

fn draw(engine: &mut GameEngine) {
    // 描画
    engine.drawing.pset(50, 50, 7);
    engine.particles.draw(&mut engine.drawing);
}
```

### フレームレート管理

```rust
fn frame_timing(engine: &GameEngine) {
    // フレームあたりのミリ秒
    let frame_time_ms = engine.frame_time_ms();
    println!("Frame time: {}ms", frame_time_ms);
    
    // 一定時間を何フレームで表現するか
    let duration_ms = 1000.0;  // 1秒
    let frames = engine.frames_for_duration(duration_ms);
    println!("1 second = {} frames at 60 FPS", frames);
}
```

---

## Resource Management

### スプライト作成と保存

```rust
use nantaraquad::{create_sprite, set_pixel, draw_sprite};

fn create_and_save_sprite() -> Result<(), String> {
    // スプライト作成（32x32、フルカラー）
    let sprite_id = create_sprite(32, 32);
    
    // ピクセル編集
    set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?;  // 赤
    set_pixel(sprite_id, 1, 0, &[0, 255, 0, 255])?;  // 緑
    set_pixel(sprite_id, 2, 0, &[0, 0, 255, 255])?;  // 青
    
    // 描画
    draw_sprite(sprite_id, 100.0, 100.0);
    
    Ok(())
}
```

### インデックスカラースプライト

```rust
use nantaraquad::create_indexed_sprite;

fn create_indexed_sprite_example() {
    // パレット定義（256色まで）
    let palette = vec![
        [0, 0, 0, 255],        // 0: 黒
        [255, 0, 0, 255],      // 1: 赤
        [0, 255, 0, 255],      // 2: 緑
        [0, 0, 255, 255],      // 3: 青
    ];
    
    // スプライト作成（16x16、パレットベース）
    let sprite_id = create_indexed_sprite(16, 16, palette);
    
    // ピクセル設定（1バイト = パレットインデックス）
    set_pixel(sprite_id, 0, 0, &[1])?;  // 赤
    set_pixel(sprite_id, 1, 0, &[2])?;  // 緑
    set_pixel(sprite_id, 2, 0, &[3])?;  // 青
}
```

---

## Advanced Patterns

### ゲームステート管理

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    Paused,
    GameOver,
}

struct Game {
    state: GameState,
    score: u32,
    lives: u32,
}

impl Game {
    fn update(&mut self, engine: &mut GameEngine) {
        match self.state {
            GameState::Title => self.update_title(engine),
            GameState::Playing => self.update_playing(engine),
            GameState::Paused => self.update_paused(engine),
            GameState::GameOver => self.update_game_over(engine),
        }
    }
    
    fn update_title(&mut self, engine: &GameEngine) {
        if engine.input.btnp(Key::Space) {
            self.state = GameState::Playing;
        }
    }
    
    fn update_playing(&mut self, engine: &GameEngine) {
        if engine.input.btnp(Key::Escape) {
            self.state = GameState::Paused;
        }
    }
    
    fn update_paused(&mut self, engine: &GameEngine) {
        if engine.input.btnp(Key::Escape) {
            self.state = GameState::Playing;
        }
    }
    
    fn update_game_over(&mut self, engine: &GameEngine) {
        if engine.input.btnp(Key::Space) {
            self.state = GameState::Title;
            self.score = 0;
            self.lives = 3;
        }
    }
}
```

### 物理演算パターン

```rust
struct PhysicsObject {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    gravity: f32,
    friction: f32,
}

impl PhysicsObject {
    fn update(&mut self, dt: f32) {
        // 重力適用
        self.vy += self.gravity * dt;
        
        // 摩擦適用
        self.vx *= 1.0 - self.friction * dt;
        
        // 位置更新
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }
    
    fn collides_with(&self, other: &PhysicsObject) -> bool {
        // 単純なAABB衝突判定
        let w1 = 10.0;
        let h1 = 10.0;
        let w2 = 10.0;
        let h2 = 10.0;
        
        self.x < other.x + w2 &&
            self.x + w1 > other.x &&
            self.y < other.y + h2 &&
            self.y + h1 > other.y
    }
}
```

### エフェクトシステム

```rust
struct Effect {
    x: f32,
    y: f32,
    lifetime: f32,
    max_lifetime: f32,
    effect_type: EffectType,
}

enum EffectType {
    Explosion,
    Heal,
    Damage,
}

impl Effect {
    fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
    }
    
    fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
    
    fn draw(&self, drawing: &mut DrawingContext) {
        let progress = self.lifetime / self.max_lifetime;
        let radius = (1.0 - progress) * 20.0;
        
        match self.effect_type {
            EffectType::Explosion => {
                drawing.circle(
                    self.x as u32,
                    self.y as u32,
                    radius as u32,
                    4,  // 赤
                );
            }
            EffectType::Heal => {
                drawing.circle(
                    self.x as u32,
                    self.y as u32,
                    radius as u32,
                    2,  // 緑
                );
            }
            EffectType::Damage => {
                drawing.circle(
                    self.x as u32,
                    self.y as u32,
                    radius as u32,
                    1,  // 青
                );
            }
        }
    }
}
```

---

## 参考

- `examples/lineboy.rs` — プラットフォーマーゲーム例
- `examples/cubeboy.rs` — 高度なプラットフォーマー例
- `docs/api/REFERENCE.md` — 完全な API リファレンス

