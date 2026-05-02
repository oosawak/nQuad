# Nantaraquad API Reference - Pyxel互換

## 概要
Nantaraquad はゲームエンジンで、Pyxel の API に似た形で設計されています。
スプライト管理、グラフィックス、入力、オーディオ、アニメーション機能を提供します。

---

## API 分類

### 1. グラフィックス描画 (Drawing API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `cls(color)` | ✅ | ✅ | 画面クリア |
| `pset(x, y, col)` | ✅ | ✅ | ピクセル描画 |
| `pget(x, y)` | ✅ | ✅ | ピクセル読み取り |
| `line(x1, y1, x2, y2, col)` | ✅ | ✅ | 線描画 |
| `rect(x, y, w, h, col)` | ✅ | ✅ | 矩形枠 |
| `rectfill(x, y, w, h, col)` | ✅ | ✅ | 矩形塗りつぶし |
| `circle(x, y, r, col)` | ✅ | ✅ | 円枠 |
| `circfill(x, y, r, col)` | ✅ | ✅ | 円塗りつぶし |
| `blt(dx, dy, sx, sy, w, h)` | ✅ | ❌ | **未実装** - 画像転送 |
| `clip(x, y, w, h)` | ✅ | ❌ | **未実装** - クリッピング領域設定 |

### 2. スプライト管理 (Sprite API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `spr(id, x, y, col)` | ✅ | ✅ | スプライト描画 |
| `map(x, y, w, h, layer)` | ✅ | ❌ | **未実装** - マップ描画 |
| `create_sprite(w, h)` | ✅ | ✅ | スプライト作成 |
| `load_sprite(path)` | ✅ | ✅ | スプライト読み込み |
| `sprite_count()` | ✅ | ✅ | スプライト数取得 |

### 3. 入力処理 (Input API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `btn(key)` | ✅ | ✅ | キー押下状態 |
| `btnp(key)` | ✅ | ✅ | キー押下（フレーム内一度） |
| `btnr(key)` | ✅ | ❌ | **未実装** - キー解放（フレーム内一度） |

### 4. オーディオ (Audio API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `sfx(id, [ch, loop, vol])` | ✅ | ✅ | 効果音再生 |
| `music(id, [fade_ms])` | ✅ | ✅ | BGM 再生 |
| `stop([ch])` | ✅ | ✅ | サウンド停止 |
| `music_stop()` | ✅ | ✅ | BGM 停止 |

### 5. アニメーション (Animation API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `spr_anim(id, x, y, anim_id)` | ❌ | ✅ | アニメーション再生スプライト描画 |
| `anim_update(delta_ms)` | ❌ | ✅ | アニメーション更新 |
| `anim_set_frame(id, frame)` | ❌ | ✅ | アニメーションフレーム設定 |
| `anim_get_frame(id)` | ❌ | ✅ | アニメーション現在フレーム取得 |

### 6. カメラ・ビューポート (Camera API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `camera(x, y)` | ✅ | ✅ | カメラ位置設定 |
| `camera_follow(target_x, target_y, speed)` | ❌ | ✅ | カメラターゲット追従 |
| `zoom(level)` | ❌ | ✅ | ズーム設定 |
| `world_to_screen(wx, wy)` | ❌ | ✅ | ワールド→スクリーン座標変換 |
| `screen_to_world(sx, sy)` | ❌ | ✅ | スクリーン→ワールド座標変換 |

### 7. パレット・色 (Color API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `pal(c1, c2)` | ✅ | ❌ | **未実装** - パレット置換 |
| `palt(col, t)` | ✅ | ❌ | **未実装** - 透明色設定 |
| `set_palette(index, r, g, b, a)` | ✅ | ✅ | パレットカラー設定 |

