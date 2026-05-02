//! スプライトドキュメント（Phase 7）
//!
//! エディタで編集する「作品」の単位。
//! - EditorDocument: 可変な編集状態を管理（Phase 7以降）
//! - SpriteDocument: 後方互換性のための従来の型
//! - SpriteAsset: 不変な資産データ
//!
//! EditorState（UI状態）とは分離。

use crate::editor::{AnimationController, Layer, LayerStack};
use crate::resource::{SpriteAsset, SpriteData};
use std::collections::HashMap;

/// 編集操作のコマンド型（Undo/Redo用）
#[derive(Clone, Debug)]
pub enum EditCommand {
    /// ピクセル描画
    PaintStroke {
        layer_id: u32,
        pixels: Vec<(u32, u32, Vec<u8>)>, // (x, y, color)
    },
    /// レイヤー追加
    AddLayer {
        layer_id: u32,
        name: String,
        sprite: SpriteData,
    },
    /// レイヤー削除
    DeleteLayer {
        layer_id: u32,
        layer_data: Layer,
        index: usize,
    },
    /// レイヤー移動
    MoveLayer {
        layer_id: u32,
        old_index: usize,
        new_index: usize,
    },
    /// レイヤー不透明度変更
    SetLayerOpacity {
        layer_id: u32,
        old_opacity: f32,
        new_opacity: f32,
    },
    /// レイヤーブレンドモード変更
    SetLayerBlendMode {
        layer_id: u32,
        old_mode: crate::editor::BlendMode,
        new_mode: crate::editor::BlendMode,
    },
    /// レイヤー可視性変更
    SetLayerVisibility {
        layer_id: u32,
        old_visible: bool,
        new_visible: bool,
    },
    /// レイヤーロック状態変更
    SetLayerLocked {
        layer_id: u32,
        old_locked: bool,
        new_locked: bool,
    },
    /// フレーム追加
    AddFrame {
        clip_id: usize,
        frame_idx: usize,
        frame_layers: LayerStack,
    },
    /// フレーム削除
    DeleteFrame {
        clip_id: usize,
        frame_idx: usize,
        frame_layers: LayerStack,
    },
    /// フレーム継続時間変更
    SetFrameDuration {
        clip_id: usize,
        frame_idx: usize,
        old_duration: u32,
        new_duration: u32,
    },
}

/// コマンド履歴管理（Undo/Redo）
#[derive(Clone, Debug)]
pub struct EditCommandHistory {
    undo_stack: Vec<EditCommand>,
    redo_stack: Vec<EditCommand>,
    max_history_size: usize, // デフォルト: 1000
}

impl EditCommandHistory {
    /// 新規履歴を作成
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history_size: 1000,
        }
    }

    /// コマンドを記録（Undo スタックに追加、Redo スタッククリア）
    pub fn record(&mut self, command: EditCommand) {
        self.undo_stack.push(command);
        self.redo_stack.clear();

        // メモリ管理：最大サイズを超えたら古いコマンドを削除
        if self.undo_stack.len() > self.max_history_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo 可能か
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Redo 可能か
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo を実行
    pub fn undo(&mut self) -> Option<EditCommand> {
        self.undo_stack.pop().map(|cmd| {
            self.redo_stack.push(cmd.clone());
            cmd
        })
    }

    /// Redo を実行
    pub fn redo(&mut self) -> Option<EditCommand> {
        self.redo_stack.pop().map(|cmd| {
            self.undo_stack.push(cmd.clone());
            cmd
        })
    }

    /// すべてクリア
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 最近のコマンド（最後の N 個）を取得
    pub fn get_recent_commands(&self, count: usize) -> Vec<EditCommand> {
        let start = if self.undo_stack.len() > count {
            self.undo_stack.len() - count
        } else {
            0
        };
        self.undo_stack[start..].to_vec()
    }
}

impl Default for EditCommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

// CommandHistory の別名（Phase 7 以降）
pub type CommandHistory = EditCommandHistory;

