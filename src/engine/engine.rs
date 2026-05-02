use crate::resource::{ColorMode, ResourcePackage, SpriteData};
use macroquad::prelude::*;

/// ゲームエンジン: リソース管理とGPU描画を統合
///
/// `Engine` はスプライトデータとGPUテクスチャを管理し、画面に描画するコア機能を提供します。
/// 256色パレットモード（Indexed256）とフルカラーモード（RGBA）の両方に対応しています。
///
/// # 主な機能
///
/// - **スプライト管理**: `add_sprite()` でスプライトを登録
/// - **ピクセル編集**: `set_pixel()` でピクセルを変更（自動GPU同期）
/// - **描画**: `draw_sprite()` で画面に描画
/// - **GPU同期**: CPU上のピクセルデータをGPUテクスチャに自動で反映
///
/// # 使用例
///
/// ```ignore
/// let mut engine = Engine::new();
///
/// // FullColor モードでスプライトを作成
/// let mut sprite = SpriteData::new(32, 32, ColorMode::FullColor);
/// sprite.set_pixel(0, 0, &[255, 0, 0, 255])?; // 赤色
///
/// // エンジンに追加（自動でテクスチャ化）
/// let sprite_id = engine.add_sprite(sprite);
///
/// // ピクセルを編集（自動で GPU に同期）
/// engine.set_pixel(sprite_id, 5, 5, &[0, 255, 0, 255])?;
///
/// // 画面に描画
/// engine.draw_sprite(sprite_id, 100.0, 100.0)?;
/// ```
pub struct Engine {
    /// スプライトリソース（CPU側）
    pub res: ResourcePackage,
    /// GPU テクスチャ（描画用）
    pub textures: Vec<Texture2D>,
}

impl Engine {
    /// 新規エンジンインスタンスの作成
    ///
    /// 空のスプライトリストとテクスチャリストで初期化します。
    pub fn new() -> Self {
        Self {
            res: ResourcePackage::new(),
            textures: vec![],
        }
    }

    /// スプライトを追加し、テクスチャを初期化
    ///
    /// 渡された `SpriteData` をリソースに追加し、自動的に GPU テクスチャに同期します。
    /// テクスチャフィルタは `FilterMode::Nearest` に設定されます（ドット絵用）。
    ///
    /// # 引数
    /// - `sprite`: 追加するスプライトデータ
    ///
    /// # 戻り値
    /// 割り当てられたスプライト ID（以降の操作で使用）
    ///
    /// # 例
    /// ```ignore
    /// let sprite = SpriteData::new(16, 16, ColorMode::FullColor);
    /// let sprite_id = engine.add_sprite(sprite);
    /// ```
    pub fn add_sprite(&mut self, sprite: SpriteData) -> usize {
        let sprite_id = self.res.add_sprite(sprite);
        self.sync_texture(sprite_id);
        sprite_id
    }

    /// ピクセルを書き換え、GPUに即時同期
    ///
    /// 指定されたスプライトのピクセルを変更し、自動的に GPU テクスチャに反映します。
    /// ピクセル値はカラーモードに応じて異なります：
    /// - **Indexed256**: 1 バイト（パレットインデックス 0-255）
    /// - **FullColor**: 4 バイト（RGBA）
    ///
    /// # 引数
    /// - `sprite_id`: 対象スプライトの ID
    /// - `x`: X座標（0 から幅-1）
    /// - `y`: Y座標（0 から高さ-1）
    /// - `value`: ピクセル値（カラーモード対応）
    ///
    /// # 戻り値
    /// - `Ok(())`: 成功
    /// - `Err(String)`: スプライト ID が無効、座標が範囲外、ピクセル値のサイズが不正
    ///
    /// # 例
    /// ```ignore
    /// // FullColor: RGBA
    /// engine.set_pixel(sprite_id, 10, 10, &[255, 128, 64, 255])?;
    ///
    /// // Indexed256: パレットインデックス
    /// engine.set_pixel(sprite_id, 10, 10, &[2])?;
    /// ```
    pub fn set_pixel(
        &mut self,
        sprite_id: usize,
        x: u32,
        y: u32,
        value: &[u8],
    ) -> Result<(), String> {
        if sprite_id >= self.res.sprites.len() {
            return Err(format!("Sprite ID {} out of range", sprite_id));
        }

        let sprite = &mut self.res.sprites[sprite_id];
        sprite.set_pixel(x, y, value)?;

        // GPU側テクスチャの更新
        self.sync_texture(sprite_id);
        Ok(())
    }

