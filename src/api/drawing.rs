use crate::resource::{ColorMode, SpriteData};

/// pyxel 互換の描画コンテキスト
pub struct DrawingContext {
    pub canvas: SpriteData,
    pub width: u32,
    pub height: u32,
    pub palette: Vec<[u8; 4]>,
}

impl DrawingContext {
    /// 新規描画コンテキストを作成
    pub fn new(width: u32, height: u32, palette: Vec<[u8; 4]>) -> Self {
        let canvas = SpriteData::new(width, height, ColorMode::FullColor);
        DrawingContext {
            canvas,
            width,
            height,
            palette,
        }
    }

    /// rect(x, y, w, h, color) - 矩形を描画（枠線）
    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: u8) {
        if w == 0 || h == 0 {
            return;
        }

        let rgba = self.index_to_rgba(color);

        // 上下のライン
        for dx in 0..w {
            let px = (x + dx as i32) as u32;
            self.set_pixel_safe(px, y as u32, &rgba);
            if h > 1 {
                self.set_pixel_safe(px, (y + h as i32 - 1) as u32, &rgba);
            }
        }

        // 左右のライン
        for dy in 1..(h as i32 - 1) {
            let py = (y + dy) as u32;
            self.set_pixel_safe(x as u32, py, &rgba);
            if w > 1 {
                self.set_pixel_safe((x + w as i32 - 1) as u32, py, &rgba);
            }
        }
    }

    /// rectfill(x, y, w, h, color) - 矩形を塗りつぶし
    pub fn rectfill(&mut self, x: i32, y: i32, w: u32, h: u32, color: u8) {
        let rgba = self.index_to_rgba(color);

        for dy in 0..h {
            for dx in 0..w {
                let px = (x + dx as i32) as u32;
                let py = (y + dy as i32) as u32;
                self.set_pixel_safe(px, py, &rgba);
            }
        }
    }

    /// circle(x, y, r, color) - 円を描画（枠線）
    pub fn circle(&mut self, x: i32, y: i32, r: i32, color: u8) {
        let rgba = self.index_to_rgba(color);

        for angle in 0..360 {
            let rad = angle as f32 * std::f32::consts::PI / 180.0;
            let px = x + (rad.cos() * r as f32) as i32;
            let py = y + (rad.sin() * r as f32) as i32;
            self.set_pixel_safe(px as u32, py as u32, &rgba);
        }
    }

    /// circfill(x, y, r, color) - 円を塗りつぶし
    pub fn circfill(&mut self, x: i32, y: i32, r: i32, color: u8) {
        let rgba = self.index_to_rgba(color);

        for dy in -(r)..=r {
            for dx in -(r)..=r {
                if dx * dx + dy * dy <= r * r {
                    let px = (x + dx) as u32;
                    let py = (y + dy) as u32;
                    self.set_pixel_safe(px, py, &rgba);
                }
            }
        }
    }

    /// line(x1, y1, x2, y2, color) - ラインを描画
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u8) {
        let rgba = self.index_to_rgba(color);
        self.bresenham_line(x1, y1, x2, y2, &rgba);
    }

    /// pset(x, y, color) - 単一ピクセル設定
    pub fn pset(&mut self, x: i32, y: i32, color: u8) {
        let rgba = self.index_to_rgba(color);
        self.set_pixel_safe(x as u32, y as u32, &rgba);
    }

    /// pget(x, y) -> Option<u8> - ピクセル取得（パレットインデックス）
    pub fn pget(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let offset = ((y as u32) * self.width + (x as u32)) as usize * 4;
        if let ColorMode::FullColor = &self.canvas.mode {
            if offset + 3 < self.canvas.pixels.len() {
                let r = self.canvas.pixels[offset];
                let g = self.canvas.pixels[offset + 1];
                let b = self.canvas.pixels[offset + 2];
                let a = self.canvas.pixels[offset + 3];

                // 最も近いパレットインデックスを探す
                return Some(self.find_closest_palette_index(r, g, b, a));
            }
        }
        None
    }

    /// cls(color) - キャンバスをクリア
    pub fn cls(&mut self, color: u8) {
        self.rectfill(0, 0, self.width, self.height, color);
    }

    /// set_palette(index, r, g, b, a) - パレット設定
    pub fn set_palette(&mut self, index: u8, r: u8, g: u8, b: u8, a: u8) {
        if (index as usize) < self.palette.len() {
            self.palette[index as usize] = [r, g, b, a];
        }
    }

    // ========== 内部ユーティリティ ==========

    fn index_to_rgba(&self, index: u8) -> [u8; 4] {
        if (index as usize) < self.palette.len() {
            self.palette[index as usize]
        } else {
            [0, 0, 0, 0]
        }
    }

    fn find_closest_palette_index(&self, r: u8, g: u8, b: u8, _a: u8) -> u8 {
        let mut closest = 0;
        let mut min_dist = u32::MAX;

        for (idx, &[pr, pg, pb, _pa]) in self.palette.iter().enumerate() {
            let dr = (r as i32 - pr as i32) as u32;
            let dg = (g as i32 - pg as i32) as u32;
            let db = (b as i32 - pb as i32) as u32;
            let dist = dr * dr + dg * dg + db * db;

            if dist < min_dist {
                min_dist = dist;
                closest = idx as u8;
            }
        }
        closest
    }

    fn set_pixel_safe(&mut self, x: u32, y: u32, rgba: &[u8; 4]) {
        if x < self.width && y < self.height {
            let _ = self.canvas.set_pixel(x, y, rgba);
        }
    }

    fn bresenham_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, rgba: &[u8; 4]) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx as f32 - dy as f32;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_pixel_safe(x as u32, y as u32, rgba);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2.0 * err;
            if e2 > -(dy as f32) {
                err -= dy as f32;
                x += sx;
            }
            if e2 < dx as f32 {
                err += dx as f32;
                y += sy;
            }
        }
    }

    /// テキストをドット絵フォントで描画
    ///
    /// 簡易 4x6 フォントを使用してテキストを描画します。
    /// 各文字は 4 ピクセル幅、6 ピクセル高さです。
    ///
    /// # 引数
    /// - `text`: 描画するテキスト
    /// - `x`: X座標
    /// - `y`: Y座標
    /// - `color`: パレットインデックス
    ///
    /// # 例
    /// ```ignore
    /// ctx.print("Hello", 10, 10, 7); // 白でテキスト描画
    /// ```
    pub fn print(&mut self, text: &str, x: i32, y: i32, color: u8) {
        let rgba = self.index_to_rgba(color);
        let mut pos_x = x;

        for ch in text.chars() {
            self.draw_char(ch, pos_x, y, &rgba);
            pos_x += 5; // 文字幅 4 + スペース 1
        }
    }

    /// 単一文字を描画（内部用）
    fn draw_char(&mut self, ch: char, x: i32, y: i32, rgba: &[u8; 4]) {
        // 簡易 ASCII ビットマップフォント
        // 各文字は 4x6 ピクセル
        let bitmap = match ch {
            'A' => [0b0110, 0b1001, 0b1111, 0b1001, 0b1001, 0b0000],
            'B' => [0b1110, 0b1001, 0b1110, 0b1001, 0b1110, 0b0000],
            'C' => [0b0110, 0b1000, 0b1000, 0b1000, 0b0110, 0b0000],
            'D' => [0b1110, 0b1001, 0b1001, 0b1001, 0b1110, 0b0000],
            'E' => [0b1111, 0b1000, 0b1110, 0b1000, 0b1111, 0b0000],
            'F' => [0b1111, 0b1000, 0b1110, 0b1000, 0b1000, 0b0000],
            'G' => [0b0110, 0b1000, 0b1011, 0b1001, 0b0110, 0b0000],
            'H' => [0b1001, 0b1001, 0b1111, 0b1001, 0b1001, 0b0000],
            'I' => [0b0111, 0b0010, 0b0010, 0b0010, 0b0111, 0b0000],
            'J' => [0b1110, 0b0010, 0b0010, 0b1010, 0b0100, 0b0000],
            'K' => [0b1001, 0b1010, 0b1100, 0b1010, 0b1001, 0b0000],
            'L' => [0b1000, 0b1000, 0b1000, 0b1000, 0b1111, 0b0000],
            'M' => [0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b00000],
            'N' => [0b1001, 0b1101, 0b1011, 0b1001, 0b1001, 0b0000],
            'O' => [0b0110, 0b1001, 0b1001, 0b1001, 0b0110, 0b0000],
            'P' => [0b1110, 0b1001, 0b1110, 0b1000, 0b1000, 0b0000],
            'Q' => [0b0110, 0b1001, 0b1001, 0b1010, 0b0101, 0b0000],
            'R' => [0b1110, 0b1001, 0b1110, 0b1010, 0b1001, 0b0000],
            'S' => [0b0111, 0b1000, 0b0110, 0b0001, 0b1110, 0b0000],
            'T' => [0b1111, 0b0010, 0b0010, 0b0010, 0b0010, 0b0000],
            'U' => [0b1001, 0b1001, 0b1001, 0b1001, 0b0110, 0b0000],
            'V' => [0b1001, 0b1001, 0b1001, 0b0110, 0b0010, 0b0000],
            'W' => [0b10001, 0b10101, 0b10101, 0b10101, 0b01010, 0b00000],
            'X' => [0b1001, 0b0110, 0b0110, 0b0110, 0b1001, 0b0000],
            'Y' => [0b1001, 0b0110, 0b0010, 0b0010, 0b0010, 0b0000],
            'Z' => [0b1111, 0b0010, 0b0100, 0b1000, 0b1111, 0b0000],
            '0' => [0b0110, 0b1001, 0b1001, 0b1001, 0b0110, 0b0000],
            '1' => [0b0010, 0b0110, 0b0010, 0b0010, 0b0111, 0b0000],
            '2' => [0b0110, 0b1001, 0b0010, 0b0100, 0b1111, 0b0000],
            '3' => [0b1110, 0b0001, 0b0110, 0b0001, 0b1110, 0b0000],
            '4' => [0b1000, 0b1100, 0b1010, 0b1111, 0b0010, 0b0000],
            '5' => [0b1111, 0b1000, 0b1110, 0b0001, 0b1110, 0b0000],
            '6' => [0b0110, 0b1000, 0b1110, 0b1001, 0b0110, 0b0000],
            '7' => [0b1111, 0b0001, 0b0010, 0b0100, 0b1000, 0b0000],
            '8' => [0b0110, 0b1001, 0b0110, 0b1001, 0b0110, 0b0000],
            '9' => [0b0110, 0b1001, 0b0111, 0b0001, 0b0110, 0b0000],
            ':' => [0b0000, 0b0010, 0b0000, 0b0010, 0b0000, 0b0000],
            '.' => [0b0000, 0b0000, 0b0000, 0b0000, 0b0010, 0b0000],
            ',' => [0b0000, 0b0000, 0b0000, 0b0010, 0b0100, 0b0000],
            '!' => [0b0010, 0b0010, 0b0010, 0b0000, 0b0010, 0b0000],
            '?' => [0b0110, 0b0001, 0b0010, 0b0000, 0b0010, 0b0000],
            ' ' => [0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000],
            _ => [0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000], // 未知の文字は空白
        };

        // ビットマップから描画
        for (row, bits) in bitmap.iter().enumerate() {
            let y_pos = y + row as i32;
            for col in 0..4 {
                if (bits & (1 << (3 - col))) != 0 {
                    let x_pos = x + col as i32;
                    self.set_pixel_safe(x_pos as u32, y_pos as u32, rgba);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawing_context_creation() {
        let palette = PYXEL_PALETTE.to_vec();
        let ctx = DrawingContext::new(160, 120, palette);
        assert_eq!(ctx.width, 160);
        assert_eq!(ctx.height, 120);
    }

    #[test]
    fn test_rect_drawing() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);
        ctx.rect(10, 10, 20, 20, 7); // white
        let color = ctx.pget(10, 10);
        assert_eq!(color, Some(7));
    }

    #[test]
    fn test_rectfill_drawing() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);
        ctx.rectfill(10, 10, 10, 10, 8); // red
        assert_eq!(ctx.pget(10, 10), Some(8));
        assert_eq!(ctx.pget(15, 15), Some(8));
    }

    #[test]
    fn test_pset_pget() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);
        ctx.pset(50, 50, 3); // green
        assert_eq!(ctx.pget(50, 50), Some(3));
        assert_eq!(ctx.pget(51, 50), None);
    }

    #[test]
    fn test_cls() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);
        ctx.rectfill(0, 0, 160, 120, 8); // fill with red
        ctx.cls(0); // clear to black
        assert_eq!(ctx.pget(80, 60), Some(0));
    }

    #[test]
    fn test_set_palette() {
        let palette = PYXEL_PALETTE.to_vec();
        let mut ctx = DrawingContext::new(160, 120, palette);
        ctx.set_palette(0, 255, 255, 0, 255); // yellow
        assert_eq!(ctx.palette[0], [255, 255, 0, 255]);
    }
}

/// pyxel 16色パレット定義
pub const PYXEL_PALETTE: &[[u8; 4]] = &[
    [0, 0, 0, 255],         // 0: black
    [43, 43, 87, 255],      // 1: navy
    [126, 37, 83, 255],     // 2: purple
    [0, 135, 81, 255],      // 3: green
    [171, 82, 54, 255],     // 4: brown
    [24, 57, 95, 255],      // 5: dark_blue
    [120, 183, 255, 255],   // 6: light_blue
    [255, 255, 255, 255],   // 7: white
    [255, 0, 77, 255],      // 8: red
    [255, 161, 0, 255],     // 9: orange
    [255, 240, 53, 255],    // 10: yellow
    [0, 231, 86, 255],      // 11: lime
    [41, 173, 255, 255],    // 12: cyan
    [131, 118, 156, 255],   // 13: gray
    [255, 119, 168, 255],   // 14: pink
    [255, 204, 170, 255],   // 15: peach
];
