//! nQuad ドキュメント形式（Phase 6.5 & 7）
//!
//! レイヤー、アニメーション、ブレンドモード、EditCommand 履歴を含む
//! 完全なドキュメント保存形式。JSON メタデータ + bincode ピクセルデータ。
//!
//! DocumentFormat v2（Phase 7）：SpriteAsset をベースに sparse cel モデルを採用。

use crate::editor::{AnimationClip, EditCommand, Layer, SpriteDocument};
use crate::resource::ColorMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// nQuad ドキュメント形式 v1
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQDocumentFormat {
    /// ファイル形式バージョン
    pub version: u32,
    /// ドキュメント名
    pub name: String,
    /// レイヤー定義
    pub layers: Vec<nQLayerDef>,
    /// アニメーションクリップ
    pub clips: Vec<nQAnimationClipData>,
    /// フレームデータ（Cel ピクセルは別ファイル）
    pub frames: Vec<nQFrameData>,
    /// 編集履歴（最後の100コマンド）
    pub history: Vec<nQCommandRecord>,
}

impl nQDocumentFormat {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            version: 1,
            name: name.into(),
            layers: Vec::new(),
            clips: Vec::new(),
            frames: Vec::new(),
            history: Vec::new(),
        }
    }
}

/// レイヤー定義データ（シリアライズ可能）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQLayerDef {
    pub id: u32,
    pub name: String,
    pub default_opacity: f32,
    pub default_blend_mode: String, // "Normal", "Add", "Multiply", "Screen"
}

/// アニメーションクリップデータ
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQAnimationClipData {
    pub name: String,
    pub frames: Vec<nQFrameData>,
    pub looping: bool,
}

/// フレームデータ（メタデータのみ）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQFrameData {
    pub frame_num: u32,
    pub duration_ms: u32,
    /// 各レイヤーの Cel ファイルパス参照
    pub cels: HashMap<u32, String>, // layer_id -> "cel_0_0.bin"
}

/// フレームメタデータ（統計情報）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQFrameMetadata {
    pub frame_num: u32,
    pub duration_ms: u32,
    pub layer_count: usize,
    pub total_pixels: u64,
}

/// 編集コマンド記録（履歴復元用）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct nQCommandRecord {
    pub timestamp: u64,
    pub command_type: String,
    pub description: String,
}

/// ドキュメント形式の検証エラー
#[derive(Clone, Debug)]
pub enum nQFormatError {
    InvalidVersion,
    MissingLayers,
    MissingClips,
    CorruptedMetadata(String),
    IOError(String),
}

impl std::fmt::Display for nQFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            nQFormatError::InvalidVersion => write!(f, "Unsupported file version"),
            nQFormatError::MissingLayers => write!(f, "No layers defined"),
            nQFormatError::MissingClips => write!(f, "No animation clips"),
            nQFormatError::CorruptedMetadata(msg) => write!(f, "Corrupted metadata: {}", msg),
            nQFormatError::IOError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for nQFormatError {}

/// ドキュメント保存・読み込み
pub mod io {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// ドキュメントを JSON メタデータとして保存
    pub fn save_metadata<P: AsRef<Path>>(
        doc: &nQDocumentFormat,
        path: P,
    ) -> Result<(), nQFormatError> {
        let json = serde_json::to_string_pretty(doc)
            .map_err(|e| nQFormatError::CorruptedMetadata(e.to_string()))?;

        fs::write(path, json).map_err(|e| nQFormatError::IOError(e.to_string()))?;

        Ok(())
    }

    /// JSON メタデータから復元
    pub fn load_metadata<P: AsRef<Path>>(path: P) -> Result<nQDocumentFormat, nQFormatError> {
        let json = fs::read_to_string(path).map_err(|e| nQFormatError::IOError(e.to_string()))?;

        let doc: nQDocumentFormat = serde_json::from_str(&json)
            .map_err(|e| nQFormatError::CorruptedMetadata(e.to_string()))?;

        // バージョンチェック
        if doc.version != 1 {
            return Err(nQFormatError::InvalidVersion);
        }

        Ok(doc)
    }

