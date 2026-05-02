//! 追加ツール
//!
//! 消しゴム、バケットフィル、色選択などの拡張ツール。

use crate::api::{get_pixel, set_pixel};

/// ツールタイプ
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolType {
    /// ペンシル（デフォルト）
    Pencil,
    /// 消しゴム
    Eraser,
    /// バケットフィル
    BucketFill,
}

/// 消しゴムツール
pub struct EraserTool;

impl EraserTool {
    /// 1ピクセルを消去（透明に）
    pub fn erase(sprite_id: usize, x: u32, y: u32) -> Result<(), String> {
        // 透明色 [0, 0, 0, 0] を設定
        set_pixel(sprite_id, x, y, &[0, 0, 0, 0])
    }

    /// 矩形範囲を消去
    pub fn erase_rect(sprite_id: usize, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), String> {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                Self::erase(sprite_id, x, y)?;
            }
        }

        Ok(())
    }
}

/// バケットフィル（フラッドフィル）
pub struct BucketFillTool;

impl BucketFillTool {
    /// 指定位置から同じ色の領域を埋める
    ///
    /// # 引数
    /// - `sprite_id`: スプライト ID
    /// - `start_x`: 開始 X 座標
    /// - `start_y`: 開始 Y 座標
    /// - `fill_color`: 塗りつぶし色
    /// - `width`: スプライト幅
    /// - `height`: スプライト高さ
    pub fn fill(
        sprite_id: usize,
        start_x: u32,
        start_y: u32,
        fill_color: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        // 開始位置の元の色を取得
        let original_color =
            get_pixel(sprite_id, start_x, start_y).ok_or("Invalid pixel position")?;

        // 元の色と塗りつぶし色が同じ場合はスキップ
        if original_color == fill_color {
            return Ok(());
        }

        // フラッドフィル実行（スタック方式）
        let mut stack = vec![(start_x, start_y)];
        let mut visited = std::collections::HashSet::new();

        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            // 境界チェック
            if x >= width || y >= height {
                continue;
            }

            // 色チェック
            if let Some(pixel) = get_pixel(sprite_id, x, y) {
                if pixel != original_color {
                    continue;
                }
            }

            // ピクセルを塗りつぶし
            let _ = set_pixel(sprite_id, x, y, fill_color);

            // 上下左右に拡張
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < height - 1 {
                stack.push((x, y + 1));
            }
        }

        Ok(())
    }
}

/// 色選択ツール（スポイト）
pub struct ColorPickerTool;

impl ColorPickerTool {
    /// 指定位置の色を取得
    pub fn pick_color(sprite_id: usize, x: u32, y: u32) -> Option<Vec<u8>> {
        get_pixel(sprite_id, x, y)
    }
}
