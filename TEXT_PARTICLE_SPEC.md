# Nantaraquad テキスト・パーティクルシステム仕様書

## テキスト・フォント周り

### 現状

#### 1. Pyxel互換 print() API
```rust
pub fn print(_text: &str, _x: i32, _y: i32, _col: u8) {
    // TODO: テキストレンダリング実装
}
```

**状態**: ✅ 関数シグネチャ定義済み、❌ 実装未完了

#### 2. エディタでのテキスト表示
```rust
// src/editor/preview.rs
use macroquad::prelude::draw_text;

draw_text("Nantaraquad Editor", 10.0, text_y, 20.0, WHITE);
draw_text(&format!("Sprite: #{}", sprite_id), 10.0, text_y, 16.0, GRAY);
```

- **使用フレームワーク**: macroquad の `draw_text()`
- **フォント**: デフォルトシステムフォント（macroquad組み込み）
- **サイズ**: 可変（20.0, 16.0など）
- **色**: macroquad Color型（RGB）

#### 3. ゲームでのテキスト表示
**Lineboy/Cubeboy**: 
- テキスト描画なし（メモリ・WASM サイズ制約）
- UI は数値・図形のみ

### 実装計画

#### SHORT TERM（推奨）

##### Option A: macroquad `draw_text()` をラップ
```rust
// src/api/drawing.rs に追加
pub fn print(text: &str, x: i32, y: i32, col: u8) {
    use macroquad::prelude::{draw_text, Color};
    
    // パレットカラーから RGBA に変換
    let color = palette_to_color(col);
    
    // macroquad で描画
    draw_text(text, x as f32, y as f32, 16.0, color);
}

fn palette_to_color(col: u8) -> Color {
    let rgba = PYXEL_PALETTE[col as usize];
    Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3])
}
```

**メリット**:
- 実装が簡単
- WASM 対応（macroquad WASM にはテキスト機能がある）
- ゲーム・エディタで統一的に使用可能

**デメリット**:
- フォント変更不可（デフォルトのみ）
- WASM バイナリサイズ増加
- 日本語フォント非対応

##### Option B: ドット絵フォントをスプライトとして実装
```rust
struct BitMapFont {
    sprite_id: usize,           // フォントスプライト
    char_width: u32,            // 文字幅（通常 4-8px）
    char_height: u32,           // 文字高さ（通常 8px）
    chars_per_row: u32,         // 1行の文字数
}

impl BitMapFont {
    pub fn draw_text(&self, text: &str, x: i32, y: i32, col: u8) {
        for (i, ch) in text.chars().enumerate() {
            let char_index = ch as u32;
            let src_x = (char_index % self.chars_per_row) * self.char_width;
            let src_y = (char_index / self.chars_per_row) * self.char_height;
            
            let dst_x = x + (i as i32 * self.char_width as i32);
            let dst_y = y;
            
            // スプライト内の領域をコピー
            draw_sprite_region(
                self.sprite_id,
                dst_x as f32, dst_y as f32,
                src_x, src_y,
                self.char_width, self.char_height
            );
        }
    }
}
```

**メリット**:
- WASM バイナリサイズ小さい
- 完全なピクセルアート制御
- 複数フォント対応
- 日本語対応可能

**デメリット**:
- 実装複雑
- フォント作成が別途必要
- パフォーマンス注意（複数 draw_sprite 呼び出し）

#### MEDIUM TERM

##### フォント管理システム
```rust
pub struct FontManager {
    fonts: HashMap<String, BitMapFont>,
    default_font: String,
}

impl FontManager {
    pub fn load_font(&mut self, name: &str, sprite_id: usize, char_width: u32) {
        self.fonts.insert(name.to_string(), BitMapFont {
            sprite_id,
            char_width,
            char_height: 8,
            chars_per_row: 16,
        });
    }
    
    pub fn print(&self, font: &str, text: &str, x: i32, y: i32) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text(text, x, y, 0);
        }
    }
}
```

#### LONG TERM

- [ ] True Type フォント対応
- [ ] 複数言語対応（CJK）
- [ ] テキストレイアウトエンジン
- [ ] 行折り返し・テキストボックス

---

## パーティクルシステム

### 現状

#### 1. Cubeboy の実装
```rust
struct Particle {
    x: f32,
    y: f32,
    dx: f32,      // 速度X
    dy: f32,      // 速度Y
    color: u8,    // パレットインデックス
    life: u32,    // 残りライフ（フレーム数）
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32, color: u8) -> Self {
        Particle {
            x, y, dx, dy, color,
            life: PARTICLE_LIFETIME_FRAMES,  // 30フレーム
        }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy += 2.0;               // 重力
        self.life = self.life.saturating_sub(1);
    }

    fn is_alive(&self) -> bool {
        self.life > 0
    }
}
```

**特徴**:
- ✅ 物理演算（重力、速度）
- ✅ ライフタイム管理
- ✅ パレットカラー対応
- ✅ 描画は pset() で 1x1 ピクセル

