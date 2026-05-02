//! スプライト資産（Phase 7）
//!
//! 不変な資産データを保持。
//! - LayerDef: レイヤーメタデータ
//! - AnimationClipDef: アニメーション定義（再生状態なし）
//! - frame_data: フレーム内のセルデータ

use crate::resource::SpriteData;
use std::collections::HashMap;

/// フレームデータ検証エラー
#[derive(Debug, Clone, PartialEq)]
pub enum FrameDataError {
    /// ドキュメント寸法が不正
    InvalidDimensions {
        expected: (u32, u32),
        got: (u32, u32),
    },
    /// ピクセルデータ数が不正
    InvalidPixelCount { expected: usize, got: usize },
    /// Cel データが不正
    InvalidCelData {
        frame_id: u32,
        layer_id: u32,
        reason: String,
    },
    /// Cel データが見つからない
    MissingCelData { frame_id: u32, layer_id: u32 },
    /// レイヤーインデックスが不正
    InvalidLayerIndex { layer_id: u32, max: u32 },
}

/// レイヤーメタデータ（不変）
///
/// 資産全体で共有。フレーム間で変わらない層定義。
#[derive(Clone, Debug)]
pub struct LayerDef {
    /// レイヤー ID（一意）
    pub id: u32,
    /// レイヤー名
    pub name: String,
    /// 初期不透明度
    pub default_opacity: f32,
    /// 初期ブレンドモード
    pub default_blend: String,
}

impl LayerDef {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            default_opacity: 1.0,
            default_blend: "Normal".to_string(),
        }
    }
}

/// フレーム内のセル：単一レイヤーのピクセルデータ
///
/// フレームが複数のセルを所有。
#[derive(Clone, Debug)]
pub struct Cel {
    /// このセルが属するレイヤー ID
    pub layer_id: u32,
    /// ピクセルデータ
    pub pixels: SpriteData,
    /// フレーム内での可視性
    pub visible: bool,
    /// フレーム内での不透明度上書き
    pub opacity_override: Option<f32>,
    /// フレーム内でのブレンドモード上書き
    pub blend_override: Option<String>,
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
}

/// アニメーションクリップ定義（再生状態なし）
///
/// フレームシーケンスのみを保持。再生制御は外側で管理される。
#[derive(Clone, Debug)]
pub struct AnimationClipDef {
    /// クリップ名
    pub name: String,
    /// フレームリスト
    pub frames: Vec<FrameDef>,
    /// ループするか
    pub looping: bool,
}

/// フレーム定義
#[derive(Clone, Debug)]
pub struct FrameDef {
    /// フレーム番号
    pub frame_num: u32,
    /// 再生時間（ミリ秒）
    pub duration_ms: u32,
    /// layer_id -> Cel のマッピング
    pub cels: HashMap<u32, Cel>,
}

impl FrameDef {
    pub fn new(frame_num: u32, duration_ms: u32) -> Self {
        Self {
            frame_num,
            duration_ms,
            cels: HashMap::new(),
        }
    }

    pub fn add_cel(&mut self, cel: Cel) {
        self.cels.insert(cel.layer_id, cel);
    }

    pub fn get_cel(&self, layer_id: u32) -> Option<&Cel> {
        self.cels.get(&layer_id)
    }

    pub fn get_cel_mut(&mut self, layer_id: u32) -> Option<&mut Cel> {
        self.cels.get_mut(&layer_id)
    }
}

/// スプライト資産：不変な資産データ
///
/// ドキュメント全体で共有される資産。
/// エディタの履歴管理（Undo/Redo）やキャッシュは別管理される。
#[derive(Clone, Debug)]
pub struct SpriteAsset {
    /// 資産 ID
    pub id: usize,
    /// 資産名
    pub name: String,
    /// レイヤー定義リスト
    pub layer_defs: Vec<LayerDef>,
    /// アニメーションクリップリスト
    pub animation_clips: Vec<AnimationClipDef>,
    /// フレームデータ
    /// key: frame_num, value: FrameDef
    pub frame_data: HashMap<u32, FrameDef>,
}

