# Nantaraquad Getting Started Guide

ゲーム開発を始めるためのステップバイステップガイドです。

---

## 📋 前提条件

- Rust 1.70+ がインストール済み
- Cargo がインストール済み

```bash
rustc --version  # 1.70 以上を確認
cargo --version
```

---

## 🚀 Step 1: プロジェクト作成

```bash
cargo new my_game
cd my_game
```

`Cargo.toml` に依存関係を追加：

```toml
[dependencies]
nantaraquad = { path = "../Nantaraquad" }
macroquad = "0.4"
```

---

## 🎨 Step 2: 最小限のゲーム

`src/main.rs` を以下のように編集：

```rust
use nantaraquad::api::drawing::DrawingContext;
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::api::game::GameEngine;

fn main() {
    // エンジン初期化（幅160、高さ120、60 FPS）
    let mut engine = GameEngine::new(160, 120, 60);
    
    // ゲームループ
    loop {
        // 画面クリア（色7 = 白）
        engine.clear(7);
        
        // ピクセル描画
        engine.drawing.pset(80, 60, 0);  // 黒いドット
        
        // 入力チェック
        if engine.input.btn(Key::Space) {
            println!("Space pressed!");
        }
        
        // フレーム更新
        engine.update(engine.frame_time_ms());
    }
}
```

**実行：**

```bash
cargo run
```

---

## 🎮 Step 3: 入力処理

キー入力を処理します：

```rust
fn update(engine: &mut GameEngine, player_x: &mut f32, player_y: &mut f32) {
    // 連続キー（押し続けているか）
    if engine.input.btn(Key::Left) {
        *player_x -= 2.0;
    }
    if engine.input.btn(Key::Right) {
        *player_x += 2.0;
    }
    
    // エッジトリガー（キーが押された瞬間）
    if engine.input.btnp(Key::Space) {
        println!("Jump!");
    }
}
```

**対応キー：**
- `Key::Left`, `Key::Right`, `Key::Up`, `Key::Down`
- `Key::A`, `Key::W`, `Key::S`, `Key::D`
- `Key::Space`, `Key::Escape`
- `Key::GamepadButtonA`, `Key::GamepadButtonB` など

---

## 🖌️ Step 4: 描画基本

### 単一ピクセル

```rust
engine.drawing.pset(x, y, color);
```

### 矩形

```rust
// 枠線
engine.drawing.rect(x, y, width, height, color);

// 塗りつぶし
engine.drawing.rectfill(x, y, width, height, color);
```

### 円

```rust
// 枠線
engine.drawing.circle(x, y, radius, color);

// 塗りつぶし
engine.drawing.circfill(x, y, radius, color);
```

### 線

```rust
engine.drawing.line(x1, y1, x2, y2, color);
```

### テキスト

```rust
engine.drawing.print("Hello!", x, y, color);
```

**色パレット（0-15）：**
- 0 = 黒
- 1 = 青
- 2 = 緑
- 3 = 水色
- 4 = 赤
- 5 = 紫
- 6 = 黄
- 7 = 白
- 8-15 = 明るい色

---

## 📍 Step 5: プレイヤー実装

```rust
struct Player {
    x: f32,
    y: f32,
    vx: f32,  // 速度 X
    vy: f32,  // 速度 Y
    width: u32,
    height: u32,
    is_on_ground: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x, y,
            vx: 0.0,
            vy: 0.0,
            width: 4,
            height: 4,
            is_on_ground: false,
        }
    }
    
    fn update(&mut self, input: &InputState, dt: f32) {
        const MOVE_SPEED: f32 = 80.0;
        const GRAVITY: f32 = 300.0;
        const JUMP_FORCE: f32 = -150.0;
        const GROUND_Y: f32 = 110.0;
        
        // 左右移動
        if input.btn(Key::Left) {
            self.vx = -MOVE_SPEED;
        } else if input.btn(Key::Right) {
            self.vx = MOVE_SPEED;
        } else {
            self.vx = 0.0;
        }
        
        // 重力
        self.vy += GRAVITY * dt;
        
        // ジャンプ
        if input.btnp(Key::Space) && self.is_on_ground {
            self.vy = JUMP_FORCE;
            self.is_on_ground = false;
        }
        
        // 位置更新
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        
        // 地面衝突
        if self.y + self.height as f32 >= GROUND_Y {
            self.y = GROUND_Y - self.height as f32;
            self.vy = 0.0;
            self.is_on_ground = true;
        }
    }
    
    fn draw(&self, drawing: &mut DrawingContext) {
        drawing.rectfill(
            self.x as u32,
            self.y as u32,
            self.width,
            self.height,
            4,  // 赤
        );
    }
}
```

---

## 🎯 Step 6: ゲームループ統合

```rust
use nantaraquad::api::game::GameEngine;

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    let mut player = Player::new(80.0, 50.0);
    
    let start = std::time::Instant::now();
    
    loop {
        let dt = start.elapsed().as_secs_f32();
        let delta = 1.0 / 60.0;  // 60 FPS
        
        // 更新
        player.update(&engine.input, delta);
        
        // 描画
        engine.clear(0);  // 黒背景
        player.draw(&mut engine.drawing);
        
        // フレーム更新
        engine.update(delta);
    }
}
```

---

## 💥 Step 7: パーティクルエフェクト

```rust
use nantaraquad::api::particles::Particle;

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    
    loop {
        if engine.input.btn(Key::Space) {
            // x=80, y=60 からパーティクル放出
            for i in 0..5 {
                let angle = (i as f32 / 5.0) * 6.28;
                let speed = 50.0;
                engine.particles.emit(
                    80.0,                           // x
                    60.0,                           // y
                    angle.cos() * speed,            // vx
                    angle.sin() * speed,            // vy
                    7,                              // color (white)
                );
            }
        }
        
        // パーティクル更新
        engine.particles.update(1.0 / 60.0);
        
        // 画面描画
        engine.clear(0);
        engine.particles.draw(&mut engine.drawing);
        
        engine.update(1.0 / 60.0);
    }
}
```

---

## 🎬 実践例：完全なゲーム

`examples/lineboy.rs` と `examples/cubeboy.rs` を参考にしてください。

実行：

```bash
cargo run --example lineboy
cargo run --example cubeboy
```

---

## 📚 次のステップ

1. **docs/api/REFERENCE.md** — 完全な API リファレンス
2. **docs/GAME_DEVELOPMENT_GUIDE.md** — ゲーム開発パターン
3. **docs/API_USAGE_EXAMPLES.md** — 高度な使用例

---

## 🆘 よくある質問

**Q: 画面サイズを変更したい**
```rust
let engine = GameEngine::new(320, 240, 60);  // 幅320、高さ240
```

**Q: FPS を変更したい**
```rust
let engine = GameEngine::new(160, 120, 120);  // 120 FPS
```

**Q: フレームレートを計測したい**
```rust
let frame_time = engine.frame_time_ms();  // ミリ秒
```

**Q: キー入力の種類を全て見たい**

`src/api/input.rs` の `Key` enum を参照

---

Happy coding! 🎮
