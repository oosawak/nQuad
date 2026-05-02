/// API メタデータ構造体
///
/// プログラムから API 情報を取得するためのメタデータ構造。
/// JSON化して、AI や他のツールから検索可能にする。

use serde::{Deserialize, Serialize};

/// API カテゴリ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiCategory {
    Drawing,
    Input,
    Sprite,
    Camera,
    Audio,
    Framework,
    Animation,
    Particles,
    Resources,
}

impl std::fmt::Display for ApiCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Drawing => write!(f, "Drawing"),
            Self::Input => write!(f, "Input"),
            Self::Sprite => write!(f, "Sprite"),
            Self::Camera => write!(f, "Camera"),
            Self::Audio => write!(f, "Audio"),
            Self::Framework => write!(f, "Framework"),
            Self::Animation => write!(f, "Animation"),
            Self::Particles => write!(f, "Particles"),
            Self::Resources => write!(f, "Resources"),
        }
    }
}

/// 関数パラメータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

/// 戻り値
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnType {
    pub type_name: String,
    pub description: String,
}

/// API 関数定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiFunction {
    pub name: String,
    pub category: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub return_type: ReturnType,
    pub example: Option<String>,
    pub stability: String, // "stable", "experimental", "deprecated"
}

/// API リファレンス全体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiReference {
    pub version: String,
    pub engine_name: String,
    pub functions: Vec<ApiFunction>,
    pub categories: Vec<String>,
}

impl ApiReference {
    /// API リファレンスを JSON 文字列として出力
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// API リファレンスを コンパクト JSON として出力
    pub fn to_json_compact(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }

    /// 特定のカテゴリの関数をフィルタ
    pub fn by_category(&self, category: &str) -> Vec<&ApiFunction> {
        self.functions
            .iter()
            .filter(|f| f.category == category)
            .collect()
    }

    /// 関数を名前で検索
    pub fn find_function(&self, name: &str) -> Option<&ApiFunction> {
        self.functions.iter().find(|f| f.name == name)
    }
}