/// エディタドキュメント：可変な編集状態（Phase 7）
///
/// 不変な資産（SpriteAsset）と、編集中の可変状態を分離。
/// エディタで編集するすべての変更可能な情報を含む：
/// - 編集履歴（Undo/Redo）
/// - キャッシュされた合成画像
/// - アクティブレイヤーインデックス
/// - 現在表示中のフレーム
#[derive(Clone, Debug)]
pub struct EditorDocument {
    /// 不変な資産データ
    pub asset: SpriteAsset,
    /// 編集履歴（Undo/Redo）
    pub history: CommandHistory,
    /// キャッシュ済み合成画像
    pub cached_composite: Option<SpriteData>,
    /// キャッシュ無効化フラグ
    pub composite_dirty: bool,
    /// エディタで選択中のレイヤーインデックス
    pub active_layer_idx: usize,
    /// エディタで表示中のフレーム
    pub current_frame: u32,
    /// アニメーション状態（Task 1-3 で SpriteAnimator に分離予定）
    pub animation_state: AnimationController,
}

impl EditorDocument {
    /// 新規ドキュメントを作成
    pub fn new(asset: SpriteAsset) -> Self {
        // デフォルトアニメーション状態
        let first_frame = crate::editor::Frame::new(
            0,
            100,
            LayerStack::new(SpriteData::new(
                32,
                32,
                crate::resource::ColorMode::FullColor,
            )),
        );
        let animation_state =
            AnimationController::new(crate::editor::AnimationClip::new("Default", first_frame));

        Self {
            asset,
            history: CommandHistory::new(),
            cached_composite: None,
            composite_dirty: true,
            active_layer_idx: 0,
            current_frame: 0,
            animation_state,
        }
    }

    /// アクティブレイヤーを取得
    pub fn get_active_layer(&self) -> Option<&Layer> {
        // asset.layer_defs から active_layer_idx を使ってレイヤーを取得する
        // 後方互換性のため、ここでは簡易的に実装
        None
    }

    /// 不透明度を設定
    pub fn set_opacity(&mut self, layer_idx: usize, opacity: f32) {
        self.composite_dirty = true;
    }

    /// フレームデータを取得
    pub fn get_frame_data(&self, frame_num: u32) -> Option<&crate::resource::asset::FrameDef> {
        self.asset.get_frame(frame_num)
    }

    /// フレームデータを設定
    pub fn set_frame_data(&mut self, frame_num: u32, frame: crate::resource::asset::FrameDef) {
        self.asset.set_frame(frame_num, frame);
        self.composite_dirty = true;
    }

    /// 現在のフレーム番号を取得
    pub fn current_frame_num(&self) -> u32 {
        self.current_frame
    }

    /// フレームを変更
    pub fn set_current_frame(&mut self, frame_num: u32) {
        self.current_frame = frame_num;
        self.composite_dirty = true;
    }

    /// 編集操作を記録
    pub fn record_edit(&mut self, command: EditCommand) {
        self.history.record(command);
        self.composite_dirty = true;
    }

    /// Undo を実行
    pub fn undo(&mut self) -> Option<EditCommand> {
        let cmd = self.history.undo()?;
        self.composite_dirty = true;
        Some(cmd)
    }

    /// Redo を実行
    pub fn redo(&mut self) -> Option<EditCommand> {
        let cmd = self.history.redo()?;
        self.composite_dirty = true;
        Some(cmd)
    }

    /// Undo 可能か
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Redo 可能か
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// 合成キャッシュを無効化
    pub fn invalidate_composite(&mut self) {
        self.composite_dirty = true;
    }

    /// 合成済みスプライトを取得（キャッシュ使用）
    pub fn get_composite_sprite(&mut self) -> SpriteData {
        if self.composite_dirty || self.cached_composite.is_none() {
            // 簡易的な合成：最初のフレームのセルデータを返す
            if let Some(frame) = self.asset.get_frame(self.current_frame) {
                if let Some(cel) = frame.cels.values().next() {
                    self.cached_composite = Some(cel.pixels.clone());
                    self.composite_dirty = false;
                }
            }
        }

        self.cached_composite
            .clone()
            .unwrap_or_else(|| SpriteData::new(32, 32, crate::resource::ColorMode::FullColor))
    }