#### 2. 使用例
```rust
// ダッシュ時にパーティクル生成
particles.push(Particle::new(
    player.x + 3.0,    // パーティクル位置
    player.y + 3.0,
    -2.0 - rand::gen_range(0.0, 2.0),  // 速度（後ろに噴き出す）
    rand::gen_range(-1.0, 1.0),
    10,                // 色（ライム色）
));

// ゲームループ内
particles.retain_mut(|p| {
    p.update();
    p.is_alive()
});

// 描画
for particle in &particles {
    ctx.pset(particle.x as i32, particle.y as i32, particle.color);
}
```

### 実装詳細

#### パーティクル種類別

| 種類 | 用途 | 生成条件 | 数量 | ライフ | 色 |
|------|------|--------|------|--------|-----|
| ダッシュ | 移動エフェクト | ダッシュ時 | 2-5個 | 30フレーム | ライム(11) |
| ジャンプ | 離地エフェクト | 地面離脱時 | 1-2個 | 20フレーム | 白(7) |
| 衝突 | 衝撃エフェクト | 敵衝突時 | 3-4個 | 15フレーム | 赤(8) |

#### パフォーマンス

**Cubeboy 検測値**:
- 最大パーティクル数: 50個/フレーム
- CPU使用率: < 2%
- メモリ: ~8KB（Vec で動的確保）

### 実装計画

#### SHORT TERM

##### パーティクルシステム汎用化
```rust
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(capacity: usize) -> Self {
        Self {
            particles: Vec::new(),
            max_particles: capacity,
        }
    }
    
    pub fn emit(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: u8) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle::new(x, y, dx, dy, color));
        }
    }
    
    pub fn update(&mut self) {
        self.particles.retain_mut(|p| {
            p.update();
            p.is_alive()
        });
    }
    
    pub fn draw(&self, ctx: &mut DrawingContext) {
        for p in &self.particles {
            ctx.pset(p.x as i32, p.y as i32, p.color);
        }
    }
}
```

##### グローバル統合
```rust
// src/api/game.rs に追加
pub struct GameEngine {
    // ...
    pub particles: ParticleSystem,
}

// Pyxel互換API
pub fn emit_particle(x: f32, y: f32, dx: f32, dy: f32, color: u8) {
    let mut engine = get_engine();
    engine.particles.emit(x, y, dx, dy, color);
}
```

#### MEDIUM TERM

##### パーティクル拡張
- [ ] スケール変化（ライフで縮小）
- [ ] 回転（角度ベース）
- [ ] スプライトベースパーティクル（1x1 以外）
- [ ] ブレンドモード（加算合成など）
- [ ] フェード（透明度変化）

##### パーティクルエミッター
```rust
pub struct ParticleEmitter {
    x: f32, y: f32,
    interval: u32,              // 生成間隔（フレーム）
    count: u32,                 // 1回の生成数
    lifetime: u32,              // ライフタイム
}

impl ParticleEmitter {
    pub fn update(&mut self, system: &mut ParticleSystem) {
        // interval フレームごとに count 個生成
        for _ in 0..self.count {
            system.emit(
                self.x + rand::gen_range(-2.0, 2.0),
                self.y + rand::gen_range(-2.0, 2.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(0, 16) as u8,
            );
        }
    }
}
```

#### LONG TERM

- [ ] GPU パーティクル（WASM WebGL）
- [ ] 物理ベースパーティクル（衝突判定）
- [ ] 粒子フィルター（煙、火、水）
- [ ] パーティクルプール最適化
- [ ] データドリブン定義（JSON/YAML）

---

## 統合実装例

### ゲーム内での使用

```rust
use nantaraquad::api::*;

struct MyGame {
    particles: ParticleSystem,
}

impl GameApp for MyGame {
    fn update(&mut self, input: &InputState, dt: f32) {
        // パーティクル更新
        self.particles.update();
        
        // テキスト表示とパーティクル生成
        if btnp(Key::Space) {
            print("Dash!", 100, 50, 7);
            
            for i in 0..5 {
                let angle = (i as f32) * 2.0 * 3.14159 / 5.0;
                let dx = angle.cos() * 2.0;
                let dy = angle.sin() * 2.0;
                self.particles.emit(100.0, 100.0, dx, dy, 11);
            }
        }
    }
    
    fn draw(&mut self, ctx: &mut DrawingContext) {
        ctx.cls(0);
        
        // ゲーム描画
        ctx.rectfill(50, 50, 50, 50, 3);
        
        // パーティクル描画
        self.particles.draw(ctx);
        
        // テキスト描画
        print("Score: 1000", 10, 10, 7);
    }
}
```

---

## 制限事項

### テキスト
- ❌ 複数フォント非対応（macroquad 使用時）
- ❌ 日本語非対応（デフォルトフォント）
- ❌ テキストボックス・行折り返し
- ⚠️ WASM でバイナリサイズ増加

### パーティクル
- ⚠️ 単色パーティクルのみ（スプライトベース未対応）
- ⚠️ 最大数制限あり（メモリ効率のため）
- ⚠️ 物理演算基本的（重力のみ）
- ❌ マルチパーティクル物理衝突なし

---

## 参考実装

### 関連ファイル
- `examples/cubeboy.rs` - パーティクル実装例
- `src/api/pyxel.rs` - print() API シグネチャ
- `src/editor/preview.rs` - macroquad draw_text() 使用例

