//! パフォーマンス最適化
//!
//! バッチ処理、メモリ効率、キャッシング戦略。

/// ピクセルバッチ処理
///
/// 複数のピクセル変更をバッチで行い、テクスチャ同期の回数を減らします。
pub struct PixelBatch {
    /// (sprite_id, x, y, color) の変更リスト
    changes: Vec<(usize, u32, u32, Vec<u8>)>,
}

impl PixelBatch {
    /// 新規バッチを作成
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// ピクセル変更をバッチに追加
    pub fn add_change(&mut self, sprite_id: usize, x: u32, y: u32, color: Vec<u8>) {
        self.changes.push((sprite_id, x, y, color));
    }

    /// バッチ内の変更数
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// バッチが空か
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// バッチを取得（イテレーション用）
    pub fn iter(&self) -> impl Iterator<Item = &(usize, u32, u32, Vec<u8>)> {
        self.changes.iter()
    }

    /// バッチをクリア
    pub fn clear(&mut self) {
        self.changes.clear();
    }

    /// バッチを実行（すべての変更を適用）
    pub fn execute(&mut self) {
        for (sprite_id, x, y, color) in self.changes.drain(..) {
            let _ = crate::api::set_pixel(sprite_id, x, y, &color);
        }
    }
}

impl Default for PixelBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// グリッドキャッシュ
///
/// グリッド線をキャッシュして毎フレーム再計算を避ける。
pub struct GridCache {
    /// キャッシュされたグリッド線の頂点リスト
    vertices: Vec<(f32, f32, f32, f32)>,
    /// 最後にキャッシュされたズーム値
    last_zoom: f32,
    /// キャッシュが有効か
    is_valid: bool,
}

impl GridCache {
    /// 新規グリッドキャッシュを作成
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            last_zoom: 0.0,
            is_valid: false,
        }
    }

    /// グリッドをキャッシュ（ズーム値が変わったときだけ再計算）
    pub fn update_cache(&mut self, zoom: f32, pan_x: f32, pan_y: f32) {
        if (zoom - self.last_zoom).abs() < 0.01 && self.is_valid {
            return; // キャッシュ有効
        }

        self.vertices.clear();
        self.last_zoom = zoom;

        let grid_size = zoom as i32;
        if grid_size <= 0 {
            return;
        }

        // 縦線
        let mut x = pan_x as i32;
        while x < 1024 {
            self.vertices
                .push((x as f32, pan_y, x as f32, pan_y + 768.0));
            x += grid_size;
        }

        // 横線
        let mut y = pan_y as i32;
        while y < 768 {
            self.vertices
                .push((pan_x, y as f32, pan_x + 1024.0, y as f32));
            y += grid_size;
        }

        self.is_valid = true;
    }

    /// キャッシュされた頂点を取得
    pub fn vertices(&self) -> &[(f32, f32, f32, f32)] {
        &self.vertices
    }

    /// キャッシュを無効化
    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }
}

impl Default for GridCache {
    fn default() -> Self {
        Self::new()
    }
}

/// メモリプール
///
/// 頻繁に割り当て・解放されるバッファを再利用。
pub struct MemoryPool {
    /// 予備バッファ（4096 バイト）
    buffers: Vec<Vec<u8>>,
}

impl MemoryPool {
    /// 新規メモリプールを作成
    pub fn new() -> Self {
        Self {
            buffers: Vec::with_capacity(4),
        }
    }

    /// バッファを取得（なければ新規作成）
    pub fn acquire(&mut self, size: usize) -> Vec<u8> {
        if let Some(buf) = self.buffers.pop() {
            if buf.capacity() >= size {
                return buf;
            }
        }
        Vec::with_capacity(size)
    }

    /// バッファを返却（再利用のため保存）
    pub fn release(&mut self, mut buffer: Vec<u8>) {
        buffer.clear();
        if self.buffers.len() < 4 {
            self.buffers.push(buffer);
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}
