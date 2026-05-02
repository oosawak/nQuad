# WASM Games Build Guide

ゲームをWebAssemblyにコンパイルする方法

## 必要な環境

```bash
# Rust のインストール（まだの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack のインストール
cargo install wasm-pack
```

## ビルド方法

### 簡単な方法（推奨）

```bash
cd games/lineboy
./build.sh
```

出力：`wasm-builds/lineboy/` に以下が生成されます
- `lineboy.js` - JavaScript wrapper
- `lineboy_bg.wasm` - WebAssembly binary

### 手動でビルド

```bash
cd games/lineboy
wasm-pack build --target web --release --out-dir ../../wasm-builds/lineboy --no-opt
```

重要なフラグ：
- `--target web` - ブラウザ用WASM生成
- `--release` - 最適化版（デバッグ版より小さい）
- `--no-opt` - 最適化ステップをスキップ（推奨）

## トラブルシューティング

### Q: `wasm-opt` エラーが出た場合

**A:** `--no-opt` フラグを使用してください（デフォルト推奨）

```bash
wasm-pack build ... --no-opt
```

### Q: `wasm-pack: command not found`

**A:** `wasm-pack` をインストール

```bash
cargo install wasm-pack
```

## 一般ユーザー向け

プリビルド済みWASMが配布されている場合、ビルドは不要です。
HTMLファイルをブラウザで開いて遊んでください。

