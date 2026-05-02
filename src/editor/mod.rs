//! エディタモジュール（Phase 3-5）
//!
//! UI フレームワーク、ピクセルペイント、リアルタイムプレビュー、
//! ファイル I/O、Undo/Redo、キーボード入力、パフォーマンス最適化、
//! PNG エクスポート、追加ツール、フィルターを提供します。

pub mod animation;
pub mod cache;
pub mod celmodel;
pub mod command;
pub mod document;
pub mod export;
pub mod file;
pub mod filters;
pub mod format;
pub mod history;
pub mod input;
pub mod integration;
pub mod layers;
pub mod paint;
pub mod perf;
pub mod preview;
pub mod session;
pub mod state;
pub mod tools;
pub mod ui;

pub use animation::{AnimationClip, AnimationController, Frame};
pub use cache::{CacheStats, CacheStrategy, CompositeCache};
pub use celmodel::{nQFrame, Cel, LayerDef};
pub use document::{
    CommandHistory, EditCommand, EditCommandHistory, EditorDocument, SpriteDocument,
};
pub use export::PngExporter;
pub use file::FileManager;
pub use filters::Filters;
pub use format::{nQAnimationClipData, nQDocumentFormat, nQFormatError, nQLayerDef};
pub use history::{PixelChange, UndoRedoStack};
pub use input::InputManager;
pub use integration::{export_to_game_entity, export_to_scene, optimize_for_game};
pub use layers::{BlendMode, Layer, LayerStack};
pub use paint::PaintTool;
pub use perf::{GridCache, MemoryPool, PixelBatch};
pub use preview::SpritePreview;
pub use session::{nQEditorPainting, nQEditorSession};
pub use state::EditorState;
pub use tools::{BucketFillTool, ColorPickerTool, EraserTool, ToolType};
pub use ui::EditorUI;
