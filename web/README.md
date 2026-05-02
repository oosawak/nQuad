# Nantaraquad Web Version

Nantaraquad の Lineboy・Cubeboy をブラウザで直接実行可能な WebAssembly バージョン。

## ビルド方法

### 前提条件
- Rust 1.56+
- wasm-pack (自動インストール可能)

### ビルド実行

```bash
bash scripts/build-wasm.sh
```

このスクリプトは以下を実行します：
1. wasm-pack がインストール済みか確認（未インストール時は自動インストール）
2. Lineboy を WASM にビルド
3. Cubeboy を WASM にビルド

ビルド出力は `pkg/lineboy/` と `pkg/cubeboy/` に生成されます。

## 実行方法

### Web サーバー起動

```bash
python3 scripts/serve-wasm.py
```

または手動で起動：

```bash
cd web
python3 -m http.server 8000
```

ブラウザで http://localhost:8000 にアクセスしてください。

### ゲーム操作

#### Lineboy
- **マウス**: ドット絵を描画
- **スペースキー**: 描画モード ON/OFF
- **↑ キー**: 赤色に変更
- **↓ キー**: 緑色に変更
- **← キー**: 青色に変更
- **→ キー**: 黄色に変更
- **A キー**: キャンバスをクリア

#### Cubeboy
- **↑↓←→ キー**: 立方体を移動
- **A キー**: 赤色に変更
- **B キー**: 緑色に変更

## 対応ブラウザ

- Chrome/Chromium 74+
- Firefox 79+
- Safari 14.1+
- Edge 79+

## トラブルシューティング

### CORS エラーが出る場合
確認してください：
1. HTTP サーバーを起動しているか
2. `http://localhost:8000` (http) でアクセスしているか (https ではない)

### WASM ファイルが見つからない場合
```bash
bash scripts/build-wasm.sh
```
で再度ビルドしてください。

### ブラウザコンソールでエラーが出る場合
F12 キーでブラウザの開発者ツールを開き、Console タブでエラーメッセージを確認してください。

## アーキテクチャ

- **macroquad**: ゲームフレームワーク（WASM ネイティブサポート）
- **wasm-bindgen**: Rust ↔ JavaScript インターフェース
- **web-sys**: Web API へのアクセス

## ビルドファイルサイズ

最適化されたリリースビルド：
- Lineboy WASM: ~2-3 MB
- Cubeboy WASM: ~2-3 MB

(gzip 圧縮時はさらに小さくなります)
