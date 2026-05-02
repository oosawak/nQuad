# Pyxel Fork Management Strategy

## 概要
Nantaraquad は公式 Pyxel リポジトリをフォークし、`pyxel_fork/` ディレクトリで管理しています。
このドキュメントは、Pyxel バージョンアップ時の組み込み戦略を定義します。

## 現在の状態
- **フォーク元**: `https://github.com/kitao/pyxel.git`
- **現在のバージョン**: v2.9.4 (d93a6d29)
- **実装対象**: pyxel-core (Rust 実装)

## バージョンアップ戦略

### 1. リモート上流の設定（初回のみ）
```bash
cd pyxel_fork
git remote add upstream https://github.com/kitao/pyxel.git
git fetch upstream
```

### 2. バージョンアップの実行
```bash
cd pyxel_fork
git fetch upstream
git merge upstream/main
# or
git rebase upstream/main  # 無変更マージの場合
```

### 3. 競合解決
競合が発生した場合は、以下の優先順位で判断：
1. `crates/pyxel-core/` の実装 → **ローカル版を優先** (Emscripten 対応)
2. `python/` のバインディング → **上流版を優先** (Nantaraquad では不要)
3. ドキュメント → **上流版を優先** (ドキュメント更新)

### 4. マージ後の検証
```bash
# Emscripten ターゲットでビルド確認
cargo build --target wasm32-unknown-emscripten

# テスト実行
cargo test --target wasm32-unknown-emscripten

# git タグを更新
git tag -a "v$(cat ../Cargo.toml | grep version | head -1 | cut -d'"' -f2)-emscripten" -m "Pyxel $(git describe --tags) with Emscripten support"
```

## ファイル構成
```
pyxel_fork/
├── crates/
│   └── pyxel-core/       ← Nantaraquad が使用するクレート
│       └── src/
│           └── platform/ ← Emscripten サポートはここ
├── python/               ← 無視 (Python バインディング)
├── wasm/                 ← 参考 (Emscripten ビルド設定)
└── .git/                 ← 上流との同期を管理
```

## 定期メンテナンス
- **月 1 回**: `git fetch upstream` で上流の最新確認
- **セキュリティ修正時**: すぐにマージ・ビルド検証
- **メジャーバージョン更新時**: 競合検査 → マージ → 全テスト実行

## シェルスクリプト統合
自動化スクリプトは `scripts/` に配置：
- `update-pyxel-fork.sh` - Pyxel バージョンアップ＆検証
- `build-wasm.sh` - Emscripten ビルド
- `ci.sh` - CI パイプライン (GitHub Actions)
