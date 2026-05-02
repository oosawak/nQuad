//! エディタ UI、ピクセルペイント、Undo/Redo、ファイル I/O のデモ
//!
//! このサンプルが示すこと：
//! - egui UI フレームワークの統合
//! - マウスでリアルタイムピクセル描画
//! - キャンバス操作（ズーム、パン）
//! - Ctrl+Z/Y による Undo/Redo
//! - Ctrl+S/O によるファイル保存・読み込み
//! - キーボードショートカット
//!
//! # 実行方法
//! ```bash
//! cargo run --example editor
//! ```
//!
//! # キーボード操作
//! - Ctrl+S: 保存（/tmp/editor_sprite.bin）
//! - Ctrl+O: 読み込み
//! - Ctrl+Z: アンドゥ
//! - Ctrl+Y: リドゥ
//! - +/-: ズーム
//! - WASD: パン
//! - ESC: 終了

use macroquad::prelude::*;
use nantaraquad::*;

#[macroquad::main("Nantaraquad Editor")]
async fn main() {
    // サンプルスプライト作成
    let sprite_id = create_sprite(64, 64);

    // グラデーション背景を描画（初期化用）
    for y in 0..64 {
        for x in 0..64 {
            let r = (x * 4) as u8;
            let g = (y * 4) as u8;
            let b = 200u8;
            set_pixel(sprite_id, x as u32, y as u32, &[r, g, b, 255]).ok();
        }
    }

    // エディタ状態
    let mut editor_state = nantaraquad::editor::EditorState::new();
    editor_state.set_sprite(sprite_id, ColorMode::FullColor);

    // UI・ペイント・入力マネージャー
    let mut editor_ui = nantaraquad::editor::EditorUI::new();
    let mut paint_tool = nantaraquad::editor::PaintTool::new();
    let mut input = nantaraquad::editor::InputManager::new();
    let preview = nantaraquad::editor::SpritePreview;

    // ファイル I/O と Undo/Redo
    let mut file_manager = nantaraquad::editor::FileManager::new();
    let mut history = nantaraquad::editor::UndoRedoStack::new();

    // メインループ
    loop {
        // キー状態更新
        input.update();

        clear_background(BLACK);

        // File I/O
        if input.save_pressed() {
            if let Some(ref sprite_data) = get_sprite_data(sprite_id) {
                match file_manager.save_sprite("/tmp/editor_sprite.bin", sprite_data) {
                    Ok(path) => println!("Saved: {:?}", path),
                    Err(e) => eprintln!("Save error: {}", e),
                }
            }
        }

        if input.open_pressed() {
            match file_manager.load_sprites("/tmp/editor_sprite.bin") {
                Ok(sprites) => {
                    if !sprites.is_empty() {
                        let loaded_id = add_sprite(sprites[0].clone());
                        editor_state.set_sprite(loaded_id, sprites[0].mode.clone());
                        history.clear();
                        println!("Loaded sprite #{}", loaded_id);
                    }
                }
                Err(e) => eprintln!("Load error: {}", e),
            }
        }

        // Undo/Redo
        if input.undo_pressed() {
            if let Some(change) = history.undo() {
                let _ = set_pixel(change.sprite_id, change.x, change.y, &change.old_color);
            }
        }

        if input.redo_pressed() {
            if let Some(change) = history.redo() {
                let _ = set_pixel(change.sprite_id, change.x, change.y, &change.new_color);
            }
        }

        // egui UI フレーム
        editor_ui.frame(&mut editor_state);

        // ペイント処理（Undo/Redo 記録付き）
        paint_tool.update(&editor_state, &mut history);

        // ズーム
        if input.zoom_in_pressed() {
            editor_state.zoom_in();
        }
        if input.zoom_out_pressed() {
            editor_state.zoom_out();
        }

        // パン
        let pan_speed = 10.0;
        if input.pan_up() {
            editor_state.pan_y += pan_speed;
        }
        if input.pan_down() {
            editor_state.pan_y -= pan_speed;
        }
        if input.pan_left() {
            editor_state.pan_x += pan_speed;
        }
        if input.pan_right() {
            editor_state.pan_x -= pan_speed;
        }

        // キャンバス描画
        preview.draw_canvas(&editor_state);
        preview.draw_cursor_preview(&editor_state);
        preview.draw_info(&editor_state);

        // 状態表示（右上）
        let mut text_y = 10.0;
        draw_text(
            &format!(
                "Undo: {} | Redo: {}",
                history.undo_count(),
                history.redo_count()
            ),
            screen_width() - 250.0,
            text_y,
            16.0,
            GRAY,
        );
        text_y += 25.0;

        if let Some(filename) = file_manager.current_filename() {
            draw_text(
                &format!("File: {}", filename),
                screen_width() - 250.0,
                text_y,
                16.0,
                GRAY,
            );
        }

        // 終了
        if input.escape_pressed() {
            break;
        }

        next_frame().await;
    }
}

/// スプライトデータを取得するヘルパー関数
fn get_sprite_data(sprite_id: usize) -> Option<SpriteData> {
    let engine = get_engine();
    engine.res.get_sprite(sprite_id).cloned()
}
