use serde::{Deserialize, Serialize};

/// カラーモード定義
///
/// スプライトのピクセル保存方式を指定します。メモリ効率と機能のトレードオフを提供します。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ColorMode {
    /// 256色パレットモード (インデックス + パレット)
    ///
    /// 各ピクセルが 1 バイト（0-255）で、パレットテーブルを参照して色を決定します。
    /// 容量削減に最適で、復古的なドット絵スタイルに向いています。
    ///
    /// # パレット形式
    /// - `Vec<[u8; 4]>` = RGBA カラーの配列（256色分）
    /// - インデックス値がパレット配列の添字として機能
    /// - 範囲外のインデックスは透明黒 `[0, 0, 0, 0]` にフォールバック
    ///
    /// # 例
    /// ```ignore
    /// let palette = vec![
    ///     [0, 0, 0, 255],       // インデックス 0: 黒
    ///     [255, 0, 0, 255],     // インデックス 1: 赤
    ///     [0, 255, 0, 255],     // インデックス 2: 緑
    /// ];
    /// let sprite = SpriteData::new(16, 16, ColorMode::Indexed256(palette));
    /// ```
    Indexed256(Vec<[u8; 4]>),
    /// フルカラーモード (RGBA直接指定)
    ///
    /// 各ピクセルが 4 バイト（RGBA）で、直接色情報を保持します。
    /// 最大 1677 万色（32 bit）対応で、グラデーションや複雑な画像に対応。
    /// メモリ使用量は Indexed256 の 4 倍。
    ///
    /// # ピクセルレイアウト
    /// - バイト 0: Red (0-255)
    /// - バイト 1: Green (0-255)
    /// - バイト 2: Blue (0-255)
    /// - バイト 3: Alpha (0=透明, 255=不透明)
    ///
    /// # 例
    /// ```ignore
    /// let sprite = SpriteData::new(16, 16, ColorMode::FullColor);
    /// sprite.set_pixel(0, 0, &[255, 0, 0, 255])?; // 赤色
    /// ```
    FullColor,
}

/// 個別のスプライトデータ (CPU側保持用)
///
/// ピクセル情報とメタデータを保持します。GPU への転送は Engine が担当。
/// シリアライズ対応（bincode）で、ディスクに保存・読み込み可能。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteData {
    /// スプライトの幅（ピクセル）
    pub width: u32,
    /// スプライトの高さ（ピクセル）
    pub height: u32,
    /// カラーモード
    pub mode: ColorMode,
    /// ピクセルデータ（メモリレイアウト）
    ///
    /// - **Indexed256**: 1 バイト/ピクセル（パレットインデックス）
    /// - **FullColor**: 4 バイト/ピクセル（RGBA）
    ///
    /// メモリレイアウトは行優先（row-major）：
    /// ```text
    /// pixels[y * width + x] = pixel at (x, y)
    /// ```
    pub pixels: Vec<u8>,
}

impl SpriteData {
    /// 新規スプライト作成
    ///
    /// 指定されたサイズとカラーモードで初期化します。
    /// ピクセルはすべて 0（黒/透明）で初期化されます。
    ///
    /// # 引数
    /// - `width`: スプライトの幅（ピクセル）
    /// - `height`: スプライトの高さ（ピクセル）
    /// - `mode`: カラーモード（Indexed256 または FullColor）
    ///
    /// # パニック
    /// - 幅または高さが 0 の場合、`pixels` は空になります（通常はエラーにしません）
    ///
    /// # 例
    /// ```ignore
    /// // フルカラー 32x32
    /// let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
    ///
    /// // インデックスカラー 16x16
    /// let palette = vec![[255, 0, 0, 255]];
    /// let sprite = SpriteData::new(16, 16, ColorMode::Indexed256(palette));
    /// ```
    pub fn new(width: u32, height: u32, mode: ColorMode) -> Self {
        let pixel_count = (width * height) as usize;
        let pixels = match &mode {
            ColorMode::Indexed256(_) => vec![0; pixel_count], // インデックス: 1byte
            ColorMode::FullColor => vec![0; pixel_count * 4], // RGBA: 4bytes
        };

        Self {
            width,
            height,
            mode,
            pixels,
        }
    }

