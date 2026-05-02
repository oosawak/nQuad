//! ファイル入出力機能
//!
//! スプライトの保存・読み込みと、最近使用したファイルの管理。
//! Phase 6.5: nQDocumentFormat による完全なドキュメント保存

use crate::editor::{nQDocumentFormat, SpriteDocument};
use crate::resource::{ResourcePackage, SpriteData};
use std::fs;
use std::path::{Path, PathBuf};

/// ファイル I/O マネージャー
pub struct FileManager {
    /// 現在開いているファイルパス
    pub current_file: Option<PathBuf>,
    /// 最近開いたファイル（最大10個）
    pub recent_files: Vec<PathBuf>,
}

impl FileManager {
    /// 新規ファイルマネージャーを作成
    pub fn new() -> Self {
        Self {
            current_file: None,
            recent_files: Vec::new(),
        }
    }

    /// スプライトをファイルに保存
    ///
    /// # 引数
    /// - `path`: 保存先パス（.bin ファイル推奨）
    /// - `sprite`: 保存するスプライト
    ///
    /// # 戻り値
    /// - `Ok(PathBuf)`: 保存したファイルパス
    /// - `Err(String)`: 保存失敗（書き込み権限、ディスク容量等）
    pub fn save_sprite(&mut self, path: &str, sprite: &SpriteData) -> Result<PathBuf, String> {
        let path = PathBuf::from(path);

        // ResourcePackage にスプライトを追加して保存
        let mut pkg = ResourcePackage::new();
        pkg.add_sprite(sprite.clone());

        crate::resource::serialize::save_package(&pkg, &path)
            .map_err(|e| format!("Save failed: {}", e))?;

        self.current_file = Some(path.clone());
        self.add_recent_file(path.clone());

        Ok(path)
    }

    /// ファイルからスプライトを読み込み
    ///
    /// # 引数
    /// - `path`: 読み込むファイルパス
    ///
    /// # 戻り値
    /// - `Ok(Vec<SpriteData>)`: 読み込んだスプライトリスト
    /// - `Err(String)`: 読み込み失敗
    pub fn load_sprites(&mut self, path: &str) -> Result<Vec<SpriteData>, String> {
        let path = PathBuf::from(path);

        let pkg = crate::resource::serialize::load_package_safe(&path)?;

        if pkg.sprites.is_empty() {
            return Err("No sprites in file".to_string());
        }

        self.current_file = Some(path.clone());
        self.add_recent_file(path);

        Ok(pkg.sprites)
    }

    /// ドキュメント（レイヤー、アニメーション、履歴含む）を保存
    ///
    /// # 形式
    /// - `.nquad/` ディレクトリを作成
    /// - `.nquad/metadata.json`: ドキュメントメタデータ（レイヤー定義、アニメーション）
    /// - `.nquad/cel_FRAME_LAYER.bin`: 各フレーム・レイヤーのピクセルデータ（bincode）
    /// - `.nquad/history.json`: 編集履歴（最後の100コマンド）
    ///
    /// # 引数
    /// - `base_path`: 保存基底パス（例: "project.nquad"）
    /// - `document`: 保存するドキュメント
    ///
    /// # 戻り値
    /// - `Ok(PathBuf)`: 保存したドキュメントディレクトリパス
    /// - `Err(String)`: 保存失敗
    pub fn save_document<P: AsRef<Path>>(
        &mut self,
        base_path: P,
        document: &SpriteDocument,
    ) -> Result<PathBuf, String> {
        let doc_dir = base_path.as_ref().to_path_buf();

        // ディレクトリ作成（既存の場合はスキップ）
        fs::create_dir_all(&doc_dir)
            .map_err(|e| format!("Failed to create document directory: {}", e))?;

        // ドキュメント形式に変換
        let format = document.to_format();

        // メタデータを保存
        crate::editor::format::io::save_metadata(&format, doc_dir.join("metadata.json"))
            .map_err(|e| format!("Failed to save metadata: {}", e))?;

        // 編集履歴を保存
        save_history(&format, &doc_dir).map_err(|e| format!("Failed to save history: {}", e))?;

        // Cel ピクセルデータを保存
        save_cel_data(document, &doc_dir).map_err(|e| format!("Failed to save cel data: {}", e))?;

        self.current_file = Some(doc_dir.clone());
        self.add_recent_file(doc_dir.clone());

        Ok(doc_dir)
    }

