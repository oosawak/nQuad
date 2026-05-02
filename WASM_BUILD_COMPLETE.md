# Nantaraquad WASM Build Guide

完全な WASM ビルドから Web 配布まで、すべての手順を網羅したガイドです。

## クイックスタート（5 分）

### 前提条件チェック

```bash
# Emscripten SDK
emcc --version        # バージョン 3.0+ が必要

# Rust toolchain
rustc --version       # 1.70+
rustup target list | grep wasm32-unknown-emscripten

# Node.js (Web サービング用)
node --version        # 16+
```

すべてがインストール済みなら、以下を実行：

```bash
cd /path/to/Nantaraquad

# 自動ビルド
./scripts/build-wasm.sh --release

# または CI パイプラインで検証
./scripts/ci.sh --release
```

完了時の出力：
```
[✓] Build Complete
WASM size: 4.2M
Ready to deploy! 🚀
```

---

## 詳細ガイド

### ステップ 1: Emscripten SDK をセットアップ

**自動セットアップ（推奨）**:
```bash
./scripts/setup-emscripten.sh
```

**手動セットアップ**: [EMSCRIPTEN_SETUP.md](./EMSCRIPTEN_SETUP.md) を参照

### ステップ 2: Pyxel Fork をセットアップ

```bash
# リモート上流を設定
cd pyxel_fork
git remote add upstream https://github.com/kitao/pyxel.git

# 最新バージョンを確認
git fetch upstream main
git describe --tags upstream/main
```

### ステップ 3: ビルド実行

#### Option A: スクリプト（推奨）

```bash
# デバッグビルド
./scripts/build-wasm.sh

# リリースビルド（最適化）
./scripts/build-wasm.sh --release

# クリーンビルド
./scripts/build-wasm.sh --clean --release
```

#### Option B: 手動ビルド

```bash
# 環境変数を確認
source ~/emsdk/emsdk_env.sh

# ビルド実行
cargo build --target wasm32-unknown-emscripten --release

# または verbose で進捗確認
RUST_BACKTRACE=1 cargo build --target wasm32-unknown-emscripten -vv
```

### ステップ 4: 出力確認

```bash
# ビルド成果物を確認
ls -lh target/wasm32-unknown-emscripten/release/
```

期待される出力：
```
-rwxr-xr-x  nquad.d         (dependency file)
-rwxr-xr-x  nquad.wasm      (~4MB)
```

### ステップ 5: 検証テスト

```bash
# 全チェック実行
./scripts/ci.sh --release

# 個別テスト
cargo test --lib --target wasm32-unknown-emscripten
```

---

## Pyxel バージョンアップの手順

Pyxel が新バージョンをリリースした際の統合手順：

### 1. バージョンチェック

```bash
cd pyxel_fork
git fetch upstream main
CURRENT=$(git describe --tags --abbrev=0)
LATEST=$(git describe --tags upstream/main --abbrev=0)
echo "Current: $CURRENT, Latest: $LATEST"
```

### 2. 自動更新（推奨）

```bash
./scripts/update-pyxel-fork.sh
```

このスクリプトが自動的に：
- 上流のコミットをフェッチ
- 競合を検査
- Emscripten ビルドを検証
- テストを実行
- 成功時にマージ

### 3. 手動マージ（競合がある場合）

```bash
cd pyxel_fork

# 上流から最新取得
git fetch upstream main

# マージ開始（競合になる）
git merge upstream/main

# 競合を解決
# 優先度:
#   1. crates/pyxel-core/ → ローカル版を優先
#   2. python/ → 上流版を優先
#   3. docs/ → 上流版を優先

# 解決後、コミット
git add .
git commit -m "Merge Pyxel upstream version v..."

# Emscripten ビルド検証
cd ..
./scripts/build-wasm.sh --release
```

---

## トラブルシューティング

### ビルドエラー

#### `emcc: command not found`
```bash
# Emscripten 環境を設定
source ~/emsdk/emsdk_env.sh

# 確認
emcc --version
```

#### `cannot find native library`
```bash
# Emscripten port を有効化
emcc --show-ports
emcc --show-ports=sdl2
```

#### `Failed to execute emcc`
```bash
# Emscripten SDK を再インストール
cd ~/emsdk
./emsdk uninstall latest
./emsdk install latest
./emsdk activate latest
```

