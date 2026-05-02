# Nantaraquad

Macroquad ベースの、グラフィックス・スプライトリソース管理エンジン。256色パレットモードとフルカラー（RGBA）の両方に対応し、実行中にスプライトを動的に編集できます。

## 特徴

- **ハイブリッドカラーモード**: 256色パレット（Indexed256）とフルカラー（RGBA）を同一プロジェクト内で混在可能
- **自動GPU同期**: CPU上のピクセル編集が自動的にGPUテクスチャに反映
- **シンプルなAPI**: グローバル関数で簡単に操作、またはインスタンス化してカスタマイズも可能
- **バイナリシリアライゼーション**: bincode形式でリソースをディスク保存・読み込み
- **Macroquad統合**: レイアウトはシンプル、描画はMacroquadの高速性を活用

## セットアップ

### Cargo.toml に追加

```toml
[dependencies]
nantaraquad = "0.1"
macroquad = "0.4"
```

### 最小限の例

```rust
use macroquad::prelude::*;
use nantaraquad::*;

#[macroquad::main("My Game")]
async fn main() {
    // FullColor スプライト (32x32) を作成
    let sprite_id = create_sprite(32, 32);
    
    // ピクセルを赤色に設定
    set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255]).ok();
    
    // メインループ
    loop {
        clear_background(BLACK);
        
        // スプライトを描画
        draw_sprite(sprite_id, 100.0, 100.0);
        
        next_frame().await;
    }
}
```

## 使用方法

### 1. スプライトの作成

#### FullColor モード（フルカラー）
```rust
let sprite_id = create_sprite(32, 32);
set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?; // RGBA
```

#### Indexed256 モード（256色パレット）
```rust
let palette = vec![
    [0, 0, 0, 255],       // インデックス 0: 黒
    [255, 0, 0, 255],     // インデックス 1: 赤
    [0, 255, 0, 255],     // インデックス 2: 緑
];
let sprite_id = create_indexed_sprite(32, 32, palette);
set_pixel(sprite_id, 0, 0, &[1])?; // パレットインデックス
```

### 2. ピクセル編集

```rust
// ピクセルを設定（自動GPU同期）
set_pixel(sprite_id, 10, 10, &[255, 128, 64, 255])?;

// ピクセルを読み取る
if let Some(color) = get_pixel(sprite_id, 10, 10) {
    println!("Pixel: {:?}", color);
}
```

### 3. 描画

```rust
// 基本的な描画
draw_sprite(sprite_id, 100.0, 100.0);

// スケール付き描画
draw_sprite_scaled(sprite_id, 100.0, 100.0, 2.0);
```

### 4. ディスク操作

```rust
use nantaraquad::*;
use std::path::Path;

// リソースパッケージとして保存
let package = ResourcePackage::new(); // または既存パッケージ
resource::serialize::save_package(&package, Path::new("sprites.bin"))?;

// ディスクから読み込み
let sprite_id = load_sprite("sprites.bin")?;
```

## アーキテクチャ

### 3層構造

```
┌─────────────────────────────────┐
│  グローバル API (api/mod.rs)     │
│  draw_sprite, set_pixel, etc...  │
└──────────────┬──────────────────┘
               │ (薄い委譲)
┌──────────────▼──────────────────┐
│   Engine (engine/engine.rs)      │
│  + add_sprite()                  │
│  + set_pixel() → auto sync       │
│  + draw_sprite()                 │
└──────────────┬──────────────────┘
               │
┌──────────────▼──────────────────┐
│ Mutex<Engine> (core/state.rs)    │
│ グローバル状態として管理          │
└─────────────────────────────────┘
```

### モジュール構成

| モジュール | 役割 |
|----------|------|
| `engine` | スプライト・テクスチャ管理、描画実装 |
| `resource` | スプライトデータ・リソースパッケージ・シリアライゼーション |
| `render` | GPU同期（テクスチャ化） |
| `api` | グローバルAPI関数（薄い委譲） |
| `core::state` | グローバルエンジン状態管理 |

## API リファレンス

### スプライト作成
- `create_sprite(width, height) -> usize` — FullColor スプライト作成
- `create_indexed_sprite(width, height, palette) -> usize` — Indexed256 スプライト作成
- `add_sprite(sprite: SpriteData) -> usize` — スプライト追加（自動GPU同期）

### ピクセル操作
- `set_pixel(sprite_id, x, y, pixel_data) -> Result` — ピクセル設定（自動GPU同期）
- `get_pixel(sprite_id, x, y) -> Option<Vec<u8>>` — ピクセル読み取り

### 描画
- `draw_sprite(id, x, y)` — 基本描画
- `draw_sprite_scaled(id, x, y, scale)` — スケール描画

### リソース管理
- `load_sprite(path) -> Result<usize>` — ディスク読み込み
- `sprite_count() -> usize` — スプライト数取得

### インスタンスAPI
```rust
let mut engine = Engine::new();
let sprite_id = engine.add_sprite(sprite);
engine.set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?;
engine.draw_sprite(sprite_id, 100.0, 100.0)?;
```

## Examples

### basic_sprite
フルカラースプライトの基本的な使用例

```bash
cargo run --example basic_sprite
```

### indexed_palette
256色パレットモードの例（チェッカーボードパターン）

```bash
cargo run --example indexed_palette
```

### save_load_sprite
ディスク保存・読み込み、リアルタイムピクセル編集の例

```bash
cargo run --example save_load_sprite
```

## 開発フェーズ

- **Phase 1** ✅ — リソース構造・シリアライゼーション
- **Phase 2** ✅ — グローバルAPI・テクスチャ同期・アーキテクチャ統一
- **Phase 3** 🚧 — エディタ統合（egui-macroquad）、ピクセルペイント、プレビュー
- **Phase 4** 📅 — シェーダー最適化、Wasm対応

## 技術スタック

- **Rust** 1.70+
- **Macroquad** 0.4 — ゲームフレームワーク
- **Serde + Bincode** — シリアライゼーション
- **Lazy Static** — グローバル状態管理

## ライセンス

MIT

## 貢献

プルリクエストを歓迎します。大きな変更の場合は、まずissueを開いて変更内容を議論してください。

---

**開発者向け:**
- `cargo doc --open` でAPI ドキュメントを生成・表示
- `cargo test --lib` でユニットテスト実行
- `cargo build --examples` で examples をコンパイル
# nQuad
