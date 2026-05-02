# Nantaraquad Pyxel互換API - 開発者ガイド

## 概要

Nantaraquad は既存の内部 API を保ちながら、Pyxel互換のAPIレイヤーを提供しています。
これにより、Pyxel経験者がそのまま Nantaraquad でゲーム開発できます。

## API構成

### 1. 既存 API（変更なし）

```rust
use nantaraquad::api::*;

// グローバルエンジン API
add_sprite(sprite)
draw_sprite(id, x, y)
create_sprite(w, h)
load_sprite(path)
```

### 2. Pyxel互換API（新規追加）

```rust
use nantaraquad::api::{
    // グラフィックス
    cls, pset, pget, line, rect, rectfill, circle, circfill,
    // スプライト
    spr,
    // 入力
    btn, btnp,
    // カメラ
    camera, zoom,
    // オーディオ
    sfx, music, stop, music_stop,
    // 色
    set_palette,
    // フレーム
    frame_time, frames_for_ms,
    // アニメーション
    spr_anim, anim_update, anim_set_frame, anim_get_frame,
    // テキスト・デバッグ
    print, stat,
};
```

## 使用例

### 基本的なゲーム構造

```rust
use nantaraquad::api::*;
use nantaraquad::api::framework::*;
use nantaraquad::api::input::Key;

struct MyGame {
    player_x: i32,
    player_y: i32,
    player_sprite: usize,
}

impl GameApp for MyGame {
    fn update(&mut self, _delta_ms: f32) {
        // Pyxel互換入力API
        if btn(Key::Up) { self.player_y -= 2; }
        if btn(Key::Down) { self.player_y += 2; }
        if btn(Key::Left) { self.player_x -= 2; }
        if btn(Key::Right) { self.player_x += 2; }
    }
    
    fn draw(&mut self) {
        // Pyxel互換描画API
        cls(0); // 黒でクリア
        
        // スプライト描画
        spr(self.player_sprite, self.player_x as f32, self.player_y as f32);
        
        // 図形描画
        rect(10, 10, 100, 50, 7); // 白い矩形枠
        rectfill(50, 50, 30, 30, 3); // 緑の矩形
        circle(100, 100, 15, 8); // 赤い円
        
        // デバッグ表示
        print(&format!("X: {}", self.player_x), 5, 5, 7);
    }
}

fn main() {
    let game = MyGame {
        player_x: 100,
        player_y: 100,
        player_sprite: 0,
    };
    
    let mut runner = GameRunner::new(game, "My Game", 256, 256, 60);
    runner.run();
}
```

### グラフィックス描画

```rust
// クリア
cls(0); // インデックス0（黒）でクリア

// ピクセル操作
pset(50, 50, 7); // 単一ピクセル描画
if let Some(col) = pget(50, 50) {
    println!("Color: {}", col);
}

// 線
line(0, 0, 100, 100, 7); // 白い対角線

// 矩形
rect(10, 10, 50, 30, 7); // 矩形枠
rectfill(10, 10, 50, 30, 3); // 矩形塗りつぶし

// 円
circle(80, 60, 20, 7); // 円枠
circfill(80, 60, 15, 8); // 円塗りつぶし
```

### スプライト操作

```rust
// スプライト描画
spr(sprite_id, 100.0, 50.0);

// アニメーション付き描画
spr_anim(sprite_id, 100.0, 50.0, anim_id);

// アニメーション更新
let dt = frame_time();
anim_update(dt);

// フレーム操作
anim_set_frame(anim_id, 3); // フレーム3に設定
let frame = anim_get_frame(anim_id);
```

### 入力処理

```rust
use nantaraquad::api::input::Key;

// キー判定
if btn(Key::Up) {
    player_y -= 1;
}

// キー押下検出（1フレーム）
if btnp(Key::Z) {
    fire_bullet(); // ショットは1フレームに1回のみ
}
```

### カメラ制御

```rust
// カメラ位置設定
camera(player_x as f32 - 80.0, player_y as f32 - 60.0);

// ズーム
zoom(2.0); // 2倍ズーム
zoom(1.0); // 通常ズーム
```

