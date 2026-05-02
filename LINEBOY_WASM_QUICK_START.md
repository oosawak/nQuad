# Lineboy WASM - Quick Start Guide

Lineboy ゲームが WebAssembly として正常にビルドされました。

## ファイル構成

```
/web/
├── lineboy_test.html      ← Lineboy テストページ
└── lineboy/               ← Lineboy WASM パッケージ
    ├── lineboy.js         ← JavaScript wrapper
    ├── lineboy_bg.wasm    ← WebAssembly バイナリ (15KB)
    └── lineboy.d.ts       ← TypeScript 定義

/games/
└── lineboy/               ← ソースコード
    ├── src/lib.rs         ← Rust ソースコード
    ├── Cargo.toml
    └── build.sh           ← ビルドスクリプト
```

## 再ビルド方法

```bash
cd games/lineboy
./build.sh
```

出力は自動的に `/web/lineboy/` にコピーされます。

## サーバーの起動

```bash
node server.js
```

ブラウザで開く：
- **Lineboy Test**: http://localhost:8000/lineboy_test.html
- **ゲームメニュー**: http://localhost:8000/

## 現在の状態

✅ Lineboy WASM ビルド完了
✅ サーバー配信確認
✅ MIME タイプ設定完了（application/wasm）
⏳ ブラウザでゲーム実行確認待ち

## トラブルシューティング

### WASM ファイルが見つからない場合

```bash
# WASM ファイルを web ディレクトリにコピー
cp -r wasm-builds/lineboy web/
```

### サーバーがポート 8000 で起動しない場合

別のプロセスがポート 8000 を使用している可能性があります。
プロセスを特定して終了してください。

## 技術仕様

- **ターゲット**: WebAssembly (wasm32-unknown-unknown)
- **フレームバッファ**: 160x120 ピクセル（インデックスカラー）
- **パレット**: Pyxel 16色
- **API**: `init()`, `update(left, right, jump)`, `render()`, `reset()`
- **依存関係**: wasm-bindgen のみ（外部ライブラリなし）

## ユーザーの視点

エンドユーザーは以下のファイルだけで遊べます：
- `lineboy_test.html` ← ブラウザで開く
- `lineboy/` フォルダ ← 一緒に配信

Rust のビルドツールは不要です。

