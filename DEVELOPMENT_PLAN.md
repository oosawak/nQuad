# エンジニア引き継ぎ資料：Macroquadベース独自リソース管理システム

## 1. プロジェクト概要
Macroquadをコアエンジンとして採用し、Pyxelのような利便性を持つ「リソースエディタ一体型エンジン」を構築する。
最大の特徴は、**256色（インデックスカラー）とフルカラー（RGBA）を同一プロジェクト内で混在可能**にし、かつ実行中にリソースを動的編集できるAPIを提供することにある。

## 2. コア・コンセプト
 * **リソースの一元管理**: ID（インデックス番号）によるスプライト参照。
 * **ハイブリッド描画**: 256色パレットモードとフルカラーモードの共存。
 * **即時反映（Hot Reloading）**: エディタでのピクセル操作を即座にGPUテクスチャへ同期。
 * **独自バイナリ形式**: serdeを用いたアセットのパッキング。

## 3. 推奨データ構造（設計案）

### 現在の実装状況
`src/resource/data.rs` に基本構造が実装済み：

```rust
pub enum ColorMode {
    Indexed256(Vec<[u8; 4]>), // 256色のカラーパレットを保持
    FullColor,                  // RGBA直接指定
}

pub struct SpriteData {
    pub width: u32,
    pub height: u32,
    pub mode: ColorMode,
    pub pixels: Vec<u8>, // Indexed: 1byte/px, Full: 4bytes/px
}

pub struct ResourcePackage {
    pub sprites: Vec<SpriteData>,
    // 拡張予定：tilemaps, sounds, fonts
}
```

### 改善点（将来対応）
- `Macroquad::Color` 型への対応を検討（現在は `[u8; 4]` タプル）
- カラーモード設計は既に十分だが、型の整合性を確認

## 4. 拡張APIの実装サンプル（MVP）

エンジンの利用者が呼び出すAPIの最小実装例：

```rust
pub struct MyEngine {
    pub res: ResourcePackage,
    pub textures: Vec<Texture2D>,
}

impl MyEngine {
    /// 新規エンジンインスタンスの作成
    pub fn new() -> Self {
        Self { 
            res: ResourcePackage::new(), 
            textures: vec![] 
        }
    }

    /// ピクセルを書き換え、GPUに即時同期する（エディタ用API）
    pub fn set_pixel(&mut self, sprite_id: usize, x: u32, y: u32, value: &[u8]) -> Result<(), String> {
        let sprite = &mut self.res.sprites[sprite_id];
        sprite.set_pixel(x, y, value)?;
        
        // GPU側テクスチャの更新
        self.sync_texture(sprite_id);
        Ok(())
    }

    /// CPU上の画像データをMacroquadテクスチャに変換
    pub fn sync_texture(&mut self, id: usize) {
        let sprite = &self.res.sprites[id];
        let image = match &sprite.mode {
            ColorMode::FullColor => Image {
                width: sprite.width as u16,
                height: sprite.height as u16,
                bytes: sprite.pixels.clone(),
            },
            ColorMode::Indexed256(palette) => {
                // インデックスからRGBAへの変換
                let mut rgba = Vec::with_capacity(sprite.pixels.len() * 4);
                for &idx in &sprite.pixels {
                    let color = palette[idx as usize];
                    rgba.extend_from_slice(&color);
                }
                Image { 
                    width: sprite.width as u16, 
                    height: sprite.height as u16, 
                    bytes: rgba 
                }
            }
        };
        
        if id < self.textures.len() {
            self.textures[id] = Texture2D::from_image(&image);
        }
    }
}
```

## 5. 開発フェーズと優先事項

### フェーズ1：データと同期の基盤
**目標**: ピクセルを1つ変えたら即座に画面が変わる最小プロトタイプの完成

- [ ] macroquad クレートの統合（Cargo.toml に追加）
- [ ] `src/engine/` モジュールの作成
- [ ] `MyEngine` 構造体の実装
- [ ] `sync_texture()` による CPU→GPU 動的反映の実装
- [ ] ピクセル書き換え → テクスチャ更新 → 画面描画 のパイプライン構築
- [ ] テスト：256色パレットモード / フルカラーモードの両方で動作確認

### フェーズ2：拡張APIの提供
**目標**: ゲーム開発者向けの簡潔なAPI設計と実装

- [ ] `draw_sprite(sprite_id, x, y)` 関数の実装
- [ ] `draw_sprite_scaled(sprite_id, x, y, scale)` などのバリエーション実装
- [ ] グローバル STATE（シングルトン）による、どこからでも呼べるAPI設計
- [ ] リソース管理の便利メソッド（`load_resource()`, `save_resource()` など）
- [ ] 統合テスト：複数スプライトの描画、リソースの保存/読み込み

### フェーズ3：エディタの統合
**目標**: 実行中にリソースを編集・プレビューできるインタラクティブ環境

- [ ] `egui-macroquad` の統合
- [ ] エディタUIの基本フレーム（スプライト選択、パレット表示）
- [ ] ピクセルペイント機能（マウス操作で直接編集）
- [ ] パレットエディタ（256色モード用、色の追加・編集）
- [ ] リアルタイムプレビュー（編集中の即時反映）
- [ ] 編集内容の保存機能

### フェーズ4：最適化
**目標**: 描画性能の向上と機能の完成

- [ ] **パレット変換用フラグメントシェーダーの導入**
  - CPU での RGBA 変換を廃止
  - GPU でパレット参照を実施
  - パレット書き換えのみで全体色変更を実現
- [ ] メモリ使用量の最適化
- [ ] WebAssembly ターゲットでの動作確認
- [ ] パフォーマンス測定と最適化

## 6. 実装時の注意点

### メモリ管理
- フルカラー画像はメモリを消費するため、エディタでのプレビュー時は不要なクローンが発生しないよう注意
- `Cow<T>` や参照を活用して Copy-on-Write パターンの適用を検討

### Wasm対応
- MacroquadはWebAssemblyに強い
- リソースファイルはバイナリ形式で軽量に保つ必要がある
- `bincode` による圧縮効率を検証

### 拡張性
- 将来的にはタイルマップ（マリオのようなステージ）も ResourcePackage に含める設計
- `tilemaps`, `sounds`, `fonts` フィールドの追加を想定した設計になっている

## 7. 参考リンク
- [macroquad](https://docs.rs/macroquad/latest/macroquad/)
- [egui-macroquad](https://docs.rs/egui-macroquad/latest/egui_macroquad/)
- [serde](https://serde.rs/)
- [bincode](https://docs.rs/bincode/latest/bincode/)
