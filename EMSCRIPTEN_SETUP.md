# Emscripten WASM Setup Guide

## 要件

Nantaraquad を Emscripten WASM にビルドするには、以下が必要です：

- **Rust**: v1.70+ (wasm32-unknown-emscripten target)
- **Emscripten SDK**: 最新版（3.1.x 推奨）
- **Node.js**: v16+
- **Python**: v3.8+

## インストール手順

### 1. Emscripten SDK をインストール

#### macOS / Linux

```bash
# Emscripten の公式リポジトリをクローン
git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
cd ~/emsdk

# 最新バージョンをインストール
./emsdk install latest
./emsdk activate latest

# 環境変数を設定（シェルプロフィールに追加）
echo 'source ~/emsdk/emsdk_env.sh' >> ~/.bashrc
# または ~/.zshrc (Zsh を使用している場合)

# 設定を反映
source ~/emsdk/emsdk_env.sh
```

#### Windows

```bash
# PowerShell で実行
git clone https://github.com/emscripten-core/emsdk.git C:\emsdk
cd C:\emsdk

# バッチファイルで初期化
emsdk.bat install latest
emsdk.bat activate latest

# 環境変数を設定（コントロールパネルで "EMSDK" → "C:\emsdk" に設定）
# または PowerShell で：
[Environment]::SetEnvironmentVariable("EMSDK", "C:\emsdk", [System.EnvironmentVariableTarget]::User)
```

### 2. Rust wasm32-unknown-emscripten ターゲットをインストール

```bash
rustup target add wasm32-unknown-emscripten
```

### 3. インストール確認

```bash
# Emscripten コンパイラバージョン確認
emcc --version

# 出力例：
# emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.27
```

## ビルド方法

### 1. 自動セットアップスクリプト（推奨）

```bash
cd /path/to/Nantaraquad
./scripts/build-wasm.sh --release
```

### 2. 手動ビルド

```bash
cd /path/to/Nantaraquad

# 環境変数を設定（既に .bashrc/.zshrc に追加した場合はスキップ可）
source ~/emsdk/emsdk_env.sh

# ビルド実行
cargo build --target wasm32-unknown-emscripten --release
```

### 3. ビルド出力

成功時の出力例：
```
Finished release [optimized] target(s) in 2m 45s
WASM size: 4.2M

✓ WASM Binary
  Path: target/wasm32-unknown-emscripten/release/nquad.wasm
  Size: 4.2M
```

## トラブルシューティング

### エラー: `emcc: command not found`

**原因**: Emscripten SDK が PATH に設定されていない

**解決法**:
```bash
# emsdk_env.sh を source する
source ~/emsdk/emsdk_env.sh

# 確認
which emcc
```

### エラー: `Failed to execute emcc`

**原因**: Emscripten SDK が見つからないか、インストールが破損している

**解決法**:
```bash
# SDK を再インストール
cd ~/emsdk
./emsdk uninstall latest
./emsdk install latest
./emsdk activate latest
source emsdk_env.sh
```

### エラー: `cannot find wasm32-unknown-emscripten`

**原因**: Rust ターゲットが見つからない

**解決法**:
```bash
rustup target add wasm32-unknown-emscripten
```

### 遅い/ビルドが止まっている

**原因**: 大規模なプロジェクトのため、初回ビルドは 5-10 分かかる可能性あり

**解決法**:
```bash
# verbose output で進捗確認
RUST_LOG=debug cargo build --target wasm32-unknown-emscripten -vv

# または別ターミナルでビルドプロセスを監視
watch -n 1 'ps aux | grep emcc'
```

## 次のステップ

ビルド完了後：

1. **WASM バイナリをテスト**
   ```bash
   ./scripts/ci.sh --release
   ```

2. **ローカル Web サーバーで実行**
   ```bash
   cd web
   npm install
   npm run dev
   ```

3. **GitHub Actions で自動化** (オプション)
   `.github/workflows/` に CI/CD パイプラインを追加

## 詳細リンク

- **Emscripten 公式ドキュメント**: https://emscripten.org/docs/
- **Rust WASM ガイド**: https://rustwasm.org/docs/
- **Pyxel WASM サポート**: https://github.com/kitao/pyxel/tree/main/wasm

## 環境変数の永続化

### Linux/macOS

`~/.bashrc` または `~/.zshrc` に追加：
```bash
export EMSDK=$HOME/emsdk
export PATH=$EMSDK:$EMSDK/upstream/emscripten:$PATH
```

### Windows PowerShell

```powershell
# 管理者権限で実行
[Environment]::SetEnvironmentVariable("EMSDK", "C:\emsdk", [System.EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("PATH", "C:\emsdk;C:\emsdk\upstream\emscripten;$env:PATH", [System.EnvironmentVariableTarget]::Machine)
```

完了後、ターミナルを再起動してください。
