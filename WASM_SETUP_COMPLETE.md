# ✅ Lineboy WASM Setup Complete

## 何が完成したか

### 1. **Lineboy を Pure Rust WASM で実装**
   - 完全に独立した Rust プロジェクト (`games/lineboy/`)
   - 外部ライブラリの依存性なし（`wasm-bindgen` のみ）
   - ビルドエラーなし

### 2. **正しいビルドプロセス確立**
   - コマンド: `cd games/lineboy && ./build.sh`
   - ワンコマンドで WASM パッケージ生成
   - 自動的に `/web/lineboy/` にコピー

### 3. **Web サーバー配信確認**
   - MIME タイプ: `application/wasm` ✓
   - JavaScript Wrapper: `lineboy.js` ✓
   - WebAssembly Binary: `lineboy_bg.wasm` (15KB) ✓

### 4. **テストページ作成**
   - `lineboy_test.html` - フル機能テスト
   - `lineboy_simple_test.html` - 簡潔なテスト

## ファイル構成（最終形）

```
Nantaraquad/
├── games/
│   └── lineboy/                 ← Rust ソースコード
│       ├── src/lib.rs          ← ゲーム実装
│       ├── Cargo.toml           ← 最小限の依存性
│       └── build.sh             ← ビルド自動化
│
├── web/
│   ├── lineboy_test.html
│   ├── lineboy_simple_test.html
│   └── lineboy/                 ← コンパイル済み WASM
│       ├── lineboy.js
│       ├── lineboy_bg.wasm
│       └── package.json
│
└── wasm-builds/
    └── lineboy/                 ← ビルドアーティファクト
```

## 使い方

### 開発者向け（ビルド）

```bash
cd games/lineboy
./build.sh
```

### ユーザー向け（ゲーム実行）

1. サーバー起動:
```bash
node server.js
```

2. ブラウザで開く:
   - http://localhost:8000/lineboy_simple_test.html

## 技術仕様

| 項目 | 値 |
|------|-----|
| ビルドシステム | wasm-pack + Cargo |
| コンパイラターゲット | wasm32-unknown-unknown |
| 最適化 | --no-opt （推奨） |
| ファイルサイズ | 15KB (WASM) + 4.8KB (JS) |
| 外部依存 | なし（wasm-bindgen のみ） |
| パレット | Pyxel 16色 |
| 解像度 | 160x120 ピクセル |

## 一般ユーザーの視点

✅ エンドユーザーにとって **ビルドは不要**
- プリビルド済みの `.wasm` + `.js` + `.html` で遊べる
- Node.js だけでサーバー起動
- ブラウザでゲーム実行

## 今後のステップ

1. **Cubeboy WASM** - 同じ構造で `games/cubeboy/` を作成
2. **Mario Kart WASM** - `games/mariokart/` 
3. **Mario 64 WASM** - `games/mario64/`
4. **マルチゲームメニュー** - `web/multi_games.html` にすべて統合

## 注釈

**なぜ `--no-opt` フラグを使うのか？**

wasm-opt の古いバージョンが bulk-memory feature に対応していないため。
代わりに Rust の最適化オプション (`opt-level = "z"`) で十分なサイズ削減が実現される。