    /// ピクセルデータを取得 (読み取り用)
    ///
    /// # 引数
    /// - `x`: X座標（0 から `width - 1`）
    /// - `y`: Y座標（0 から `height - 1`）
    ///
    /// # 戻り値
    /// - `Some(&[u8])`: ピクセル値へのスライス
    ///   - Indexed256: 1 バイト
    ///   - FullColor: 4 バイト
    /// - `None`: 座標が範囲外
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let offset = (y * self.width + x) as usize;
        match &self.mode {
            ColorMode::Indexed256(_) => {
                let idx = offset;
                self.pixels.get(idx..(idx + 1))
            }
            ColorMode::FullColor => {
                let idx = offset * 4;
                self.pixels.get(idx..(idx + 4))
            }
        }
    }

    /// ピクセルデータを設定 (書き込み用)
    ///
    /// # 引数
    /// - `x`: X座標
    /// - `y`: Y座標
    /// - `value`: ピクセル値
    ///   - Indexed256: `&[u8]` 長さ 1（パレットインデックス 0-255）
    ///   - FullColor: `&[u8]` 長さ 4（RGBA）
    ///
    /// # 戻り値
    /// - `Ok(())`: 設定成功
    /// - `Err(String)`: 座標範囲外またはピクセル値サイズ不正
    ///
    /// # 例
    /// ```ignore
    /// let mut sprite = SpriteData::new(16, 16, ColorMode::FullColor);
    /// sprite.set_pixel(5, 5, &[255, 0, 0, 255])?; // 赤色
    /// ```
    pub fn set_pixel(&mut self, x: u32, y: u32, value: &[u8]) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel out of bounds: ({}, {})", x, y));
        }

        let offset = (y * self.width + x) as usize;
        match &self.mode {
            ColorMode::Indexed256(_) => {
                if value.len() != 1 {
                    return Err("Indexed256 requires 1 byte per pixel".to_string());
                }
                self.pixels[offset] = value[0];
            }
            ColorMode::FullColor => {
                if value.len() != 4 {
                    return Err("FullColor requires 4 bytes per pixel (RGBA)".to_string());
                }
                let idx = offset * 4;
                self.pixels[idx..idx + 4].copy_from_slice(value);
            }
        }

        Ok(())
    }

    /// 期待されるピクセルデータサイズを計算
    ///
    /// # 戻り値
    /// - Indexed256: width * height
    /// - FullColor: width * height * 4
    pub fn get_expected_pixel_size(&self) -> usize {
        let pixel_count = (self.width * self.height) as usize;
        match &self.mode {
            ColorMode::Indexed256(_) => pixel_count,
            ColorMode::FullColor => pixel_count * 4,
        }
    }
}

/// リソースパッケージ全体 (ファイル保存単位)
///
/// 複数のスプライトをまとめてファイルに保存・読み込みできるコンテナ。
/// bincode でシリアライズされ、`.bin` ファイルとして保存されます。
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourcePackage {
    /// スプライトリスト
    pub sprites: Vec<SpriteData>,
    // 将来: tilemaps, sounds, fonts
}

impl ResourcePackage {
    /// 新規パッケージ作成
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    /// スプライト追加
    ///
    /// パッケージにスプライトを追加し、そのインデックスを返します。
    ///
    /// # 戻り値
    /// 割り当てられたスプライトのインデックス（ID）
    pub fn add_sprite(&mut self, sprite: SpriteData) -> usize {
        self.sprites.push(sprite);
        self.sprites.len() - 1
    }

    /// スプライト取得 (可変)
    ///
    /// ピクセルデータを変更したい場合はこのメソッドを使用。
    /// その後、Engine の `sync_texture()` を呼んで GPU に反映してください。
    pub fn get_sprite_mut(&mut self, id: usize) -> Option<&mut SpriteData> {
        self.sprites.get_mut(id)
    }

    /// スプライト取得 (読み取り専用)
    pub fn get_sprite(&self, id: usize) -> Option<&SpriteData> {
        self.sprites.get(id)
    }
}

impl Default for ResourcePackage {
    fn default() -> Self {
        Self::new()
    }
}
