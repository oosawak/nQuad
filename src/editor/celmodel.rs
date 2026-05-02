//! Cel モデル（Phase 6.5）
//!
//! レイヤー構造と cel データの分離。
//! - LayerDef: 安定した層メタデータ（ID、名前など）
//! - Cel: 単一レイヤーのピクセルデータ
//! - Frame: フレームは per-layer cel を格納（LayerStack は不要）

use crate::resource::SpriteData;
use std::collections::HashMap;

/// レイヤーメタデータ（安定的）
///
/// ドキュメント全体で共有。フレーム間で変わらない層定義。
#[derive(Clone, Debug)]
pub struct LayerDef {
    /// レイヤー ID（一意）
    pub id: u32,
    /// レイヤー名
    pub name: String,
    /// 初期不透明度
    pub default_opacity: f32,
    /// 初期ブレンドモード
    pub default_blend: crate::editor::BlendMode,
}

impl LayerDef {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            default_opacity: 1.0,
            default_blend: crate::editor::BlendMode::Normal,
        }
    }
}

/// Cel：単一レイヤーのピクセルデータ
///
/// Frame が複数の Cel を所有。
/// Cel は read-only の SpriteData + メタデータ。
#[derive(Clone, Debug)]
pub struct Cel {
    /// この Cel が属するレイヤー ID
    pub layer_id: u32,
    /// ピクセルデータ
    pub pixels: SpriteData,
    /// フレーム内での可視性（グローバルではなくこのフレーム限定）
    pub visible: bool,
    /// フレーム内での不透明度上書き（None = default を使う）
    pub opacity_override: Option<f32>,
    /// フレーム内でのブレンドモード上書き（None = default を使う）
    pub blend_override: Option<crate::editor::BlendMode>,
}

impl Cel {
    pub fn new(layer_id: u32, pixels: SpriteData) -> Self {
        Self {
            layer_id,
            pixels,
            visible: true,
            opacity_override: None,
            blend_override: None,
        }
    }

    /// このフレームで有効な不透明度を取得
    pub fn effective_opacity(&self, default: f32) -> f32 {
        self.opacity_override.unwrap_or(default)
    }

    /// このフレームで有効なブレンドモードを取得
    pub fn effective_blend(&self, default: crate::editor::BlendMode) -> crate::editor::BlendMode {
        self.blend_override.unwrap_or(default)
    }
}

/// フレーム（Cel モデル版）
///
/// レイヤースタックを所有せず、per-layer cel を HashMap で管理。
#[derive(Clone, Debug)]
pub struct nQFrame {
    pub frame_num: u32,
    pub duration_ms: u32,
    /// layer_id -> Cel のマッピング
    pub cels: HashMap<u32, Cel>,
}

impl nQFrame {
    pub fn new(frame_num: u32, duration_ms: u32) -> Self {
        Self {
            frame_num,
            duration_ms,
            cels: HashMap::new(),
        }
    }

    /// Cel を追加
    pub fn add_cel(&mut self, cel: Cel) {
        self.cels.insert(cel.layer_id, cel);
    }

    /// Cel を取得
    pub fn get_cel(&self, layer_id: u32) -> Option<&Cel> {
        self.cels.get(&layer_id)
    }

    /// Cel を可変参照で取得
    pub fn get_cel_mut(&mut self, layer_id: u32) -> Option<&mut Cel> {
        self.cels.get_mut(&layer_id)
    }

    /// Cel を削除
    pub fn remove_cel(&mut self, layer_id: u32) -> Option<Cel> {
        self.cels.remove(&layer_id)
    }

    /// このフレーム内の全 Cel を列挙
    pub fn cels(&self) -> impl Iterator<Item = &Cel> {
        self.cels.values()
    }

    /// 可視 Cel のみを列挙
    pub fn visible_cels(&self) -> impl Iterator<Item = &Cel> {
        self.cels.values().filter(|c| c.visible)
    }

    /// レイヤー ID のリストを取得（ソート済み）
    pub fn layer_ids(&self) -> Vec<u32> {
        let mut ids: Vec<_> = self.cels.keys().copied().collect();
        ids.sort();
        ids
    }

