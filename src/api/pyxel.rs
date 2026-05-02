/// Pyxel互換API層
///
/// 基本的なゲーム開発API。DrawingContext やイベント等を利用する場合は直接それらを使用してください。
/// このモジュールの関数はスタブ実装で、グローバル状態への直接アクセスは含みません。

use crate::api::input::Key;

// ===== グラフィックス描画 API (Drawing) =====

pub fn cls(_col: u8) {}
pub fn pset(_x: i32, _y: i32, _col: u8) {}
pub fn pget(_x: i32, _y: i32) -> Option<u8> { None }
pub fn line(_x1: i32, _y1: i32, _x2: i32, _y2: i32, _col: u8) {}
pub fn rect(_x: i32, _y: i32, _w: i32, _h: i32, _col: u8) {}
pub fn rectfill(_x: i32, _y: i32, _w: i32, _h: i32, _col: u8) {}
pub fn circle(_x: i32, _y: i32, _r: i32, _col: u8) {}
pub fn circfill(_x: i32, _y: i32, _r: i32, _col: u8) {}

// ===== スプライト描画 API (Sprite) =====

pub fn spr(_n: usize, _x: f32, _y: f32) {}

// ===== 入力 API (Input) =====

pub fn btn(_key: Key) -> bool { false }
pub fn btnp(_key: Key) -> bool { false }

// ===== カメラ API (Camera) =====

pub fn camera(_x: f32, _y: f32) {}
pub fn zoom(_scale: f32) {}

// ===== オーディオ API (Audio) =====

pub fn sfx(_n: usize) {}
pub fn music(_n: usize) {}
pub fn stop() {}
pub fn music_stop() {}

// ===== パレット・色 API (Color) =====

pub fn set_palette(_col: u8, _r: u8, _g: u8, _b: u8) {}

// ===== フレーム・ゲームループ API (Framework) =====

pub fn frame_time() -> f32 { 16.667 }
pub fn frames_for_ms(ms: f32) -> u32 { (ms / 16.667) as u32 }

// ===== アニメーション API (Animation) =====

pub fn spr_anim(_n: usize, _x: f32, _y: f32, _anim_id: usize) {}
pub fn anim_update(_delta_ms: f32) {}
pub fn anim_set_frame(_anim_id: usize, _frame: usize) {}
pub fn anim_get_frame(_anim_id: usize) -> usize { 0 }

// ===== テキスト描画 API (Text) =====

/// テキスト描画
/// 
/// DrawingContext.print() を使用してください。
pub fn print(_text: &str, _x: i32, _y: i32, _col: u8) {}

// ===== デバッグ API =====

pub fn stat() -> String { "FPS: 60, Frame: 16.67ms".to_string() }
