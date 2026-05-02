//! egui ベースの UI
//!
//! メニューバー、ツールパレット、プロパティパネル、キャンバスを描画します。

use super::state::EditorState;
use crate::resource::ColorMode;
use egui_macroquad::egui;

/// egui UI マネージャー
pub struct EditorUI {
    /// 色選択ダイアログが開いているか
    pub color_picker_open: bool,
    /// ブラシサイズスライダー値
    pub brush_size_slider: f32,
    /// ブラシカラー（egui Color32 で管理）
    pub brush_color_egui: egui::Color32,
}

impl EditorUI {
    /// 新規 UI を作成
    pub fn new() -> Self {
        Self {
            color_picker_open: false,
            brush_size_slider: 1.0,
            brush_color_egui: egui::Color32::RED,
        }
    }

    /// egui フレームを実行
    pub fn frame(&mut self, state: &mut EditorState) {
        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("🎨 Nantaraquad Editor");
                    ui.separator();
                    if ui.button("📁 Open").clicked() {
                        // TODO: ファイルダイアログ
                    }
                    if ui.button("💾 Save").clicked() {
                        // TODO: ファイル保存
                    }
                });
            });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(200.0)
                .show(egui_ctx, |ui| {
                    ui.heading("Tools");

                    ui.group(|ui| {
                        ui.label("Brush Color");
                        ui.color_edit_button_srgba(&mut self.brush_color_egui);

                        // egui Color32 から [u8; 4] に変換
                        let [r, g, b, a] = self.brush_color_egui.to_srgba_unmultiplied();
                        state.brush_color = [r, g, b, a];
                    });

                    ui.group(|ui| {
                        ui.label("Brush Size");
                        if ui
                            .add(
                                egui::Slider::new(&mut self.brush_size_slider, 1.0..=16.0)
                                    .text("px"),
                            )
                            .changed()
                        {
                            state.set_brush_size(self.brush_size_slider as u32);
                        }
                    });

                    ui.separator();

                    ui.group(|ui| {
                        ui.label("View");
                        if ui.button("Zoom In (+)").clicked() {
                            state.zoom_in();
                        }
                        if ui.button("Zoom Out (-)").clicked() {
                            state.zoom_out();
                        }
                        ui.label(format!("Zoom: {:.1}x", state.zoom));
                    });

                    ui.separator();

                    if let Some(ColorMode::Indexed256(_)) = state.current_color_mode {
                        ui.label("📊 Indexed256 Mode");
                    } else {
                        ui.label("🎨 FullColor Mode");
                    }

                    if let Some(sprite_id) = state.sprite_id {
                        ui.label(format!("Sprite ID: {}", sprite_id));
                    } else {
                        ui.label("No sprite loaded");
                    }
                });

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.label("Canvas area - drawing will happen here");
                // TODO: キャンバス描画領域
            });
        });

        egui_macroquad::draw();
    }
}

impl Default for EditorUI {
    fn default() -> Self {
        Self::new()
    }
}