    /// ドキュメント形式として保存
    pub fn to_format(&self) -> crate::editor::nQDocumentFormat {
        use crate::editor::format::{
            nQAnimationClipData, nQDocumentFormat, nQFrameData, nQLayerDef,
        };

        let mut format = nQDocumentFormat::new(self.asset.name.clone());

        // レイヤー定義を変換
        for layer_def in &self.asset.layer_defs {
            format.layers.push(nQLayerDef {
                id: layer_def.id,
                name: layer_def.name.clone(),
                default_opacity: layer_def.default_opacity,
                default_blend_mode: layer_def.default_blend.clone(),
            });
        }

        // アニメーションクリップを変換
        for clip in &self.asset.animation_clips {
            let mut frames = Vec::new();
            for frame_def in &clip.frames {
                let mut cels = HashMap::new();
                for layer_def in &self.asset.layer_defs {
                    let cel_file = format!("cel_{}_{}.bin", frame_def.frame_num, layer_def.id);
                    cels.insert(layer_def.id, cel_file);
                }
                frames.push(nQFrameData {
                    frame_num: frame_def.frame_num,
                    duration_ms: frame_def.duration_ms,
                    cels,
                });
            }

            format.clips.push(nQAnimationClipData {
                name: clip.name.clone(),
                frames,
                looping: clip.looping,
            });
        }

        format
    }
}

///
/// エディタで編集するすべての情報を含む：
/// - レイヤー構造
/// - アニメーション定義
/// - 編集履歴
/// - 合成キャッシュ
/// - フレームデータ（複数フレーム対応）
#[derive(Clone, Debug)]
pub struct SpriteDocument {
    /// ドキュメント ID（ユニーク）
    pub id: usize,
    /// ドキュメント名（ファイル名など）
    pub name: String,
    /// レイヤースタック
    pub layers: LayerStack,
    /// アニメーション管理
    pub animations: AnimationController,
    /// 編集履歴
    pub history: EditCommandHistory,
    /// 合成結果キャッシュ
    cached_composite: Option<SpriteData>,
    /// キャッシュが有効か
    composite_dirty: bool,
    /// フレームごとのレイヤーデータ（Cel モデル）
    /// key: frame_num, value: HashMap<layer_id, layer_sprite>
    frame_data: std::collections::HashMap<u32, std::collections::HashMap<u32, SpriteData>>,
    /// 現在のフレーム番号
    current_frame: u32,
}

impl SpriteDocument {
    /// 新規ドキュメントを作成
    pub fn new(id: usize, name: impl Into<String>, initial_sprite: SpriteData) -> Self {
        let layers = LayerStack::new(initial_sprite.clone());

        // デフォルトアニメーションクリップを作成
        let first_frame = crate::editor::Frame::new(0, 100, layers.clone());
        let default_clip = crate::editor::AnimationClip::new("Default", first_frame);
        let animations = AnimationController::new(default_clip);

        // フレームデータ初期化
        let mut frame_data = std::collections::HashMap::new();
        let mut frame_0_layers = std::collections::HashMap::new();
        for layer in layers.layers() {
            frame_0_layers.insert(layer.id, layer.sprite.clone());
        }
        frame_data.insert(0, frame_0_layers);

        Self {
            id,
            name: name.into(),
            layers,
            animations,
            history: EditCommandHistory::new(),
            cached_composite: None,
            composite_dirty: true,
            frame_data,
            current_frame: 0,
        }
    }

    /// 編集操作を記録
    pub fn record_edit(&mut self, command: EditCommand) {
        self.history.record(command);
        self.invalidate_composite();
    }

    /// Undo を実行
    pub fn undo(&mut self) -> Option<EditCommand> {
        let cmd = self.history.undo()?;
        self.invalidate_composite();
        Some(cmd)
    }