### 8. ゲームループ・フレーム (Framework API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `init()` | ✅ | ✅ | 初期化（自動） |
| `run()` | ✅ | ✅ | ゲームループ実行 |
| `quit()` | ✅ | ❌ | **未実装** - ゲーム終了 |
| `frame_time()` | ✅ | ✅ | フレーム時間取得 |
| `tick()` | ✅ | ❌ | **未実装** - 時刻取得 |

### 9. デバッグ・ユーティリティ (Debug API)

| API | Pyxel | Nantaraquad | 説明 |
|-----|-------|-------------|------|
| `print(text, x, y, col)` | ✅ | ❌ | **未実装** - テキスト描画 |
| `stat()` | ❌ | ❌ | **未実装** - 統計情報 |

---

## 実装状況サマリー

### 実装済み (16 API)
✅ Drawing: cls, pset, pget, line, rect, rectfill, circle, circfill
✅ Sprite: spr, create_sprite, load_sprite, sprite_count
✅ Input: btn, btnp
✅ Audio: sfx, music
✅ Animation: 基本フレーム管理
✅ Camera: 基本機能
✅ Framework: init, run, frame_time

### 未実装 (10 API)
❌ blt, clip (描画関数)
❌ map (マップシステム)
❌ btnr (入力)
❌ pal, palt (パレット置換)
❌ quit, tick (フレーム管理)
❌ print (テキスト)
❌ stat (デバッグ)

---

## 推奨実装順序

### 優先度 HIGH (ゲーム制作に必須)
1. `print(text, x, y, col)` - デバッグ・UI テキスト表示
2. `clip(x, y, w, h)` - 描画領域制限
3. `map(x, y, w, h, layer)` - タイルマップシステム
4. `btnr(key)` - キー解放イベント

### 優先度 MEDIUM (ゲーム制作便利)
5. `pal(c1, c2)` - パレット置換（視覚効果用）
6. `palt(col, t)` - 透明色設定
7. `tick()` - 経過時間取得（デバッグ用）

### 優先度 LOW (オプション)
8. `stat()` - 統計情報（パフォーマンス分析）
9. `quit()` - 終了制御
10. `blt(dx, dy, sx, sy, w, h)` - 低レベル画像操作

---

## 拡張 API（Pyxel にはない）

Nantaraquad のゲームエンジン独自機能：

### アニメーションシステム
- `spr_anim(id, x, y, anim_id)` - アニメーション再生
- `anim_update(delta_ms)` - フレーム更新
- `anim_set_frame(id, frame)` - フレーム設定
- `anim_get_frame(id)` - 現在フレーム取得

### カメラシステム
- `camera_follow(target, speed)` - スムーズ追従
- `zoom(level)` - ズーム
- `world_to_screen()` - 座標変換

### ゲーム構造
- `GameApp` trait - ゲーム実装インターフェース
- `GameRunner` - ゲームループ管理
- `Scene` - マルチエンティティ管理

---

## 使用例

### 基本的なゲーム

```rust
use nantaraquad::api::framework::{GameApp, GameRunner};

struct MyGame {
    x: i32,
    y: i32,
}

impl GameApp for MyGame {
    fn update(&mut self, _delta_ms: f32) {
        if input::btn(Key::Up) { self.y -= 1; }
        if input::btn(Key::Down) { self.y += 1; }
    }
    
    fn draw(&mut self) {
        drawing::cls(0); // 黒でクリア
        drawing::rectfill(self.x, self.y, 8, 8, 3); // 黄色矩形
    }
}

fn main() {
    let game = MyGame { x: 100, y: 100 };
    let mut runner = GameRunner::new(game, "My Game", 256, 256, 60);
    runner.run();
}
```

---

## 次のアクション

1. **HIGH 優先度 API の実装**
   - `print()` - テキスト描画
   - `clip()` - クリッピング領域
   - `map()` - タイルマップ

2. **エディタアプリケーション開発**
   - egui-macroquad UI 統合
   - レイヤーパネル実装
   - タイムラインUI 実装

3. **デモゲーム拡張**
   - Lineboy/Cubeboy に新機能追加
   - パフォーマンステスト