### ビルドが遅い

初回ビルドは 5-10 分かかります。以後は高速化します。

```bash
# キャッシュをクリアした強制再ビルド
cargo clean --target wasm32-unknown-emscripten
./scripts/build-wasm.sh --release
```

### WASM ファイルサイズが大きい

リリースビルド時の最適化オプション：

```bash
# profiles/release を確認
cat Cargo.toml | grep -A 5 "\[profile.release\]"

# さらに最適化する場合
export RUSTFLAGS="-C target-feature=+crt-static -C opt-level=z"
cargo build --target wasm32-unknown-emscripten --release
```

---

## CI/CD 統合

### GitHub Actions パイプライン

`.github/workflows/wasm-build.yml`:
```yaml
name: WASM Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-emscripten
      - run: |
          git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
          cd ~/emsdk
          ./emsdk install latest
          ./emsdk activate latest
          source ./emsdk_env.sh
      - run: ./scripts/ci.sh --release
```

### ローカル Pre-commit フック

`.git/hooks/pre-commit`:
```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."
./scripts/ci.sh --skip-tests

exit $?
```

---

## 配布用パッケージング

### Web サーバー用

```bash
# WASM バイナリを public ディレクトリにコピー
mkdir -p web/public/wasm
cp target/wasm32-unknown-emscripten/release/nquad.wasm web/public/wasm/

# Web サーバーを起動
cd web
npm install
npm run dev

# ブラウザで確認
# http://localhost:5173
```

### Docker 化

`Dockerfile`:
```dockerfile
FROM rust:1.75-slim as builder

RUN apt-get update && apt-get install -y \
    git python3 cmake

# Emscripten セットアップ
RUN git clone https://github.com/emscripten-core/emsdk.git /opt/emsdk && \
    cd /opt/emsdk && \
    ./emsdk install latest && \
    ./emsdk activate latest

ENV EMSDK=/opt/emsdk
ENV PATH=/opt/emsdk:/opt/emsdk/upstream/emscripten:$PATH

WORKDIR /app
COPY . .

RUN rustup target add wasm32-unknown-emscripten && \
    ./scripts/build-wasm.sh --release

FROM node:18-slim
COPY --from=builder /app/target/wasm32-unknown-emscripten/release/nquad.wasm /app/wasm/
COPY --from=builder /app/web /app/web
WORKDIR /app/web
RUN npm install
CMD ["npm", "run", "dev"]
```

ビルド・実行：
```bash
docker build -t nantaraquad-wasm .
docker run -p 5173:5173 nantaraquad-wasm
```

---

## パフォーマンス最適化

### WASM バイナリサイズ

```bash
# wasm-opt で圧縮（オプション）
# 注: 別途インストールが必要
wasm-opt -Oz -o nquad.wasm target/wasm32-unknown-emscripten/release/nquad.wasm

# サイズ比較
ls -lh target/wasm32-unknown-emscripten/release/nquad.wasm
```

### 実行時パフォーマンス

```javascript
// web/src/index.js
const wasmModule = await import('../wasm/nquad.js');

// ベンチマーク
console.time('frameRender');
wasmModule.render_frame();
console.timeEnd('frameRender');
```

---

## よくある質問

**Q: Emscripten が必要ですか？**
A: はい。Pyxel-core は Emscripten の SDL2 ポートを使用するため、必須です。

**Q: Node.js は必須ですか？**
A: Web サーバーで実行する場合は推奨です。Python の SimpleHTTPServer でも可能です。

**Q: ビルドを高速化できますか？**
A: 初回ビルドは時間がかかりますが、`cargo build` を複数回実行すると高速化します。

**Q: Windows でビルドできますか？**
A: はい。PowerShell で `scripts/setup-emscripten.sh` を実行してください。

---

## さらに詳しく

- [EMSCRIPTEN_SETUP.md](./EMSCRIPTEN_SETUP.md) - Emscripten セットアップの詳細
- [PYXEL_FORK_MANAGEMENT.md](./PYXEL_FORK_MANAGEMENT.md) - Pyxel フォーク管理戦略
- [Pyxel 公式ドキュメント](https://github.com/kitao/pyxel)
- [Emscripten 公式ガイド](https://emscripten.org/docs/)
