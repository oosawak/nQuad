//! Phase 6.5: EditorState と SpriteDocument の統合層
//!
//! 既存の EditorState（UI状態）と新規の SpriteDocument（データモデル）を
//! 統合し、Cel モデルへの移行を支援します。

use crate::editor::{EditorState, LayerStack, SpriteDocument};
use crate::nquad::{nQDocumentId, nQLayerId};
use crate::resource::{ColorMode, SpriteData};

/// エディタセッション：ドキュメント管理とUI状態の統合
///
/// Phase 6.5 で導入。複数ドキュメント対応、Cel モデル対応。
#[derive(Debug)]
pub struct nQEditorSession {
    /// 開いているドキュメント
    documents: Vec<SpriteDocument>,
    /// アクティブなドキュメント ID
    active_doc_id: nQDocumentId,
    /// エディタ UI 状態（ブラシ、ズーム、パンなど）
    ui_state: EditorState,
    /// 次の割り当てドキュメント ID
    next_doc_id: nQDocumentId,
}

impl nQEditorSession {
    /// 新規セッションを作成
    pub fn new() -> Self {
        // デフォルトドキュメントを作成
        let default_sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = SpriteDocument::new(0, "Untitled", default_sprite);

        Self {
            documents: vec![doc],
            active_doc_id: 0,
            ui_state: EditorState::new(),
            next_doc_id: 1,
        }
    }

    /// 新規ドキュメントを作成
    pub fn create_document(
        &mut self,
        name: impl Into<String>,
        width: u32,
        height: u32,
    ) -> nQDocumentId {
        let sprite = SpriteData::new(width, height, ColorMode::FullColor);
        let doc = SpriteDocument::new(self.next_doc_id, name, sprite);

        let id = self.next_doc_id;
        self.next_doc_id += 1;

        self.documents.push(doc);
        self.active_doc_id = id;

        id
    }

    /// ドキュメントを取得
    pub fn get_document(&self, doc_id: nQDocumentId) -> Option<&SpriteDocument> {
        self.documents.iter().find(|d| d.id == doc_id)
    }

    /// ドキュメントを可変参照で取得
    pub fn get_document_mut(&mut self, doc_id: nQDocumentId) -> Option<&mut SpriteDocument> {
        self.documents.iter_mut().find(|d| d.id == doc_id)
    }

    /// アクティブなドキュメントを取得
    pub fn active_document(&self) -> &SpriteDocument {
        self.documents
            .iter()
            .find(|d| d.id == self.active_doc_id)
            .expect("active document must exist")
    }

    /// アクティブなドキュメントを可変参照で取得
    pub fn active_document_mut(&mut self) -> &mut SpriteDocument {
        self.documents
            .iter_mut()
            .find(|d| d.id == self.active_doc_id)
            .expect("active document must exist")
    }

    /// ドキュメントを選択
    pub fn select_document(&mut self, doc_id: nQDocumentId) -> bool {
        if self.documents.iter().any(|d| d.id == doc_id) {
            self.active_doc_id = doc_id;
            true
        } else {
            false
        }
    }

    /// UI 状態を取得
    pub fn ui_state(&self) -> &EditorState {
        &self.ui_state
    }

    /// UI 状態を可変参照で取得
    pub fn ui_state_mut(&mut self) -> &mut EditorState {
        &mut self.ui_state
    }

    /// ドキュメント数
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// 全ドキュメントを列挙
    pub fn documents(&self) -> &[SpriteDocument] {
        &self.documents
    }

    /// ドキュメントを保存（ファイル I/O）
    pub fn save_document(&self, doc_id: nQDocumentId, path: &str) -> Result<(), String> {
        if let Some(doc) = self.get_document(doc_id) {
            // TODO: Phase 6.5 ファイル形式を実装
            // doc.save(path)?;
            Ok(())
        } else {
            Err(format!("Document {} not found", doc_id))
        }
    }

    /// ドキュメントを読み込み（ファイル I/O）
    pub fn load_document(&mut self, path: &str) -> Result<nQDocumentId, String> {
        // TODO: Phase 6.5 ファイル形式を実装
        // let doc = SpriteDocument::load(path)?;
        Err("Not implemented yet".to_string())
    }
}

impl Default for nQEditorSession {
    fn default() -> Self {
        Self::new()
    }
}

/// アクティブレイヤーのピクセル編集（UI → ドキュメント）
pub trait nQEditorPainting {
    /// アクティブレイヤーにピクセルを設定
    fn paint_active_layer(&mut self, x: u32, y: u32, color: &[u8]) -> Result<(), String>;

    /// アクティブレイヤーからピクセルを取得
    fn sample_active_layer(&self, x: u32, y: u32) -> Option<Vec<u8>>;

    /// アクティブレイヤー ID
    fn active_layer_id(&self) -> Option<nQLayerId>;
}

impl nQEditorPainting for nQEditorSession {
    fn paint_active_layer(&mut self, x: u32, y: u32, color: &[u8]) -> Result<(), String> {
        let doc = self.active_document_mut();

        if let Some(layer) = doc.layers.active_layer_mut() {
            // ロック状態チェック
            if layer.locked {
                return Err("Layer is locked".to_string());
            }

            layer.sprite.set_pixel(x, y, color)?;
            doc.invalidate_composite();
            Ok(())
        } else {
            Err("No active layer".to_string())
        }
    }

    fn sample_active_layer(&self, x: u32, y: u32) -> Option<Vec<u8>> {
        let doc = self.active_document();

        if let Some(layer) = doc.layers.active_layer() {
            layer.sprite.get_pixel(x, y).map(|p| p.to_vec())
        } else {
            None
        }
    }

    fn active_layer_id(&self) -> Option<nQLayerId> {
        let doc = self.active_document();
        doc.layers.active_layer().map(|l| l.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = nQEditorSession::new();
        assert_eq!(session.document_count(), 1);
        assert_eq!(session.active_doc_id, 0);
    }

    #[test]
    fn test_create_document() {
        let mut session = nQEditorSession::new();
        let doc_id = session.create_document("Test", 64, 64);

        assert_eq!(session.document_count(), 2);
        assert_eq!(doc_id, 1);
        assert_eq!(session.active_doc_id, 1);
    }

    #[test]
    fn test_select_document() {
        let mut session = nQEditorSession::new();
        session.create_document("Doc2", 32, 32);

        assert!(session.select_document(0));
        assert_eq!(session.active_doc_id, 0);
    }

    #[test]
    fn test_paint_active_layer() {
        let mut session = nQEditorSession::new();
        let color = vec![255, 0, 0, 255];

        let result = session.paint_active_layer(0, 0, &color);
        assert!(result.is_ok());

        let sampled = session.sample_active_layer(0, 0);
        assert_eq!(sampled, Some(color));
    }

    #[test]
    fn test_locked_layer_prevents_painting() {
        let mut session = nQEditorSession::new();

        let doc = session.active_document_mut();
        if let Some(layer) = doc.layers.active_layer_mut() {
            layer.locked = true;
        }

        let result = session.paint_active_layer(0, 0, &[255, 0, 0, 255]);
        assert!(result.is_err());
    }
}
