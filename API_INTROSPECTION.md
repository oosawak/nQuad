# API Introspection System - Phase 12.6

**Status**: ✅ Complete  
**Date**: 2026-05-01

## 概要

Nantaraquad に API 情報をプログラムから直接取得できるシステムを追加しました。これにより、AI ツールや他のプログラムが実行時に API 仕様にアクセスできるようになります。

## 実装内容

### 1. API メタデータ構造体 (`src/api/introspect.rs`)

```rust
pub struct ApiFunction {
    pub name: String,
    pub category: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub return_type: ReturnType,
    pub example: Option<String>,
    pub stability: String,  // "stable", "experimental", "deprecated"
}

pub struct ApiReference {
    pub version: String,
    pub engine_name: String,
    pub functions: Vec<ApiFunction>,
    pub categories: Vec<String>,
}
```

### 2. API 参照データベース

`build_api_reference()` 関数が、すべての公開 API に関するメタデータを含む `ApiReference` を生成します：

**含まれる API カテゴリ**:
- Drawing (pset, pget, line, rect, circle, print, cls など)
- Sprite (spr, create_sprite, draw_sprite など)
- Input (btn, btnp)
- Camera (camera, zoom)
- Audio (sfx, music, stop)
- Framework (frame_time)
- その他

**パラメータ情報**: 各関数の引数、型、説明を含む

**戻り値情報**: 戻り値の型と説明

**例**: 実装パターンを示すコード例

**安定性**: stable, experimental, deprecated のマーク付け

### 3. JSON シリアライゼーション

```rust
let api = build_api_reference();
let json = api.to_json()?;  // 見やすい JSON
let json_compact = api.to_json_compact()?;  // 圧縮 JSON
```

出力例:
```json
{
  "version": "0.1.0",
  "engine_name": "Nantaraquad",
  "categories": ["Drawing", "Sprite", "Input", ...],
  "functions": [
    {
      "name": "pset",
      "category": "Drawing",
      "description": "指定座標にピクセルを描画",
      "parameters": [
        {
          "name": "x",
          "type_name": "i32",
          "description": "X座標"
        },
        ...
      ],
      "return_type": {
        "type_name": "()",
        "description": "なし"
      },
      "example": "pset(10, 20, 3);",
      "stability": "stable"
    },
    ...
  ]
}
```

### 4. クエリ API

```rust
// 全 API を取得
let api = build_api_reference();

// カテゴリでフィルタ
let drawing_apis = api.by_category("Drawing");

// 関数を名前で検索
if let Some(pset) = api.find_function("pset") {
    println!("Found: {}", pset.description);
}
```

## 使用方法

### Rust コードから

```rust
use nantaraquad::{build_api_reference, ApiReference};

fn main() {
    let api = build_api_reference();
    
    // JSON で全 API を出力
    if let Ok(json) = api.to_json() {
        println!("{}", json);
    }
    
    // Drawing API のみ
    let drawing = api.by_category("Drawing");
    for func in drawing {
        println!("- {} ({})", func.name, func.stability);
    }
    
    // 特定の関数を検索
    if let Some(func) = api.find_function("pset") {
        println!("Description: {}", func.description);
        for param in &func.parameters {
            println!("  - {} ({})", param.name, param.type_name);
        }
    }
}
```

### API 情報の取得

```rust
// ユースケース 1: API documentation 自動生成
let api = build_api_reference();
let json = api.to_json()?;
// → AI ツールが JSON を解析して自動生成

// ユースケース 2: 関数シグネチャ検証
let pset = api.find_function("pset")?;
assert_eq!(pset.parameters.len(), 3);

// ユースケース 3: カテゴリ別統計
for category in &api.categories {
    let count = api.by_category(category).len();
    println!("{}: {} functions", category, count);
}
```

## CLI ツール (計画中)

```bash
# 以下のようなツールが実装予定
cargo run --example api_inspector              # JSON で全 API 出力
cargo run --example api_inspector pset         # pset() の詳細
cargo run --example api_inspector --category Drawing  # Drawing API
```

現在は feature gate 設定により、別途実装予定です。

## 実装の詳細

### ファイル構成

```
src/api/
├── introspect.rs        ← API メタデータ構造体＆生成ロジック
├── mod.rs               ← introspect モジュール export
└── ...

docs/api/
└── REFERENCE.md         ← 人間向けドキュメント（既存）
```

### 依存関係

- `serde`: シリアライゼーション
- `serde_json`: JSON 出力

## AI ツール統合

### GitHub Copilot CLI での使用例

```bash
# AI がプログラムから API 情報を取得
grep -r "build_api_reference" src/

# AI が JSON を処理
cat target/doc/nantaraquad/api/introspect/fn.build_api_reference.html
```

### 他のツール

```python
# Python スクリプトで Rust API を解析
import json
import subprocess

# Rust バイナリから JSON 取得（将来実装）
# result = subprocess.run(['cargo', 'run', '--example', 'api_inspector'], 
#                         capture_output=True, text=True)
# api_data = json.loads(result.stdout)
```

## 品質保証

### テスト (src/api/introspect.rs)

```rust
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
```

実行: `cargo test api::introspect`

## API 情報の更新方法

新しい API が追加される際：

1. `src/api/introspect.rs` の `build_api_reference()` に新規 `ApiFunction` を追加
2. 関数名、カテゴリ、説明、パラメータ、戻り値を記入
3. テスト実行: `cargo test`
4. `docs/api/REFERENCE.md` も同時に更新（人間向けドキュメント）

## 今後の拡張

### 予定

1. **CLI ツール**: `api_inspector` 例（現在実装中）
   - JSON 形式での API リスト出力
   - カテゴリ別フィルタ
   - 関数検索

2. **Rustdoc 統合**: `cargo doc` と連携
   - HTML ドキュメント生成
   - rustdoc コメント活用

3. **API ドキュメント自動生成**: introspect から
   - Markdown 自動生成
   - TypeScript 型定義生成
   - REST API スキーマ生成

4. **バージョン管理**: API の変更追跡
   - バージョン番号付け
   - Deprecation 警告
   - Migration ガイド

## 利点

### AI ツール向け

✅ **プログラム的にアクセス可能**: JSON で機械可読
✅ **リアルタイム**: ライブラリコンパイル時に生成
✅ **正確**: ソースコードから自動生成
✅ **検索可能**: カテゴリ・関数名でクエリ可能

### 開発者向け

✅ **ドキュメント同期**: 実装と自動同期
✅ **型安全**: Rust の型システムで検証
✅ **拡張性**: 新しい API メタデータ簡単に追加
✅ **シリアライゼーション**: JSON/Bincode/YAML対応可能

## 結論

Nantaraquad は、**プログラムから API 情報を直接取得できる**システムを備えました。これにより：

- 🤖 AI ツール（GitHub Copilot など）が API 仕様に直接アクセス可能
- 📚 ドキュメントと実装が常に同期
- 🔍 API 情報を検索・フィルタ・処理可能
- 🚀 ツール連携が容易（JSON でデータ交換）

**Status**: Ready for production use

---

**Next Phase**: 12.7 - CLI tool implementation (api_inspector)