    /// Redo を実行
    pub fn redo(&mut self) -> Option<EditCommand> {
        let cmd = self.history.redo()?;
        self.invalidate_composite();
        Some(cmd)
    }

    /// 現在のフレーム（アニメーションで表示するレイヤー構成）を取得
    pub fn current_display_layers(&self) -> &LayerStack {
        // TODO: アニメーションコントローラから現在フレームを取得
        // 当面はレイヤースタックをそのまま返す
        &self.layers
    }

    /// 合成済みスプライトを取得（キャッシュ使用）
    pub fn get_composite_sprite(&mut self) -> SpriteData {
        if self.composite_dirty || self.cached_composite.is_none() {
            if let Some(composite) = self.layers.composite() {
                self.cached_composite = Some(composite);
                self.composite_dirty = false;
            }
        }

        self.cached_composite.clone().unwrap_or_else(|| {
            // キャッシュがない場合は最初の可視レイヤーをデフォルトに
            self.layers
                .active_layer()
                .map(|l| l.sprite.clone())
                .unwrap_or_else(|| SpriteData::new(32, 32, crate::resource::ColorMode::FullColor))
        })
    }

    /// 合成キャッシュを無効化
    pub fn invalidate_composite(&mut self) {
        self.composite_dirty = true;
    }

    /// Undo 可能か
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Redo 可能か
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// フレームデータを取得
    pub fn get_frame_layers(
        &self,
        frame_num: u32,
    ) -> Option<&std::collections::HashMap<u32, SpriteData>> {
        self.frame_data.get(&frame_num)
    }

    /// フレームデータを設定
    pub fn set_frame_layers(
        &mut self,
        frame_num: u32,
        layers: std::collections::HashMap<u32, SpriteData>,
    ) {
        self.frame_data.insert(frame_num, layers);
    }

    /// 現在のフレーム番号を取得
    pub fn current_frame_num(&self) -> u32 {
        self.current_frame
    }

    /// フレームを変更
    pub fn set_current_frame(&mut self, frame_num: u32) {
        self.current_frame = frame_num;
        self.invalidate_composite();
    }

    /// ドキュメント形式として保存（メタデータ生成のみ）
    ///
    /// Cel ピクセルデータは別途 save_cel_data() で保存する。
    /// Returns: nQDocumentFormat メタデータ
    pub fn to_format(&self) -> crate::editor::nQDocumentFormat {
        use crate::editor::format::{
            nQAnimationClipData, nQDocumentFormat, nQFrameData, nQLayerDef,
        };
        use std::collections::HashMap;

        let mut format = nQDocumentFormat::new(self.name.clone());

        // レイヤー定義を変換
        for layer in self.layers.layers() {
            format.layers.push(nQLayerDef {
                id: layer.id,
                name: layer.name.clone(),
                default_opacity: layer.opacity,
                default_blend_mode: format!("{:?}", layer.blend_mode),
            });
        }

        // アニメーションクリップを変換
        for clip in self.animations.clips() {
            let mut frames = Vec::new();
            for frame in clip.get_frames() {
                let mut cels = HashMap::new();
                // 各フレームのセルファイル参照を生成
                for layer in self.layers.layers() {
                    let cel_file = format!("cel_{}_{}.bin", frame.frame_num, layer.id);
                    cels.insert(layer.id, cel_file);
                }

                frames.push(nQFrameData {
                    frame_num: frame.frame_num,
                    duration_ms: frame.duration_ms,
                    cels,
                });
            }

            format.clips.push(nQAnimationClipData {
                name: clip.name.clone(),
                frames,
                looping: clip.is_looping(),
            });
        }

        // 編集履歴の要約（最後の100コマンド）
        let records = self.history.get_recent_commands(100);
        for (idx, cmd) in records.into_iter().enumerate() {
            let (cmd_type, description) = match cmd {
                EditCommand::PaintStroke { layer_id, .. } => {
                    ("PaintStroke", format!("Paint on layer {}", layer_id))
                }
                EditCommand::AddLayer { name, .. } => ("AddLayer", format!("Add layer '{}'", name)),
                EditCommand::DeleteLayer { layer_id, .. } => {
                    ("DeleteLayer", format!("Delete layer {}", layer_id))
                }
                EditCommand::SetLayerOpacity { new_opacity, .. } => {
                    ("SetOpacity", format!("Opacity -> {:.2}", new_opacity))
                }
                EditCommand::SetLayerBlendMode { new_mode, .. } => {
                    ("SetBlendMode", format!("Blend -> {:?}", new_mode))
                }
                _ => ("Other", "Edit operation".to_string()),
            };

            format.history.push(crate::editor::format::nQCommandRecord {
                timestamp: idx as u64,
                command_type: cmd_type.to_string(),
                description,
            });
        }

        format
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    #[test]
    fn test_document_creation() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let doc = SpriteDocument::new(0, "Test", sprite);

        assert_eq!(doc.id, 0);
        assert_eq!(doc.name, "Test");
        assert!(doc.layers.active_layer().is_some());
        assert!(!doc.can_undo());
    }