    /// Cel ファイル名を生成
    pub fn cel_filename(frame_num: u32, layer_id: u32) -> String {
        format!("cel_{}_{}.bin", frame_num, layer_id)
    }

    /// ドキュメントディレクトリ名を生成
    pub fn doc_dir_name(doc_name: &str) -> String {
        format!("{}.nquad", doc_name.replace(" ", "_").to_lowercase())
    }
}

/// ドキュメントメタデータ（DocumentFormat v2）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// ドキュメント名
    pub name: String,
    /// スプライト幅
    pub width: u32,
    /// スプライト高さ
    pub height: u32,
    /// 総フレーム数
    pub frame_count: u32,
    /// ファイル形式バージョン
    pub version: u32,
}

/// レイヤー定義（DocumentFormat v2）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerDef {
    /// レイヤー ID
    pub id: u32,
    /// レイヤー名
    pub name: String,
    /// デフォルト不透明度
    pub default_opacity: f32,
    /// デフォルトブレンドモード
    pub default_blend_mode: String,
}

/// Cel データ（ピクセルデータ付き、DocumentFormat v2）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CelData {
    /// Cel が属するレイヤー ID
    pub layer_id: u32,
    /// Cel が属するフレーム番号
    pub frame_num: u32,
    /// ピクセルデータ（bincode シリアライズ可能）
    pub pixels: Vec<u8>,
    /// ピクセル幅
    pub width: u32,
    /// ピクセル高さ
    pub height: u32,
    /// カラーモード
    pub color_mode: ColorMode,
    /// フレーム内での可視性
    pub visible: bool,
    /// フレーム内での不透明度上書き
    pub opacity_override: Option<f32>,
    /// フレーム内でのブレンドモード上書き
    pub blend_override: Option<String>,
}

/// アニメーションクリップ定義（DocumentFormat v2）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationClipFormat {
    /// クリップ名
    pub name: String,
    /// フレーム番号リスト
    pub frame_numbers: Vec<u32>,
    /// ループするか
    pub looping: bool,
    /// クリップ内のフレーム持続時間（フレーム番号 → 持続時間 ms）
    pub frame_durations: HashMap<u32, u32>,
}

/// ドキュメント形式 v2（Phase 7）
///
/// SpriteAsset をベースに sparse cel モデルを採用。
/// 必要な Cel のみを保存し、メモリ効率を実現。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentFormat {
    /// ドキュメントメタデータ
    pub metadata: DocumentMetadata,
    /// レイヤー定義リスト
    pub layers: Vec<LayerDef>,
    /// アニメーションクリップリスト
    pub clips: Vec<AnimationClipFormat>,
    /// Cel データ（sparse: 存在する cel のみ）
    pub cel_data: Vec<CelData>,
}

impl DocumentFormat {
    /// 新規ドキュメント形式を作成
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            metadata: DocumentMetadata {
                name: name.into(),
                width,
                height,
                frame_count: 0,
                version: 2,
            },
            layers: Vec::new(),
            clips: Vec::new(),
            cel_data: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_format_creation() {
        let doc = nQDocumentFormat::new("Test Document");
        assert_eq!(doc.version, 1);
        assert_eq!(doc.name, "Test Document");
        assert!(doc.layers.is_empty());
    }

    #[test]
    fn test_layer_def_serialization() {
        let layer = nQLayerDef {
            id: 0,
            name: "Layer 0".to_string(),
            default_opacity: 1.0,
            default_blend_mode: "Normal".to_string(),
        };

        let json = serde_json::to_string(&layer).unwrap();
        let restored: nQLayerDef = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.id, 0);
        assert_eq!(restored.name, "Layer 0");
    }

    #[test]
    fn test_cel_filename() {
        let name = io::cel_filename(0, 0);
        assert_eq!(name, "cel_0_0.bin");

        let name = io::cel_filename(10, 5);
        assert_eq!(name, "cel_10_5.bin");
    }

    #[test]
    fn test_doc_dir_name() {
        let dir = io::doc_dir_name("My Project");
        assert_eq!(dir, "my_project.nquad");
    }
}