### オーディオ

```rust
// 効果音
sfx(0); // サウンド0を再生

// BGM
music(0); // BGM 0を再生

// 停止
stop(); // すべてのサウンド停止
music_stop(); // BGM停止
```

### パレット・色

```rust
// パレット色設定
set_palette(8, 255, 0, 0); // パレット8を赤に設定
set_palette(9, 0, 255, 0); // パレット9を緑に設定
set_palette(10, 0, 0, 255); // パレット10を青に設定
```

## API対応表

| Pyxel | Nantaraquad | 状態 | 説明 |
|-------|-------------|------|------|
| `cls(col)` | `cls(col)` | ✅ | 画面クリア |
| `pset(x, y, col)` | `pset(x, y, col)` | ✅ | ピクセル描画 |
| `pget(x, y)` | `pget(x, y)` | ✅ | ピクセル読み取り |
| `line(x1, y1, x2, y2, col)` | `line(x1, y1, x2, y2, col)` | ✅ | 線描画 |
| `rect(x, y, w, h, col)` | `rect(x, y, w, h, col)` | ✅ | 矩形枠 |
| `rectfill(x, y, w, h, col)` | `rectfill(x, y, w, h, col)` | ✅ | 矩形塗りつぶし |
| `circle(x, y, r, col)` | `circle(x, y, r, col)` | ✅ | 円枠 |
| `circfill(x, y, r, col)` | `circfill(x, y, r, col)` | ✅ | 円塗りつぶし |
| `spr(n, x, y, col)` | `spr(n, x, y)` | ✅ | スプライト描画 |
| `btn(key)` | `btn(key)` | ✅ | キー押下判定 |
| `btnp(key)` | `btnp(key)` | ✅ | キー押下検出 |
| `camera(x, y)` | `camera(x, y)` | ✅ | カメラ位置設定 |
| `zoom(scale)` | `zoom(scale)` | ✅ | ズーム設定 |
| `sfx(n, [ch, loop, vol])` | `sfx(n)` | ✅ | 効果音再生 |
| `music(n, [fade])` | `music(n)` | ✅ | BGM再生 |
| `stop([ch])` | `stop()` | ✅ | サウンド停止 |
| `print(text, x, y, col)` | `print(text, x, y, col)` | ❌ | テキスト描画（未実装） |
| `blt(dx, dy, sx, sy, w, h)` | - | ❌ | 画像転送（未実装） |
| `clip(x, y, w, h)` | - | ❌ | クリッピング（未実装） |
| `pal(c1, c2)` | - | ❌ | パレット置換（未実装） |

## 既存APIとの併用

既存のグローバルエンジン API は変更されません。必要に応じて共存させられます：

```rust
use nantaraquad::api::*;
use nantaraquad::api::framework::*;

// 既存API（そのまま）
let sprite_id = create_sprite(32, 32);
draw_sprite(sprite_id, 100.0, 100.0);

// Pyxel互換API（新規）
spr(sprite_id, 150.0, 150.0);

// 両方使用可能
cls(0);
rectfill(10, 10, 50, 30, 3);
```

## 今後の実装予定

HIGH優先度：
- `print()` - テキスト描画
- `clip()` - クリッピング領域
- `map()` - タイルマップシステム

MEDIUM優先度：
- `pal()` - パレット置換（視覚効果）
- `blt()` - 画像転送

## トラブルシューティング

### Q: エディタはどう使うのか？

A: エディタは別のゲームアプリケーション（`src/bin/editor.rs`）として実装されます。
Pyxel互換APIを使って開発されます。

### Q: パフォーマンスは？

A: Pyxel互換API は既存の実装をそのままラップしているため、
既存APIと同等のパフォーマンスを保有しています。

### Q: Pyxel ゲームを直接移植できるか？

A: ほぼ可能ですが、以下の差異があります：
- テキスト描画 (`print`) はまだ未実装
- パレット置換 (`pal`) は未実装
- マップシステム (`map`) は未実装

これらを除く基本機能（描画、入力、カメラ、オーディオ）は完全互換です。