/// Nantaraquad API 仕様を構築
pub fn build_api_reference() -> ApiReference {
    let functions = vec![
        // === Drawing API ===
        ApiFunction {
            name: "pset".to_string(),
            category: "Drawing".to_string(),
            description: "指定座標にピクセルを描画".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "Y座標".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色（0-15、パレットインデックス）".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("pset(10, 20, 3);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "pget".to_string(),
            category: "Drawing".to_string(),
            description: "指定座標のピクセル色を取得".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "Y座標".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "Option<u8>".to_string(),
                description: "ピクセル色（範囲外の場合 None）".to_string(),
            },
            example: Some("if let Some(col) = pget(10, 20) { println!(\"Color: {}\", col); }".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "line".to_string(),
            category: "Drawing".to_string(),
            description: "2点間に直線を描画".to_string(),
            parameters: vec![
                Parameter {
                    name: "x1".to_string(),
                    type_name: "i32".to_string(),
                    description: "開始点 X座標".to_string(),
                },
                Parameter {
                    name: "y1".to_string(),
                    type_name: "i32".to_string(),
                    description: "開始点 Y座標".to_string(),
                },
                Parameter {
                    name: "x2".to_string(),
                    type_name: "i32".to_string(),
                    description: "終了点 X座標".to_string(),
                },
                Parameter {
                    name: "y2".to_string(),
                    type_name: "i32".to_string(),
                    description: "終了点 Y座標".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("line(10, 20, 50, 60, 5);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "rect".to_string(),
            category: "Drawing".to_string(),
            description: "矩形を描画（枠線）".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "左上 X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "左上 Y座標".to_string(),
                },
                Parameter {
                    name: "w".to_string(),
                    type_name: "i32".to_string(),
                    description: "幅".to_string(),
                },
                Parameter {
                    name: "h".to_string(),
                    type_name: "i32".to_string(),
                    description: "高さ".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("rect(10, 10, 50, 30, 7);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "rectfill".to_string(),
            category: "Drawing".to_string(),
            description: "矩形を塗りつぶし".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "左上 X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "左上 Y座標".to_string(),
                },
                Parameter {
                    name: "w".to_string(),
                    type_name: "i32".to_string(),
                    description: "幅".to_string(),
                },
                Parameter {
                    name: "h".to_string(),
                    type_name: "i32".to_string(),
                    description: "高さ".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("rectfill(10, 10, 50, 30, 3);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "circle".to_string(),
            category: "Drawing".to_string(),
            description: "円を描画（枠線）".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "中心 X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "中心 Y座標".to_string(),
                },
                Parameter {
                    name: "r".to_string(),
                    type_name: "i32".to_string(),
                    description: "半径".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("circle(50, 50, 20, 9);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "circfill".to_string(),
            category: "Drawing".to_string(),
            description: "円を塗りつぶし".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "中心 X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "中心 Y座標".to_string(),
                },
                Parameter {
                    name: "r".to_string(),
                    type_name: "i32".to_string(),
                    description: "半径".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("circfill(50, 50, 20, 12);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "print".to_string(),
            category: "Drawing".to_string(),
            description: "テキストを画面に描画".to_string(),
            parameters: vec![
                Parameter {
                    name: "text".to_string(),
                    type_name: "&str".to_string(),
                    description: "描画するテキスト".to_string(),
                },
                Parameter {
                    name: "x".to_string(),
                    type_name: "i32".to_string(),
                    description: "X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "i32".to_string(),
                    description: "Y座標".to_string(),
                },
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "色".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("print(\"Hello, World!\", 10, 10, 7);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "cls".to_string(),
            category: "Drawing".to_string(),
            description: "画面をクリア".to_string(),
            parameters: vec![
                Parameter {
                    name: "col".to_string(),
                    type_name: "u8".to_string(),
                    description: "クリア色（パレットインデックス）".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("cls(0);".to_string()),
            stability: "stable".to_string(),
        },
        // === Sprite API ===
        ApiFunction {
            name: "spr".to_string(),
            category: "Sprite".to_string(),
            description: "スプライトを描画".to_string(),
            parameters: vec![
                Parameter {
                    name: "n".to_string(),
                    type_name: "usize".to_string(),
                    description: "スプライト ID".to_string(),
                },
                Parameter {
                    name: "x".to_string(),
                    type_name: "f32".to_string(),
                    description: "描画位置 X座標".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "f32".to_string(),
                    description: "描画位置 Y座標".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("spr(0, 100.0, 50.0);".to_string()),
            stability: "stable".to_string(),
        },
        // === Input API ===
        ApiFunction {
            name: "btn".to_string(),
            category: "Input".to_string(),
            description: "キーが現在押されているか判定".to_string(),
            parameters: vec![
                Parameter {
                    name: "key".to_string(),
                    type_name: "Key".to_string(),
                    description: "キー定数（Key::Up, Key::Down, Key::Left, Key::Right, Key::Z, Key::X）".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "bool".to_string(),
                description: "押下状態".to_string(),
            },
            example: Some("if btn(Key::Up) { println!(\"Up pressed\"); }".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "btnp".to_string(),
            category: "Input".to_string(),
            description: "キーが押下直後か判定（1フレームのみ true）".to_string(),
            parameters: vec![
                Parameter {
                    name: "key".to_string(),
                    type_name: "Key".to_string(),
                    description: "キー定数".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "bool".to_string(),
                description: "押下直後の状態".to_string(),
            },
            example: Some("if btnp(Key::Z) { println!(\"Z button pressed\"); }".to_string()),
            stability: "stable".to_string(),
        },
        // === Camera API ===
        ApiFunction {
            name: "camera".to_string(),
            category: "Camera".to_string(),
            description: "カメラ位置を設定".to_string(),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    type_name: "f32".to_string(),
                    description: "カメラ X位置".to_string(),
                },
                Parameter {
                    name: "y".to_string(),
                    type_name: "f32".to_string(),
                    description: "カメラ Y位置".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("camera(player_x - 80.0, player_y - 60.0);".to_string()),
            stability: "stable".to_string(),
        },
        ApiFunction {
            name: "zoom".to_string(),
            category: "Camera".to_string(),
            description: "ズームレベルを設定".to_string(),
            parameters: vec![
                Parameter {
                    name: "scale".to_string(),
                    type_name: "f32".to_string(),
                    description: "ズーム倍率（1.0 = 等倍）".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("zoom(2.0);".to_string()),
            stability: "stable".to_string(),
        },
        // === Audio API ===
        ApiFunction {
            name: "sfx".to_string(),
            category: "Audio".to_string(),
            description: "効果音を再生".to_string(),
            parameters: vec![
                Parameter {
                    name: "n".to_string(),
                    type_name: "usize".to_string(),
                    description: "サウンド ID".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("sfx(0);".to_string()),
            stability: "experimental".to_string(),
        },
        ApiFunction {
            name: "music".to_string(),
            category: "Audio".to_string(),
            description: "音楽を再生".to_string(),
            parameters: vec![
                Parameter {
                    name: "n".to_string(),
                    type_name: "usize".to_string(),
                    description: "ミュージック ID".to_string(),
                },
            ],
            return_type: ReturnType {
                type_name: "()".to_string(),
                description: "なし".to_string(),
            },
            example: Some("music(0);".to_string()),
            stability: "experimental".to_string(),
        },
        // === Framework API ===
        ApiFunction {
            name: "frame_time".to_string(),
            category: "Framework".to_string(),
            description: "前フレームの経過時間をミリ秒で取得".to_string(),
            parameters: vec![],
            return_type: ReturnType {
                type_name: "f32".to_string(),
                description: "経過時間（ミリ秒）".to_string(),
            },
            example: Some("let dt = frame_time();".to_string()),
            stability: "stable".to_string(),
        },
    ];

    let categories = vec![
        "Drawing".to_string(),
        "Sprite".to_string(),
        "Input".to_string(),
        "Camera".to_string(),
        "Audio".to_string(),
        "Framework".to_string(),
        "Animation".to_string(),
        "Particles".to_string(),
        "Resources".to_string(),
    ];

    ApiReference {
        version: "0.1.0".to_string(),
        engine_name: "Nantaraquad".to_string(),
        functions,
        categories,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_reference_creation() {
        let api = build_api_reference();
        assert!(!api.functions.is_empty());
        assert_eq!(api.engine_name, "Nantaraquad");
    }

    #[test]
    fn test_json_serialization() {
        let api = build_api_reference();
        let json = api.to_json().expect("JSON serialization failed");
        assert!(json.contains("pset"));
        assert!(json.contains("Drawing"));
    }

    #[test]
    fn test_find_function() {
        let api = build_api_reference();
        let pset = api.find_function("pset");
        assert!(pset.is_some());
    }

    #[test]
    fn test_filter_by_category() {
        let api = build_api_reference();
        let drawing = api.by_category("Drawing");
        assert!(!drawing.is_empty());
    }
}
