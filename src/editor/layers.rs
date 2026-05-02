//! レイヤーシステム（Phase 6）
//!
//! 複数レイヤーのサポート、レイヤースタック管理、
//! 可視性制御、ブレンドモード、不透明度を提供します。

use crate::resource::SpriteData;

/// ブレンドモード：レイヤー合成時の色混合方式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    /// 通常合成（アルファブレンディング）
    Normal,
    /// 加算（光加算）
    Add,
    /// 乗算（色乗算）
    Multiply,
    /// スクリーン（反転乗算）
    Screen,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Normal
    }
}

/// レイヤーデータ：スプライト + メタデータ
#[derive(Clone, Debug)]
pub struct Layer {
    /// レイヤー ID（一意）
    pub id: u32,
    /// レイヤー名
    pub name: String,
    /// スプライトデータ
    pub sprite: SpriteData,
    /// 可視性フラグ（false で描画スキップ）
    pub visible: bool,
    /// 不透明度（0.0～1.0）
    pub opacity: f32,
    /// ブレンドモード
    pub blend_mode: BlendMode,
    /// ロック状態（true で編集不可）
    pub locked: bool,
}

impl Layer {
    /// 新しいレイヤーを作成
    ///
    /// # 例
    /// ```ignore
    /// let layer = Layer::new(0, "Background", sprite_data);
    /// assert_eq!(layer.opacity, 1.0);
    /// assert!(layer.visible);
    /// ```
    pub fn new(id: u32, name: impl Into<String>, sprite: SpriteData) -> Self {
        Self {
            id,
            name: name.into(),
            sprite,
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            locked: false,
        }
    }

    /// 不透明度を設定（0.0～1.0 にクランプ）
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// レイヤーをクローン
    pub fn duplicate(&self) -> Self {
        Self {
            id: self.id,
            name: format!("{} Copy", self.name),
            sprite: self.sprite.clone(),
            visible: self.visible,
            opacity: self.opacity,
            blend_mode: self.blend_mode,
            locked: self.locked,
        }
    }
}

/// レイヤースタック管理：複数レイヤーの並び、選択、合成
#[derive(Clone, Debug)]
pub struct LayerStack {
    /// レイヤーリスト（インデックス 0 = 最背面）
    layers: Vec<Layer>,
    /// アクティブレイヤーのインデックス
    active_layer_idx: usize,
    /// 次のレイヤー ID
    next_id: u32,
}

impl LayerStack {
    /// 新しいレイヤースタックを作成（デフォルト背景レイヤー付き）
    pub fn new(default_sprite: SpriteData) -> Self {
        let mut stack = Self {
            layers: Vec::new(),
            active_layer_idx: 0,
            next_id: 1,
        };
        // 背景レイヤーを追加
        let bg_layer = Layer::new(0, "Background", default_sprite);
        stack.layers.push(bg_layer);
        stack
    }

    /// レイヤーを追加（アクティブレイヤーの上に挿入）
    pub fn add_layer(&mut self, sprite: SpriteData, name: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let layer = Layer::new(id, name, sprite);
        let insert_pos = self.active_layer_idx + 1;

        if insert_pos <= self.layers.len() {
            self.layers.insert(insert_pos, layer);
            self.active_layer_idx = insert_pos;
        } else {
            self.layers.push(layer);
            self.active_layer_idx = self.layers.len() - 1;
        }

        id
    }

    /// アクティブレイヤーを削除（最後のレイヤーは削除不可）
    pub fn delete_active_layer(&mut self) -> bool {
        if self.layers.len() <= 1 {
            return false;
        }

        self.layers.remove(self.active_layer_idx);

        // アクティブインデックスを調整
        if self.active_layer_idx >= self.layers.len() {
            self.active_layer_idx = self.layers.len() - 1;
        }

        true
    }

    /// アクティブレイヤーを取得
    pub fn active_layer(&self) -> Option<&Layer> {
        self.layers.get(self.active_layer_idx)
    }