    /// セルをクローン（新規フレーム作成用）
    pub fn duplicate(&self) -> Self {
        Self {
            frame_num: self.frame_num,
            duration_ms: self.duration_ms,
            cels: self.cels.clone(),
        }
    }

    /// フレーム内で全層を composite（元のレイヤー定義を参考に）
    pub fn composite_with_defs(&self, layer_defs: &[LayerDef]) -> Result<SpriteData, String> {
        if self.cels.is_empty() {
            return Err("No cels in frame".to_string());
        }

        // 最初の可視 Cel をベースとする
        let mut result = None;

        // レイヤー定義の順序でコンポーズ
        for layer_def in layer_defs {
            if let Some(cel) = self.get_cel(layer_def.id) {
                if !cel.visible {
                    continue;
                }

                let opacity = cel.effective_opacity(layer_def.default_opacity);
                let blend = cel.effective_blend(layer_def.default_blend);

                match &mut result {
                    None => {
                        // 最初の可視 cel
                        result = Some(cel.pixels.clone());
                    }
                    Some(ref mut canvas) => {
                        // 合成
                        Self::blend_sprites(canvas, &cel.pixels, opacity, blend)?;
                    }
                }
            }
        }

        result.ok_or_else(|| "No visible cels".to_string())
    }

    /// 2 つのスプライトをブレンド（簡易版）
    fn blend_sprites(
        canvas: &mut SpriteData,
        layer: &SpriteData,
        opacity: f32,
        _blend: crate::editor::BlendMode,
    ) -> Result<(), String> {
        if canvas.width != layer.width || canvas.height != layer.height {
            return Err("Sprite dimensions must match".to_string());
        }

        let width = canvas.width as usize;
        let height = canvas.height as usize;

        for y in 0..height {
            for x in 0..width {
                if let (Some(bg), Some(fg)) = (
                    canvas.get_pixel(x as u32, y as u32),
                    layer.get_pixel(x as u32, y as u32),
                ) {
                    // 簡易アルファ合成（Normal ブレンド）
                    let fg_a = (fg[3] as f32 / 255.0) * opacity;
                    let bg_a = bg[3] as f32 / 255.0;
                    let out_a = fg_a + bg_a * (1.0 - fg_a);

                    if out_a > 0.001 {
                        let r = ((fg[0] as f32 * fg_a + bg[0] as f32 * bg_a * (1.0 - fg_a)) / out_a)
                            as u8;
                        let g = ((fg[1] as f32 * fg_a + bg[1] as f32 * bg_a * (1.0 - fg_a)) / out_a)
                            as u8;
                        let b = ((fg[2] as f32 * fg_a + bg[2] as f32 * bg_a * (1.0 - fg_a)) / out_a)
                            as u8;
                        let a = (out_a * 255.0) as u8;

                        canvas.set_pixel(x as u32, y as u32, &[r, g, b, a])?;
                    } else {
                        canvas.set_pixel(x as u32, y as u32, &[0, 0, 0, 0])?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    #[test]
    fn test_cel_creation() {
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        let cel = Cel::new(0, pixels);

        assert_eq!(cel.layer_id, 0);
        assert!(cel.visible);
        assert!(cel.opacity_override.is_none());
    }

    #[test]
    fn test_frame_add_cel() {
        let mut frame = nQFrame::new(0, 100);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        let cel = Cel::new(0, pixels);

        frame.add_cel(cel);
        assert_eq!(frame.cels.len(), 1);
        assert!(frame.get_cel(0).is_some());
    }

    #[test]
    fn test_frame_layer_ids() {
        let mut frame = nQFrame::new(0, 100);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);

        frame.add_cel(Cel::new(2, pixels.clone()));
        frame.add_cel(Cel::new(0, pixels.clone()));
        frame.add_cel(Cel::new(1, pixels));

        assert_eq!(frame.layer_ids(), vec![0, 1, 2]);
    }

    #[test]
    fn test_effective_opacity() {
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        let mut cel = Cel::new(0, pixels);

        assert_eq!(cel.effective_opacity(1.0), 1.0);

        cel.opacity_override = Some(0.5);
        assert_eq!(cel.effective_opacity(1.0), 0.5);
    }
}