    /// ドキュメントを読み込み
    ///
    /// # 引数
    /// - `doc_path`: ドキュメントディレクトリパス
    ///
    /// # 戻り値
    /// - `Ok(SpriteDocument)`: 読み込んだドキュメント
    /// - `Err(String)`: 読み込み失敗
    pub fn load_document<P: AsRef<Path>>(&mut self, doc_path: P) -> Result<SpriteDocument, String> {
        let doc_dir = doc_path.as_ref().to_path_buf();

        // メタデータを読み込み
        let format = crate::editor::format::io::load_metadata(doc_dir.join("metadata.json"))
            .map_err(|e| format!("Failed to load metadata: {}", e))?;

        // ドキュメント作成（最初のレイヤーのスプライトから）
        let first_sprite = SpriteData::new(32, 32, crate::resource::ColorMode::FullColor);
        let mut document = SpriteDocument::new(0, &format.name, first_sprite);

        // Cel ピクセルデータを読み込み
        load_cel_data(&mut document, &doc_dir, &format)
            .map_err(|e| format!("Failed to load cel data: {}", e))?;

        self.current_file = Some(doc_dir.clone());
        self.add_recent_file(doc_dir);

        Ok(document)
    }

    /// 最近開いたファイルを追加
    fn add_recent_file(&mut self, path: PathBuf) {
        // すでにリストにあれば削除（重複を避ける）
        self.recent_files.retain(|p| p != &path);

        // リストの先頭に追加
        self.recent_files.insert(0, path);

        // 最大10個に制限
        if self.recent_files.len() > 10 {
            self.recent_files.pop();
        }
    }

    /// 現在のファイル名を取得（パスなし）
    pub fn current_filename(&self) -> Option<String> {
        self.current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== ヘルパー関数 ====================

/// 編集履歴を JSON で保存
fn save_history(format: &nQDocumentFormat, doc_dir: &Path) -> Result<(), String> {
    let history_path = doc_dir.join("history.json");

    let json = serde_json::to_string_pretty(&format.history).map_err(|e| e.to_string())?;

    fs::write(history_path, json).map_err(|e| e.to_string())
}

/// Cel ピクセルデータを保存（複数フレーム対応）
fn save_cel_data(document: &SpriteDocument, doc_dir: &Path) -> Result<(), String> {
    let layers = document.layers.layers();

    // 各フレームについて
    if let Some(frame_layers) = document.get_frame_layers(0) {
        for (layer_id, sprite) in frame_layers {
            let cel_filename = format!("cel_0_{}.bin", layer_id);
            let cel_path = doc_dir.join(&cel_filename);

            let file = fs::File::create(&cel_path).map_err(|e| e.to_string())?;
            bincode::serialize_into(file, sprite)
                .map_err(|e| format!("Bincode serialization failed: {}", e))?;
        }
    } else {
        // フレームデータがない場合は現在のレイヤーから生成
        for layer in layers {
            let cel_filename = format!("cel_0_{}.bin", layer.id);
            let cel_path = doc_dir.join(&cel_filename);

            let file = fs::File::create(&cel_path).map_err(|e| e.to_string())?;
            bincode::serialize_into(file, &layer.sprite)
                .map_err(|e| format!("Bincode serialization failed: {}", e))?;
        }
    }

    Ok(())
}

/// Cel ピクセルデータを読み込み（複数フレーム対応）
fn load_cel_data(
    document: &mut SpriteDocument,
    doc_dir: &Path,
    format: &nQDocumentFormat,
) -> Result<(), String> {
    // フレームごとにセルを読み込み
    for frame_meta in &format.frames {
        let mut frame_layers = std::collections::HashMap::new();

        // 各レイヤーのセルファイルを読み込み
        for (layer_id, cel_filename) in &frame_meta.cels {
            let cel_path = doc_dir.join(cel_filename);

            if cel_path.exists() {
                let file = fs::File::open(&cel_path)
                    .map_err(|e| format!("Failed to open cel file {}: {}", cel_filename, e))?;

                let sprite: SpriteData = bincode::deserialize_from(file)
                    .map_err(|e| format!("Bincode deserialization failed: {}", e))?;

                frame_layers.insert(*layer_id, sprite);
            }
        }

        if !frame_layers.is_empty() {
            document.set_frame_layers(frame_meta.frame_num, frame_layers);
        }
    }

    // フレームメタデータを設定
    // TODO: アニメーションクリップごとのフレーム数を同期

    Ok(())
}