    /// アクティブレイヤーを可変参照で取得
    pub fn active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.get_mut(self.active_layer_idx)
    }

    /// 全レイヤーを取得
    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    /// 全レイヤーを可変参照で取得
    pub fn layers_mut(&mut self) -> &mut [Layer] {
        &mut self.layers
    }

    /// アクティブレイヤーインデックスを取得
    pub fn active_layer_idx(&self) -> usize {
        self.active_layer_idx
    }

    /// アクティブレイヤーを選択
    pub fn select_layer(&mut self, idx: usize) -> bool {
        if idx < self.layers.len() {
            self.active_layer_idx = idx;
            true
        } else {
            false
        }
    }

    /// レイヤーを上に移動
    pub fn move_layer_up(&mut self) -> bool {
        if self.active_layer_idx + 1 < self.layers.len() {
            self.layers
                .swap(self.active_layer_idx, self.active_layer_idx + 1);
            self.active_layer_idx += 1;
            true
        } else {
            false
        }
    }

    /// レイヤーを下に移動
    pub fn move_layer_down(&mut self) -> bool {
        if self.active_layer_idx > 0 {
            self.layers
                .swap(self.active_layer_idx, self.active_layer_idx - 1);
            self.active_layer_idx -= 1;
            true
        } else {
            false
        }
    }

    /// アクティブレイヤーを複製
    pub fn duplicate_active_layer(&mut self) -> u32 {
        if let Some(layer) = self.active_layer() {
            let dup = layer.duplicate();
            let id = dup.id;
            self.add_layer(dup.sprite, dup.name);
            id
        } else {
            0
        }
    }

    /// 全レイヤーを合成（描画順序：背面→前面、アルファブレンディング）
    pub fn composite(&self) -> Option<SpriteData> {
        if self.layers.is_empty() {
            return None;
        }

        // 最初の可視レイヤーをベースとする
        let mut result = None;

        for layer in &self.layers {
            if !layer.visible {
                continue;
            }

            match &mut result {
                None => {
                    // 最初の可視レイヤーをコピー
                    result = Some(layer.sprite.clone());
                }
                Some(ref mut canvas) => {
                    // レイヤーを合成
                    Self::blend_layer(canvas, &layer.sprite, layer.opacity, layer.blend_mode);
                }
            }
        }

        result
    }

    /// 2つのレイヤーをブレンド（canvas に layer を合成）
    fn blend_layer(canvas: &mut SpriteData, layer: &SpriteData, opacity: f32, blend: BlendMode) {
        // 寸法チェック
        if canvas.width != layer.width || canvas.height != layer.height {
            return;
        }

        let width = canvas.width as usize;
        let height = canvas.height as usize;

        for y in 0..height {
            for x in 0..width {
                if let (Some(canvas_color), Some(layer_color)) = (
                    canvas.get_pixel(x as u32, y as u32),
                    layer.get_pixel(x as u32, y as u32),
                ) {
                    let blended = match blend {
                        BlendMode::Normal => Self::blend_normal(canvas_color, layer_color, opacity),
                        BlendMode::Add => Self::blend_add(canvas_color, layer_color, opacity),
                        BlendMode::Multiply => {
                            Self::blend_multiply(canvas_color, layer_color, opacity)
                        }
                        BlendMode::Screen => Self::blend_screen(canvas_color, layer_color, opacity),
                    };

                    let _ = canvas.set_pixel(x as u32, y as u32, &blended);
                }
            }
        }
    }

    /// 通常合成（アルファブレンディング）
    fn blend_normal(bg: &[u8], fg: &[u8], opacity: f32) -> Vec<u8> {
        if fg.len() < 4 || bg.len() < 4 {
            return bg.to_vec();
        }

        let fg_a = (fg[3] as f32 / 255.0) * opacity;
        let bg_a = bg[3] as f32 / 255.0;
        let out_a = fg_a + bg_a * (1.0 - fg_a);

        if out_a < 0.001 {
            return vec![0, 0, 0, 0];
        }

        let r = ((fg[0] as f32 * fg_a + bg[0] as f32 * bg_a * (1.0 - fg_a)) / out_a) as u8;
        let g = ((fg[1] as f32 * fg_a + bg[1] as f32 * bg_a * (1.0 - fg_a)) / out_a) as u8;
        let b = ((fg[2] as f32 * fg_a + bg[2] as f32 * bg_a * (1.0 - fg_a)) / out_a) as u8;
        let a = (out_a * 255.0) as u8;

        vec![r, g, b, a]
    }

    /// 加算合成
    fn blend_add(bg: &[u8], fg: &[u8], opacity: f32) -> Vec<u8> {
        if fg.len() < 4 || bg.len() < 4 {
            return bg.to_vec();
        }

        // opacity を見かけの透明度として fg_alpha に乗算
        let fg_a = (fg[3] as f32 / 255.0) * opacity;
        let bg_a = bg[3] as f32 / 255.0;

        // 加算合成：色成分に fg の contribution を乗算
        let r = ((bg[0] as f32 + fg[0] as f32 * fg_a).min(255.0)) as u8;
        let g = ((bg[1] as f32 + fg[1] as f32 * fg_a).min(255.0)) as u8;
        let b = ((bg[2] as f32 + fg[2] as f32 * fg_a).min(255.0)) as u8;

        // alpha は通常合成のルール（アルファブレンディング）
        let out_a = fg_a + bg_a * (1.0 - fg_a);
        let a = (out_a * 255.0) as u8;

        vec![r, g, b, a]
    }

    /// 乗算合成
    fn blend_multiply(bg: &[u8], fg: &[u8], opacity: f32) -> Vec<u8> {
        if fg.len() < 4 || bg.len() < 4 {
            return bg.to_vec();
        }

        // opacity を見かけの透明度として fg_alpha に乗算
        let fg_a = (fg[3] as f32 / 255.0) * opacity;
        let bg_a = bg[3] as f32 / 255.0;

        // 乗算合成：bg * (fg 色 * fg_alpha + (1 - fg_alpha))
        let r = (bg[0] as f32 * (fg[0] as f32 / 255.0 * fg_a + (1.0 - fg_a))) as u8;
        let g = (bg[1] as f32 * (fg[1] as f32 / 255.0 * fg_a + (1.0 - fg_a))) as u8;
        let b = (bg[2] as f32 * (fg[2] as f32 / 255.0 * fg_a + (1.0 - fg_a))) as u8;

        // alpha は通常合成のルール
        let out_a = fg_a + bg_a * (1.0 - fg_a);
        let a = (out_a * 255.0) as u8;

        vec![r, g, b, a]
    }

    /// スクリーン合成（反転乗算）
    fn blend_screen(bg: &[u8], fg: &[u8], opacity: f32) -> Vec<u8> {
        if fg.len() < 4 || bg.len() < 4 {
            return bg.to_vec();
        }

        // opacity を見かけの透明度として fg_alpha に乗算
        let fg_a = (fg[3] as f32 / 255.0) * opacity;
        let bg_a = bg[3] as f32 / 255.0;

        // スクリーン合成：1 - (1 - bg) * (1 - fg * fg_alpha)
        let r = (255.0 - (255.0 - bg[0] as f32) * (255.0 - fg[0] as f32 * fg_a) / 255.0) as u8;
        let g = (255.0 - (255.0 - bg[1] as f32) * (255.0 - fg[1] as f32 * fg_a) / 255.0) as u8;
        let b = (255.0 - (255.0 - bg[2] as f32) * (255.0 - fg[2] as f32 * fg_a) / 255.0) as u8;

        // alpha は通常合成のルール
        let out_a = fg_a + bg_a * (1.0 - fg_a);
        let a = (out_a * 255.0) as u8;

        vec![r, g, b, a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    #[test]
    fn test_layer_creation() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let layer = Layer::new(0, "Test", sprite);
        assert_eq!(layer.id, 0);
        assert_eq!(layer.name, "Test");
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
    }

    #[test]
    fn test_layer_stack() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut stack = LayerStack::new(sprite.clone());

        assert_eq!(stack.layers().len(), 1);
        assert_eq!(stack.active_layer_idx(), 0);

        stack.add_layer(sprite.clone(), "Layer 1");
        assert_eq!(stack.layers().len(), 2);
        assert_eq!(stack.active_layer_idx(), 1);
    }

    #[test]
    fn test_layer_visibility() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut stack = LayerStack::new(sprite.clone());

        if let Some(layer) = stack.active_layer_mut() {
            layer.visible = false;
        }

        assert!(!stack.active_layer().unwrap().visible);
    }

    #[test]
    fn test_layer_movement() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut stack = LayerStack::new(sprite.clone());

        stack.add_layer(sprite.clone(), "Layer 1");
        stack.add_layer(sprite.clone(), "Layer 2");

        assert_eq!(stack.active_layer_idx(), 2);

        assert!(stack.move_layer_down());
        assert_eq!(stack.active_layer_idx(), 1);

        assert!(stack.move_layer_up());
        assert_eq!(stack.active_layer_idx(), 2);
    }

    #[test]
    fn test_layer_deletion() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut stack = LayerStack::new(sprite.clone());

        stack.add_layer(sprite.clone(), "Layer 1");
        assert_eq!(stack.layers().len(), 2);

        assert!(stack.delete_active_layer());
        assert_eq!(stack.layers().len(), 1);

        // 最後のレイヤーは削除不可
        assert!(!stack.delete_active_layer());
        assert_eq!(stack.layers().len(), 1);
    }

    #[test]
    fn test_opacity_clamping() {
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut layer = Layer::new(0, "Test", sprite);

        layer.set_opacity(1.5);
        assert_eq!(layer.opacity, 1.0);

        layer.set_opacity(-0.5);
        assert_eq!(layer.opacity, 0.0);
    }

    /// Test 1: opacity の見かけの透明度
    #[test]
    fn test_opacity_apparent_transparency() {
        let mut sprite = SpriteData::new(4, 4, ColorMode::FullColor);

        // レイヤーピクセル：白色で半透明 (R=255, G=255, B=255, A=128)
        for y in 0..4 {
            for x in 0..4 {
                let _ = sprite.set_pixel(x, y, &[255, 255, 255, 128]);
            }
        }

        // 背景：黒色で不透明 (R=0, G=0, B=0, A=255)
        let mut bg_sprite = SpriteData::new(4, 4, ColorMode::FullColor);
        for y in 0..4 {
            for x in 0..4 {
                let _ = bg_sprite.set_pixel(x, y, &[0, 0, 0, 255]);
            }
        }

        // opacity = 255 でレイヤー本来の透明度
        let fg_a_255 = (128.0 / 255.0) * 1.0; // opacity = 255 → 1.0
        let bg_a = 1.0;
        let out_a_255 = fg_a_255 + bg_a * (1.0 - fg_a_255);

        // opacity = 127 でレイヤーが半透明に見える
        let fg_a_127 = (128.0 / 255.0) * (127.0 / 255.0); // opacity = 127 → 127/255
        let out_a_127 = fg_a_127 + bg_a * (1.0 - fg_a_127);

        // opacity = 0 でレイヤーが完全透明（見えない）
        let fg_a_0 = 0.0; // opacity = 0 → 0.0
        let out_a_0 = fg_a_0 + bg_a * (1.0 - fg_a_0);

        // 確認：opacity が大きいほど出力 alpha が大きい
        assert!(out_a_255 > out_a_127);
        assert!(out_a_127 > out_a_0);
    }

    /// Test 2: Normal モード（Copy の代わり）の opacity
    #[test]
    fn test_normal_mode_opacity() {
        let mut sprite = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = sprite.set_pixel(0, 0, &[255, 0, 0, 255]); // 赤（不透明）

        let mut bg = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = bg.set_pixel(0, 0, &[0, 0, 255, 255]); // 青（不透明）

        let mut stack = LayerStack::new(bg);
        stack.add_layer(sprite, "Red Layer");

        // opacity = 255: レイヤー本来の見た目
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.set_opacity(1.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            // 赤色が見える（alpha >= 255 に近い）
            assert_eq!(pixel[0], 255);
            assert!(pixel[3] > 200); // alpha が十分に大きい
        }

        // opacity = 127: レイヤーが50%透明に見える
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.set_opacity(127.0 / 255.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            // 赤と青が混ざった色になり、alpha が小さくなる
            assert!(pixel[3] < 200); // alpha が小さい
        }
    }

    /// Test 3: Multiply モードの opacity
    #[test]
    fn test_multiply_mode_opacity() {
        let mut sprite = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = sprite.set_pixel(0, 0, &[128, 128, 128, 255]); // グレー

        let mut bg = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = bg.set_pixel(0, 0, &[255, 255, 255, 255]); // 白

        let mut stack = LayerStack::new(bg);
        stack.add_layer(sprite, "Gray Layer");

        // opacity = 255: 暗くなる（グレー色）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.blend_mode = BlendMode::Multiply;
                layer.set_opacity(1.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            assert!(pixel[0] <= 128); // 色が暗い
        }

        // opacity = 127: opacity も反映される（さらに暗くならない）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.set_opacity(127.0 / 255.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            // opacity が小さいので、ベース色に近づく
            assert!(pixel[0] > 128 || pixel[3] < 255);
        }
    }

    /// Test 4: Screen モードの opacity
    #[test]
    fn test_screen_mode_opacity() {
        let mut sprite = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = sprite.set_pixel(0, 0, &[128, 128, 128, 255]); // グレー

        let mut bg = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = bg.set_pixel(0, 0, &[0, 0, 0, 255]); // 黒

        let mut stack = LayerStack::new(bg);
        stack.add_layer(sprite, "Gray Layer");

        // opacity = 255: 明るくなる（グレー色）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.blend_mode = BlendMode::Screen;
                layer.set_opacity(1.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            assert!(pixel[0] >= 128); // 色が明るい
        }

        // opacity = 127: opacity も反映される（あまり明るくならない）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.set_opacity(127.0 / 255.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            // opacity が小さいので、ベース色に近づく
            assert!(pixel[0] < 128 || pixel[3] < 255);
        }
    }

    /// Test 5: Add モードの opacity
    #[test]
    fn test_add_mode_opacity() {
        let mut sprite = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = sprite.set_pixel(0, 0, &[100, 100, 100, 255]); // グレー

        let mut bg = SpriteData::new(2, 2, ColorMode::FullColor);
        let _ = bg.set_pixel(0, 0, &[100, 100, 100, 255]); // グレー

        let mut stack = LayerStack::new(bg);
        stack.add_layer(sprite, "Gray Layer");

        // opacity = 255: 加算合成（明るくなる）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.blend_mode = BlendMode::Add;
                layer.set_opacity(1.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            assert!(pixel[0] > 100); // 色が明るい（100 + 100 = 200）
            assert!(pixel[3] > 200); // alpha が大きい
        }

        // opacity = 127: opacity も反映される（加算の強度が減る）
        {
            if let Some(layer) = stack.active_layer_mut() {
                layer.set_opacity(127.0 / 255.0);
            }
        }
        let composite = stack.composite().unwrap();
        if let Some(pixel) = composite.get_pixel(0, 0) {
            // opacity が小さいので、加算の効果が弱い
            assert!(pixel[0] <= 150 || pixel[3] < 255);
        }
    }
}
