//! Undo/Redo システム
//!
//! ピクセル変更履歴を管理し、Ctrl+Z/Ctrl+Y で操作を戻す・進める。

use std::collections::VecDeque;

/// ピクセル変更レコード
#[derive(Clone, Debug)]
pub struct PixelChange {
    /// スプライト ID
    pub sprite_id: usize,
    /// X座標
    pub x: u32,
    /// Y座標
    pub y: u32,
    /// 変更前の色（RGB or インデックス）
    pub old_color: Vec<u8>,
    /// 変更後の色
    pub new_color: Vec<u8>,
}

/// Undo/Redo スタック
///
/// 最大1000フレームまで履歴を保持します。
pub struct UndoRedoStack {
    /// Undo スタック
    undo_stack: VecDeque<PixelChange>,
    /// Redo スタック
    redo_stack: VecDeque<PixelChange>,
    /// 最大履歴数
    max_history: usize,
}

impl UndoRedoStack {
    /// 新規スタックを作成
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 1000,
        }
    }

    /// ピクセル変更を記録
    pub fn record(&mut self, change: PixelChange) {
        self.undo_stack.push_back(change);
        self.redo_stack.clear(); // 新しい操作があれば Redo スタックをクリア

        // 履歴数が多すぎたら古い方を削除
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }
    }

    /// Undo（1ステップ戻す）
    ///
    /// # 戻り値
    /// - `Some(PixelChange)`: アンドゥ対象の変更（old_color に戻す）
    /// - `None`: アンドゥできる操作がない
    pub fn undo(&mut self) -> Option<PixelChange> {
        if let Some(change) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(change.clone());
            Some(change)
        } else {
            None
        }
    }

    /// Redo（1ステップ進める）
    ///
    /// # 戻り値
    /// - `Some(PixelChange)`: リドゥ対象の変更（new_color に進める）
    /// - `None`: リドゥできる操作がない
    pub fn redo(&mut self) -> Option<PixelChange> {
        if let Some(change) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(change.clone());
            Some(change)
        } else {
            None
        }
    }

    /// アンドゥ可能か
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// リドゥ可能か
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 履歴をクリア
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// アンドゥスタックのサイズ
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// リドゥスタックのサイズ
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for UndoRedoStack {
    fn default() -> Self {
        Self::new()
    }
}