    /// ピクセルを読み取る（読み取り専用）
    ///
    /// 指定されたピクセルの値を取得します。
    ///
    /// # 引数
    /// - `sprite_id`: スプライト ID
    /// - `x`: X座標
    /// - `y`: Y座標
    ///
    /// # 戻り値
    /// - `Some(Vec<u8>)`: ピクセル値
    /// - `None`: スプライト ID が無効または座標が範囲外
    pub fn get_pixel(&self, sprite_id: usize, x: u32, y: u32) -> Option<Vec<u8>> {
        self.res
            .sprites
            .get(sprite_id)
            .and_then(|sprite| sprite.get_pixel(x, y))
            .map(|slice| slice.to_vec())
    }

    /// CPU上の画像データをMacroquadテクスチャに変換して同期
    ///
    /// スプライトのピクセルデータをGPU用テクスチャに変換します。
    /// 以下の処理を実行：
    /// - Indexed256 モード: パレットを参照してRGBAに変換
    /// - FullColor モード: ピクセルをそのままコピー
    /// - FilterMode を Nearest に設定（ドット絵の品質保証）
    ///
    /// # 注意
    /// このメソッドは `add_sprite()` と `set_pixel()` から自動で呼ばれるため、
    /// 通常はユーザーが直接呼ぶ必要はありません。
    pub fn sync_texture(&mut self, id: usize) {
        if id >= self.res.sprites.len() {
            return;
        }

        let sprite = &self.res.sprites[id];
        let image = match &sprite.mode {
            ColorMode::FullColor => Image {
                width: sprite.width as u16,
                height: sprite.height as u16,
                bytes: sprite.pixels.clone(),
            },
            ColorMode::Indexed256(palette) => {
                // インデックスからRGBAへの変換
                let mut rgba = Vec::with_capacity(sprite.pixels.len() * 4);
                for &idx in &sprite.pixels {
                    let color = palette.get(idx as usize).copied().unwrap_or([0, 0, 0, 0]);
                    rgba.extend_from_slice(&color);
                }
                Image {
                    width: sprite.width as u16,
                    height: sprite.height as u16,
                    bytes: rgba,
                }
            }
        };

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        if id < self.textures.len() {
            self.textures[id] = texture;
        } else {
            self.textures.push(texture);
        }
    }

    /// スプライトを描画
    ///
    /// 指定されたスプライトをスクリーン座標に描画します。
    ///
    /// # 引数
    /// - `sprite_id`: スプライト ID
    /// - `x`: スクリーン上の X座標
    /// - `y`: スクリーン上の Y座標
    ///
    /// # 戻り値
    /// - `Ok(())`: 描画成功
    /// - `Err(String)`: テクスチャが初期化されていない
    ///
    /// # 例
    /// ```ignore
    /// engine.draw_sprite(sprite_id, 100.0, 50.0)?;
    /// ```
    pub fn draw_sprite(&self, sprite_id: usize, x: f32, y: f32) -> Result<(), String> {
        if sprite_id >= self.textures.len() {
            return Err(format!("Texture for sprite {} not initialized", sprite_id));
        }

        draw_texture(&self.textures[sprite_id], x, y, WHITE);
        Ok(())
    }

    /// スプライトをスケール付きで描画
    ///
    /// 拡大・縮小して描画します。
    ///
    /// # 引数
    /// - `sprite_id`: スプライト ID
    /// - `x`: スクリーン上の X座標
    /// - `y`: スクリーン上の Y座標
    /// - `scale`: スケール倍率（1.0 = 等倍、2.0 = 2倍、0.5 = 半分）
    ///
    /// # 戻り値
    /// - `Ok(())`: 描画成功
    /// - `Err(String)`: テクスチャが初期化されていない
    ///
    /// # 例
    /// ```ignore
    /// engine.draw_sprite_scaled(sprite_id, 100.0, 50.0, 2.5)?;
    /// ```
    pub fn draw_sprite_scaled(
        &self,
        sprite_id: usize,
        x: f32,
        y: f32,
        scale: f32,
    ) -> Result<(), String> {
        if sprite_id >= self.textures.len() {
            return Err(format!("Texture for sprite {} not initialized", sprite_id));
        }

        let sprite = &self.res.sprites[sprite_id];
        draw_texture_ex(
            &self.textures[sprite_id],
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    sprite.width as f32 * scale,
                    sprite.height as f32 * scale,
                )),
                ..Default::default()
            },
        );
        Ok(())
    }

    /// スプライト数を取得
    ///
    /// 現在登録されているスプライトの総数を返します。
    pub fn sprite_count(&self) -> usize {
        self.res.sprites.len()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
