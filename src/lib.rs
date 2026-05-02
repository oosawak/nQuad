//! # Nantaraquad: Macroquad スプライト・リソース管理エンジン
//!
//! Nantaraquad は、Macroquad 上で動作するハイブリッドカラーモード対応の
//! スプライト・リソース管理ライブラリです。
//! 256色パレット（Indexed256）とフルカラー（RGBA）を同一プロジェクト内で混在させることができます。
//!
//! ## 主な特徴
//!
//! - **ハイブリッドカラーモード**: Indexed256 と FullColor の両方をサポート
//! - **自動GPU同期**: CPU上のピクセル編集が自動的にGPUテクスチャに反映
//! - **シンプルなAPI**: グローバル関数で直感的に操作可能
//! - **ディスク保存**: bincode形式でリソースをシリアライズ・デシリアライズ
//! - **フレームワーク統合**: Macroquad の高速描画機構を活用
//!
//! ## アーキテクチャ概要
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  グローバル API                      │
//! │  (draw_sprite, set_pixel等)         │
//! └──────────────┬──────────────────────┘
//!                │ 委譲
//! ┌──────────────▼──────────────────────┐
//! │  Engine                             │
//! │  - スプライト・テクスチャ管理        │
//! │  - GPU同期 (sync_texture)           │
//! │  - 描画実装                         │
//! └──────────────┬──────────────────────┘
//!                │
//! ┌──────────────▼──────────────────────┐
//! │  Mutex<Engine> (グローバル状態)     │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## クイックスタート
//!
//! ### グローバル API を使用（推奨）
//!
//! ```ignore
//! use macroquad::prelude::*;
//! use nantaraquad::*;
//!
//! #[macroquad::main("My Game")]
//! async fn main() {
//!     // FullColor スプライトを作成
//!     let sprite_id = create_sprite(32, 32);
//!     
//!     // ピクセル編集（自動GPU同期）
//!     set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?;
//!     
//!     loop {
//!         clear_background(BLACK);
//!         draw_sprite(sprite_id, 100.0, 100.0);
//!         next_frame().await;
//!     }
//! }
//! ```
//!
//! ### インスタンス API を使用（カスタマイズ用）
//!
//! ```ignore
//! let mut engine = Engine::new();
//! let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
//! let sprite_id = engine.add_sprite(sprite);
//! engine.set_pixel(sprite_id, 0, 0, &[255, 0, 0, 255])?;
//! ```
//!
//! ## モジュール構成
//!
//! | モジュール | 説明 |
//! |----------|------|
//! | [`engine`] | スプライト・テクスチャ管理、描画実装。Engine 構造体が中心。 |
//! | [`resource`] | スプライトデータ、カラーモード、パッケージ、シリアライゼーション。 |
//! | [`render`] | GPU同期処理（テクスチャ化）。内部用途。 |
//! | [`api`] | グローバルAPI関数。ユーザーが主に使用。 |
//! | [`core`] | グローバルエンジン状態管理。内部用途。 |
//! | [`editor`] | エディタ機能プレースホルダー。Phase 3 で実装予定。 |
//!
//! ## API リファレンス
//!
//! ### スプライト作成
//! - [`create_sprite`] — FullColor スプライト作成
//! - [`create_indexed_sprite`] — Indexed256 スプライト作成
//! - [`add_sprite`] — スプライト直接追加
//!
//! ### ピクセル操作
//! - [`set_pixel`] — ピクセル設定（自動同期）
//! - [`get_pixel`] — ピクセル読み取り
//!
//! ### 描画
//! - [`draw_sprite`] — 基本描画
//! - [`draw_sprite_scaled`] — スケール描画
//!
//! ### リソース管理
//! - [`load_sprite`] — ディスク読み込み
//! - [`sprite_count`] — スプライト数取得
//!
//! ## 実装例
//!
//! 詳細な使用例は `examples/` ディレクトリを参照：
//! - `examples/basic_sprite.rs` — FullColor スプライト基本
//! - `examples/indexed_palette.rs` — Indexed256 パレット操作
//! - `examples/save_load_sprite.rs` — ディスク I/O とリアルタイム編集
//!
//! 実行：
//! ```bash
//! cargo run --example basic_sprite
//! cargo run --example indexed_palette
//! cargo run --example save_load_sprite
//! ```
//!
//! ## 設計原則
//!
//! ### 単一実装原則
//! すべての機能は [`Engine`] に統一実装されます。
//! グローバル API 関数は単なる委譲で、実装の重複を排除します。
//!
//! ### 自動GPU同期
//! `set_pixel()` と `add_sprite()` は自動的に GPU テクスチャを同期させます。
//! 手動同期は不要です。
//!
//! ### 安全なパレットアクセス
//! Indexed256 でパレット範囲外のインデックスが指定された場合、
//! 透明黒 `[0, 0, 0, 0]` にフォールバックします。パニックはありません。
//!
//! ### ピクセルパーフェクト描画
//! テクスチャフィルタは `FilterMode::Nearest` に自動設定され、
//! ドット絵のような見た目を保証します。
//!
//! ## 開発フェーズ
//!
//! - ✅ **Phase 1** — リソース構造・シリアライゼーション
//! - ✅ **Phase 2** — グローバルAPI・テクスチャ同期・アーキテクチャ統一
//! - 🚧 **Phase 3** — エディタ統合（egui-macroquad）、ピクセルペイント、プレビュー
//! - 📅 **Phase 4** — シェーダー最適化、Wasm対応
//!
//! ## 技術スタック
//!
//! - **Rust** 1.70+
//! - **Macroquad** 0.4 — ゲームフレームワーク
//! - **Serde + Bincode** — シリアライゼーション
//! - **Lazy Static** — グローバル状態管理

pub mod api;
pub mod audio;
pub mod core;
pub mod editor;
pub mod engine;
pub mod math;
pub mod nquad_api;
pub mod platform;
pub mod render;
pub mod resource; // nQuad 統一 API

// Engine (インスタンス用、テスト用)
pub use engine::{Engine, PlaybackState, SpriteAnimator};

// オーディオ
pub use audio::{AudioManager, SoundBank};

// グローバル状態
pub use core::state::{get_engine, ENGINE};

// 3D数学（マリオ64用）
pub use math::{Vec3, IsometricProjector, IsoCamera};

// グローバル API (メイン API)
pub use api::{
    add_sprite, create_indexed_sprite, create_sprite, draw_sprite, draw_sprite_scaled, get_pixel,
    load_sprite, set_pixel, sprite_count, GameEngine, PyxelAudio, build_api_reference, ApiReference, ApiFunction,
};

// リソース型
pub use resource::{ColorMode, ResourcePackage, SpriteData};

// ユーティリティ
pub use render::sync::sync_texture_from_sprite;

// ===== nQuad API（統一命名） =====
pub mod nquad {
    //! nQuad ゲームエンジン・エディタ統一 API
    //! 短縮名称 "nQuad" (nQ) で明確なインターフェース
    pub use crate::nquad_api::*;
}

// スタイル別のエクスポート
pub use nquad::{
    colors,
    nQAnimationClip,
    nQAnimationController,
    nQBlendMode,
    nQColor,
    // 型エイリアス
    nQColorMode,
    nQDocument,
    nQDocumentId,
    nQDrawParams,
    nQEditCommand,
    nQEditCommandHistory,
    nQFrame,
    // エディタ型
    nQLayer,
    nQLayerId,
    nQLayerStack,
    nQMouseButton,
    nQPlaybackState,
    nQSpriteData,
    nQSpriteId,
    // ヘルパー
    nq_color,
    nq_color_rgba,
};