    #[test]
    fn test_command_history() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = SpriteDocument::new(0, "Test", sprite);

        let cmd = EditCommand::SetLayerOpacity {
            layer_id: 0,
            old_opacity: 1.0,
            new_opacity: 0.5,
        };

        doc.record_edit(cmd);
        assert!(doc.can_undo());
        assert!(!doc.can_redo());

        doc.undo();
        assert!(!doc.can_undo());
        assert!(doc.can_redo());

        doc.redo();
        assert!(doc.can_undo());
    }

    #[test]
    fn test_composite_cache() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = SpriteDocument::new(0, "Test", sprite);

        assert!(doc.composite_dirty);

        let _ = doc.get_composite_sprite();
        assert!(!doc.composite_dirty);

        doc.invalidate_composite();
        assert!(doc.composite_dirty);
    }

    #[test]
    fn test_editor_document_creation() {
        let mut asset = SpriteAsset::new(1, "TestAsset");
        let layer_def = crate::resource::asset::LayerDef::new(0, "Layer 0");
        asset.add_layer_def(layer_def);

        let doc = EditorDocument::new(asset);
        assert_eq!(doc.asset.id, 1);
        assert_eq!(doc.asset.name, "TestAsset");
        assert_eq!(doc.active_layer_idx, 0);
        assert_eq!(doc.current_frame, 0);
        assert!(doc.composite_dirty);
        assert!(!doc.can_undo());
    }

    #[test]
    fn test_editor_document_frame_navigation() {
        let asset = SpriteAsset::new(1, "TestAsset");
        let mut doc = EditorDocument::new(asset);

        doc.set_current_frame(5);
        assert_eq!(doc.current_frame_num(), 5);
        assert!(doc.composite_dirty);
    }

    #[test]
    fn test_editor_document_history() {
        let asset = SpriteAsset::new(1, "TestAsset");
        let mut doc = EditorDocument::new(asset);

        let cmd = EditCommand::SetLayerOpacity {
            layer_id: 0,
            old_opacity: 1.0,
            new_opacity: 0.5,
        };

        doc.record_edit(cmd);
        assert!(doc.can_undo());
        assert!(!doc.can_redo());

        doc.undo();
        assert!(!doc.can_undo());
        assert!(doc.can_redo());

        doc.redo();
        assert!(doc.can_undo());
    }

    #[test]
    fn test_editor_document_to_format() {
        let mut asset = SpriteAsset::new(1, "TestAsset");
        let layer_def = crate::resource::asset::LayerDef::new(0, "Layer 0");
        asset.add_layer_def(layer_def);

        let doc = EditorDocument::new(asset);
        let format = doc.to_format();

        assert_eq!(format.name, "TestAsset");
        assert_eq!(format.layers.len(), 1);
    }
}