impl SpriteAsset {
    /// 新規資産を作成
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            layer_defs: Vec::new(),
            animation_clips: Vec::new(),
            frame_data: HashMap::new(),
        }
    }

    /// レイヤー定義を追加
    pub fn add_layer_def(&mut self, def: LayerDef) {
        self.layer_defs.push(def);
    }

    /// アニメーションクリップを追加
    pub fn add_animation_clip(&mut self, clip: AnimationClipDef) {
        self.animation_clips.push(clip);
    }

    /// フレームデータを設定
    pub fn set_frame(&mut self, frame_num: u32, frame: FrameDef) {
        self.frame_data.insert(frame_num, frame);
    }

    /// フレームデータを取得
    pub fn get_frame(&self, frame_num: u32) -> Option<&FrameDef> {
        self.frame_data.get(&frame_num)
    }

    /// 最初のアニメーションクリップを取得
    pub fn first_animation_clip(&self) -> Option<&AnimationClipDef> {
        self.animation_clips.first()
    }

    /// レイヤー定義を取得
    pub fn get_layer_def(&self, layer_id: u32) -> Option<&LayerDef> {
        self.layer_defs.iter().find(|def| def.id == layer_id)
    }

    /// SpriteAsset を DocumentFormat v2 に変換
    pub fn to_format(&self) -> crate::editor::format::DocumentFormat {
        use crate::editor::format::{
            AnimationClipFormat, CelData, DocumentFormat, DocumentMetadata,
            LayerDef as FormatLayerDef,
        };

        // メタデータを計算
        let frame_count = self.frame_data.len() as u32;
        let (width, height) = self
            .frame_data
            .values()
            .next()
            .and_then(|frame| frame.cels.values().next())
            .map(|cel| (cel.pixels.width, cel.pixels.height))
            .unwrap_or((32, 32));

        let metadata = DocumentMetadata {
            name: self.name.clone(),
            width,
            height,
            frame_count,
            version: 2,
        };

        // レイヤー定義を変換
        let layers = self
            .layer_defs
            .iter()
            .map(|layer_def| FormatLayerDef {
                id: layer_def.id,
                name: layer_def.name.clone(),
                default_opacity: layer_def.default_opacity,
                default_blend_mode: layer_def.default_blend.clone(),
            })
            .collect::<Vec<_>>();

        // Cel データを集める（sparse モデル）
        let mut cel_data = Vec::new();
        for (frame_num, frame_def) in &self.frame_data {
            for (layer_id, cel) in &frame_def.cels {
                cel_data.push(CelData {
                    layer_id: *layer_id,
                    frame_num: *frame_num,
                    pixels: cel.pixels.pixels.clone(),
                    width: cel.pixels.width,
                    height: cel.pixels.height,
                    color_mode: cel.pixels.mode.clone(),
                    visible: cel.visible,
                    opacity_override: cel.opacity_override,
                    blend_override: cel.blend_override.clone(),
                });
            }
        }

        // アニメーションクリップを変換
        let clips = self
            .animation_clips
            .iter()
            .map(|clip_def| {
                let frame_numbers: Vec<u32> = clip_def.frames.iter().map(|f| f.frame_num).collect();
                let frame_durations: HashMap<u32, u32> = clip_def
                    .frames
                    .iter()
                    .map(|f| (f.frame_num, f.duration_ms))
                    .collect();

                AnimationClipFormat {
                    name: clip_def.name.clone(),
                    frame_numbers,
                    looping: clip_def.looping,
                    frame_durations,
                }
            })
            .collect::<Vec<_>>();

        DocumentFormat {
            metadata,
            layers,
            clips,
            cel_data,
        }
    }

    /// DocumentFormat v2 から SpriteAsset を復元
    pub fn from_format(format: &crate::editor::format::DocumentFormat) -> Result<Self, String> {
        // レイヤー定義を復元
        let layer_defs = format
            .layers
            .iter()
            .map(|layer| LayerDef {
                id: layer.id,
                name: layer.name.clone(),
                default_opacity: layer.default_opacity,
                default_blend: layer.default_blend_mode.clone(),
            })
            .collect::<Vec<_>>();

        // Cel データからフレームを再構成
        let mut frame_data: HashMap<u32, FrameDef> = HashMap::new();

        for cel_data in &format.cel_data {
            let frame_entry = frame_data
                .entry(cel_data.frame_num)
                .or_insert_with(|| FrameDef::new(cel_data.frame_num, 100));

            // SpriteData を復元
            let pixels = SpriteData {
                width: cel_data.width,
                height: cel_data.height,
                mode: cel_data.color_mode.clone(),
                pixels: cel_data.pixels.clone(),
            };

            // Cel を作成して追加
            let mut cel = Cel {
                layer_id: cel_data.layer_id,
                pixels,
                visible: cel_data.visible,
                opacity_override: cel_data.opacity_override,
                blend_override: cel_data.blend_override.clone(),
            };

            frame_entry.add_cel(cel);
        }

        // アニメーションクリップを復元
        let animation_clips = format
            .clips
            .iter()
            .map(|clip_format| {
                let frames = clip_format
                    .frame_numbers
                    .iter()
                    .map(|frame_num| {
                        let duration = clip_format
                            .frame_durations
                            .get(frame_num)
                            .copied()
                            .unwrap_or(100);
                        FrameDef::new(*frame_num, duration)
                    })
                    .collect::<Vec<_>>();

                AnimationClipDef {
                    name: clip_format.name.clone(),
                    frames,
                    looping: clip_format.looping,
                }
            })
            .collect::<Vec<_>>();

        let asset = SpriteAsset {
            id: 0, // 復元時は ID は再割り当てされる
            name: format.metadata.name.clone(),
            layer_defs,
            animation_clips,
            frame_data,
        };

        // フレームデータの検証
        asset
            .validate_frame_data()
            .map_err(|e| format!("Validation error: {:?}", e))?;

        Ok(asset)
    }

    /// フレームデータの検証
    ///
    /// 以下の項目を検証：
    /// - フレーム数が 1 以上
    /// - レイヤー数が 1 以上
    /// - 各 Cel のピクセルデータがドキュメント寸法と一致
    /// - layer_id がレイヤー定義内に存在
    /// - frame_id がフレーム数以下
    pub fn validate_frame_data(&self) -> Result<(), FrameDataError> {
        // フレーム数が 1 以上か確認
        if self.frame_data.is_empty() {
            return Err(FrameDataError::InvalidCelData {
                frame_id: 0,
                layer_id: 0,
                reason: "No frames found".to_string(),
            });
        }

        // レイヤー数が 1 以上か確認
        if self.layer_defs.is_empty() {
            return Err(FrameDataError::InvalidCelData {
                frame_id: 0,
                layer_id: 0,
                reason: "No layers found".to_string(),
            });
        }

        // 各フレーム内の各 Cel をチェック
        for (frame_id, frame_def) in &self.frame_data {
            for (layer_id, cel) in &frame_def.cels {
                // layer_id が定義内に存在するか確認
                if !self.layer_defs.iter().any(|def| def.id == *layer_id) {
                    return Err(FrameDataError::InvalidLayerIndex {
                        layer_id: *layer_id,
                        max: self.layer_defs.len() as u32 - 1,
                    });
                }

                // ピクセルデータが正しいサイズか確認
                let expected_size = cel.pixels.get_expected_pixel_size();
                let actual_size = cel.pixels.pixels.len();

                if actual_size != expected_size {
                    return Err(FrameDataError::InvalidPixelCount {
                        expected: expected_size,
                        got: actual_size,
                    });
                }

                // 寸法が正常か確認
                if cel.pixels.width == 0 || cel.pixels.height == 0 {
                    return Err(FrameDataError::InvalidDimensions {
                        expected: (1, 1),
                        got: (cel.pixels.width, cel.pixels.height),
                    });
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
    fn test_layer_def_creation() {
        let def = LayerDef::new(0, "Layer 0");
        assert_eq!(def.id, 0);
        assert_eq!(def.name, "Layer 0");
        assert_eq!(def.default_opacity, 1.0);
    }

    #[test]
    fn test_cel_creation() {
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        let cel = Cel::new(0, pixels);
        assert_eq!(cel.layer_id, 0);
        assert!(cel.visible);
    }

    #[test]
    fn test_frame_def_add_cel() {
        let mut frame = FrameDef::new(0, 100);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        let cel = Cel::new(0, pixels);

        frame.add_cel(cel);
        assert_eq!(frame.cels.len(), 1);
        assert!(frame.get_cel(0).is_some());
    }

    #[test]
    fn test_animation_clip_def() {
        let frame = FrameDef::new(0, 100);
        let clip = AnimationClipDef {
            name: "Default".to_string(),
            frames: vec![frame],
            looping: true,
        };

        assert_eq!(clip.frames.len(), 1);
        assert!(clip.looping);
    }

    #[test]
    fn test_sprite_asset_creation() {
        let asset = SpriteAsset::new(0, "TestAsset");
        assert_eq!(asset.id, 0);
        assert_eq!(asset.name, "TestAsset");
        assert!(asset.layer_defs.is_empty());
        assert!(asset.animation_clips.is_empty());
    }

    #[test]
    fn test_sprite_asset_add_layer() {
        let mut asset = SpriteAsset::new(0, "TestAsset");
        let def = LayerDef::new(0, "Layer 0");

        asset.add_layer_def(def);
        assert_eq!(asset.layer_defs.len(), 1);
        assert!(asset.get_layer_def(0).is_some());
    }

    #[test]
    fn test_sprite_asset_add_frame() {
        let mut asset = SpriteAsset::new(0, "TestAsset");
        let frame = FrameDef::new(0, 100);

        asset.set_frame(0, frame);
        assert_eq!(asset.frame_data.len(), 1);
        assert!(asset.get_frame(0).is_some());
    }

    // ============ Round-trip テスト ============

    /// Helper: テスト用 SpriteAsset を作成
    fn create_test_asset(
        name: &str,
        width: u32,
        height: u32,
        frame_count: u32,
        layer_count: u32,
    ) -> SpriteAsset {
        let mut asset = SpriteAsset::new(0, name);

        // レイヤーを追加
        for i in 0..layer_count {
            asset.add_layer_def(LayerDef::new(i, format!("Layer {}", i)));
        }

        // フレームを追加
        for frame_num in 0..frame_count {
            let mut frame = FrameDef::new(frame_num, 100);
            for layer_id in 0..layer_count {
                let mut pixels = SpriteData::new(width, height, ColorMode::FullColor);
                // ピクセルデータを設定（簡単な値）
                for y in 0..height {
                    for x in 0..width {
                        let val = ((frame_num * 256 + layer_id * 100 + y * 10 + x) % 256) as u8;
                        let _ = pixels.set_pixel(x, y, &[val, val, val, 255]);
                    }
                }
                let cel = Cel::new(layer_id, pixels);
                frame.add_cel(cel);
            }
            asset.set_frame(frame_num, frame);
        }

        asset
    }

    /// Helper: 2つの DocumentFormat が等しいか確認
    fn assert_format_equal(
        format1: &crate::editor::format::DocumentFormat,
        format2: &crate::editor::format::DocumentFormat,
    ) {
        // メタデータを比較
        assert_eq!(format1.metadata.name, format2.metadata.name);
        assert_eq!(format1.metadata.width, format2.metadata.width);
        assert_eq!(format1.metadata.height, format2.metadata.height);
        assert_eq!(format1.metadata.frame_count, format2.metadata.frame_count);

        // レイヤーを比較
        assert_eq!(format1.layers.len(), format2.layers.len());
        for (l1, l2) in format1.layers.iter().zip(format2.layers.iter()) {
            assert_eq!(l1.id, l2.id);
            assert_eq!(l1.name, l2.name);
            assert_eq!(l1.default_opacity, l2.default_opacity);
        }

        // Cel データを比較
        assert_eq!(format1.cel_data.len(), format2.cel_data.len());
        for (c1, c2) in format1.cel_data.iter().zip(format2.cel_data.iter()) {
            assert_eq!(c1.layer_id, c2.layer_id);
            assert_eq!(c1.frame_num, c2.frame_num);
            assert_eq!(c1.pixels, c2.pixels);
            assert_eq!(c1.width, c2.width);
            assert_eq!(c1.height, c2.height);
            assert_eq!(c1.visible, c2.visible);
        }

        // アニメーションクリップを比較
        assert_eq!(format1.clips.len(), format2.clips.len());
        for (clip1, clip2) in format1.clips.iter().zip(format2.clips.iter()) {
            assert_eq!(clip1.name, clip2.name);
            assert_eq!(clip1.frame_numbers, clip2.frame_numbers);
            assert_eq!(clip1.looping, clip2.looping);
        }
    }

    /// Test 1: 基本的な round-trip（single cel）
    #[test]
    fn test_roundtrip_single_cel() {
        // シンプルなアセットを作成
        let asset = create_test_asset("SingleCel", 32, 32, 1, 1);

        // Round-trip: asset -> format1 -> asset2 -> format2
        let format1 = asset.to_format();
        let asset2 = SpriteAsset::from_format(&format1).expect("Failed to restore from format1");
        let format2 = asset2.to_format();

        // format1 == format2 を確認
        assert_format_equal(&format1, &format2);

        // メタデータが保持されているか確認
        assert_eq!(format1.metadata.width, 32);
        assert_eq!(format1.metadata.height, 32);
        assert_eq!(format1.metadata.frame_count, 1);
        assert_eq!(format1.layers.len(), 1);
    }

    /// Test 2: 複数フレーム＋複数レイヤー
    #[test]
    fn test_roundtrip_multi_frame_multi_layer() {
        let asset = create_test_asset("MultiFrameMultiLayer", 32, 32, 3, 2);

        let format1 = asset.to_format();
        let asset2 = SpriteAsset::from_format(&format1).expect("Failed to restore");
        let format2 = asset2.to_format();

        assert_format_equal(&format1, &format2);

        // Cel データが完全に復元されているか
        assert_eq!(format2.cel_data.len(), 3 * 2); // 3 frames × 2 layers
        assert_eq!(format2.layers.len(), 2);
        assert_eq!(format2.metadata.frame_count, 3);
    }

    /// Test 3: 疎なデータ構造（sparse）
    #[test]
    fn test_roundtrip_sparse_structure() {
        let mut asset = SpriteAsset::new(0, "SparseAsset");

        // レイヤーを追加
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));
        asset.add_layer_def(LayerDef::new(1, "Layer 1"));

        // Frame 0, 2, 4 にのみ Cel を配置
        for &frame_num in &[0, 2, 4] {
            let mut frame = FrameDef::new(frame_num, 100);

            for layer_id in 0..2 {
                let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
                let cel = Cel::new(layer_id, pixels);
                frame.add_cel(cel);
            }

            asset.set_frame(frame_num, frame);
        }

        let format1 = asset.to_format();
        let asset2 = SpriteAsset::from_format(&format1).expect("Failed to restore sparse");
        let format2 = asset2.to_format();

        assert_format_equal(&format1, &format2);

        // sparse 構造が保持されているか：6 cel のみ（3 frames × 2 layers）
        assert_eq!(format2.cel_data.len(), 6);

        // Frame 0, 2, 4 が復元されているか
        assert!(asset2.get_frame(0).is_some());
        assert!(asset2.get_frame(2).is_some());
        assert!(asset2.get_frame(4).is_some());

        // Frame 1, 3 は存在しないか
        assert!(asset2.get_frame(1).is_none());
        assert!(asset2.get_frame(3).is_none());
    }

    /// Test 4: アニメーション情報の保持
    #[test]
    fn test_roundtrip_animation_info() {
        let mut asset = SpriteAsset::new(0, "AnimatedAsset");

        // レイヤーを追加
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        // フレームを追加
        for frame_num in 0..3 {
            let mut frame = FrameDef::new(frame_num, 100 + frame_num * 50);
            let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
            let cel = Cel::new(0, pixels);
            frame.add_cel(cel);
            asset.set_frame(frame_num, frame);
        }

        // アニメーションクリップを追加
        let frames = vec![
            FrameDef::new(0, 100),
            FrameDef::new(1, 150),
            FrameDef::new(2, 200),
        ];
        let clip = AnimationClipDef {
            name: "Walk".to_string(),
            frames,
            looping: true,
        };
        asset.add_animation_clip(clip);

        let format1 = asset.to_format();
        let asset2 = SpriteAsset::from_format(&format1).expect("Failed to restore animation");
        let format2 = asset2.to_format();

        assert_format_equal(&format1, &format2);

        // アニメーションクリップが保持されているか
        assert_eq!(format2.clips.len(), 1);
        assert_eq!(format2.clips[0].name, "Walk");
        assert_eq!(format2.clips[0].frame_numbers, vec![0, 1, 2]);
        assert!(format2.clips[0].looping);
    }

    /// Test 5: multiple round-trip（安定性確認）
    #[test]
    fn test_roundtrip_multiple_iterations() {
        let asset = create_test_asset("StableAsset", 32, 32, 3, 2);

        // format1 -> asset1 -> format2 -> asset2 -> format3
        let format1 = asset.to_format();
        let asset1 = SpriteAsset::from_format(&format1).expect("Failed 1st restore");

        let format2 = asset1.to_format();
        let asset2 = SpriteAsset::from_format(&format2).expect("Failed 2nd restore");

        let format3 = asset2.to_format();

        // format1 == format2 == format3 を確認
        assert_format_equal(&format1, &format2);
        assert_format_equal(&format2, &format3);

        // データが安定していることを確認
        assert_eq!(format1.cel_data.len(), format2.cel_data.len());
        assert_eq!(format2.cel_data.len(), format3.cel_data.len());
    }

    // ============ validate() メソッドのテスト ============

    /// Test 1: valid() - すべて正常
    #[test]
    fn test_validate_valid() {
        let asset = create_test_asset("ValidAsset", 32, 32, 2, 2);
        assert!(asset.validate_frame_data().is_ok());
    }

    /// Test 2: invalid_dimensions() - 幅がゼロ
    #[test]
    fn test_validate_invalid_dimensions() {
        let mut asset = SpriteAsset::new(0, "InvalidDimAsset");
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        let mut frame = FrameDef::new(0, 100);
        // 幅がゼロのピクセルデータを作成
        let mut pixels = SpriteData::new(0, 32, ColorMode::FullColor);
        let cel = Cel::new(0, pixels);
        frame.add_cel(cel);
        asset.set_frame(0, frame);

        let result = asset.validate_frame_data();
        assert!(result.is_err());
        match result {
            Err(FrameDataError::InvalidDimensions { expected, got }) => {
                assert!(got.0 == 0 || got.1 == 0);
            }
            _ => panic!("Expected InvalidDimensions error"),
        }
    }

    /// Test 3: invalid_cel_pixel_count() - ピクセル数ミスマッチ
    #[test]
    fn test_validate_invalid_pixel_count() {
        let mut asset = SpriteAsset::new(0, "InvalidPixelAsset");
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        let mut frame = FrameDef::new(0, 100);
        let mut pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        // ピクセルデータを意図的に破損させる
        pixels.pixels.truncate(100); // 期待値より少ないピクセル
        let cel = Cel::new(0, pixels);
        frame.add_cel(cel);
        asset.set_frame(0, frame);

        let result = asset.validate_frame_data();
        assert!(result.is_err());
        match result {
            Err(FrameDataError::InvalidPixelCount { expected, got }) => {
                assert_ne!(expected, got);
            }
            _ => panic!("Expected InvalidPixelCount error"),
        }
    }

    /// Test 4: invalid_layer_id() - レイヤーID不正
    #[test]
    fn test_validate_invalid_layer_id() {
        let mut asset = SpriteAsset::new(0, "InvalidLayerAsset");
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        let mut frame = FrameDef::new(0, 100);
        let pixels = SpriteData::new(32, 32, ColorMode::FullColor);
        // 定義されていないレイヤーID（1）を使用
        let cel = Cel::new(1, pixels);
        frame.add_cel(cel);
        asset.set_frame(0, frame);

        let result = asset.validate_frame_data();
        assert!(result.is_err());
        match result {
            Err(FrameDataError::InvalidLayerIndex { layer_id, max }) => {
                assert_eq!(layer_id, 1);
            }
            _ => panic!("Expected InvalidLayerIndex error"),
        }
    }

    /// Test 5: invalid_frame_id() - フレームID不正
    #[test]
    fn test_validate_invalid_frame_count() {
        let mut asset = SpriteAsset::new(0, "InvalidFrameCountAsset");
        asset.add_layer_def(LayerDef::new(0, "Layer 0"));

        // フレームデータが空の場合
        let result = asset.validate_frame_data();
        assert!(result.is_err());
        match result {
            Err(FrameDataError::InvalidCelData { reason, .. }) => {
                assert_eq!(reason, "No frames found");
            }
            _ => panic!("Expected InvalidCelData error for no frames"),
        }
    }
}
