# Discord 投稿ツール使用ガイド

Nantaraquad の開発進捗をリアルタイムで Discord に投稿するツールです。

## セットアップ

### 1. Discord Webhook URL を取得

Discord サーバーで Webhook を作成：

1. サーバー設定 → チャンネル → 連携サービス
2. 新しい Webhook を作成
3. URL をコピー（例：`https://discordapp.com/api/webhooks/...`）

### 2. 環境変数を設定（オプション）

```bash
# .bashrc または .zshrc に追加
export DISCORD_WEBHOOK="https://discordapp.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"

# または毎回指定
./scripts/discord_manager.sh $DISCORD_WEBHOOK progress
```

## 使用方法

### マネージャースクリプト（推奨）

```bash
# ヘルプ表示
./scripts/discord_manager.sh help

# 進捗レポートを投稿
./scripts/discord_manager.sh <WEBHOOK_URL> progress

# API リファレンスを投稿
./scripts/discord_manager.sh <WEBHOOK_URL> api

# 最新コミット情報を投稿
./scripts/discord_manager.sh <WEBHOOK_URL> commit

# 開発状況サマリーを投稿
./scripts/discord_manager.sh <WEBHOOK_URL> status

# カスタムメッセージを投稿
./scripts/discord_manager.sh <WEBHOOK_URL> notify "タイトル" "メッセージ内容" "色コード"
```

### 直接スクリプト実行

```bash
# 基本的な通知
./scripts/discord_notify.sh <WEBHOOK_URL> "タイトル" "メッセージ" "3447003"

# 進捗レポート
./scripts/progress_report.sh <WEBHOOK_URL>

# API リファレンス
./scripts/api_reference_post.sh <WEBHOOK_URL>
```

## 環境変数での使用

```bash
# 環境変数を設定
export DISCORD_WEBHOOK="https://discordapp.com/api/webhooks/..."

# その後、WEBHOOK_URL を省略可能
./scripts/discord_manager.sh progress
./scripts/discord_manager.sh api
./scripts/discord_manager.sh status
```

## カラーコード

Discord Embed で使用できる色（10進数）：

| 色 | カラーコード | 用途 |
|-----|-----------|------|
| 🔵 青 | `3447003` | 情報（デフォルト） |
| 🟢 緑 | `65280` | 完了、成功 |
| 🔴 赤 | `16711680` | エラー、注意 |
| 🟡 黄 | `16776960` | 警告 |
| ⚫ 灰色 | `9437184` | その他 |

```bash
# 赤で投稿
./scripts/discord_manager.sh <WEBHOOK_URL> notify "エラー" "ビルド失敗" "16711680"

# 緑で投稿
./scripts/discord_manager.sh <WEBHOOK_URL> notify "成功" "Phase 6.5 完了" "65280"
```

## 実例

### 進捗レポート投稿

```bash
export DISCORD_WEBHOOK="https://discordapp.com/api/webhooks/..."
./scripts/discord_manager.sh progress
```

**Discord に表示される内容：**
- Repository 情報（ブランチ、コミット数）
- Code 統計（ファイル数、行数）
- Phase 6.5 の実績
- 次フェーズの計画

### API リファレンス投稿

```bash
./scripts/discord_manager.sh api
```

**Discord に表示される内容：**
- Core types
- Game Engine API
- File I/O
- Editing API
- macroquad 統合
- Common patterns

### カスタム通知

```bash
./scripts/discord_manager.sh notify \
  "フェーズ開始" \
  "Phase 7 (Game Engine API) を開始します" \
  "3447003"
```

### CI/CD 統合（例：GitHub Actions）

```yaml
# .github/workflows/discord-notify.yml
name: Discord Notification

on:
  push:
    branches: [master]

jobs:
  notify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Post Progress Report
        env:
          DISCORD_WEBHOOK: ${{ secrets.DISCORD_WEBHOOK }}
        run: |
          chmod +x scripts/discord_manager.sh
          ./scripts/discord_manager.sh progress
```

## トラブルシューティング

### Webhook URL が無効

```
Error: 404 Not Found
```

**解決方法：**
1. Webhook URL が正しくコピーされているか確認
2. Webhook がまだ有効か確認（削除されていないか）
3. URL に余計なスペースがないか確認

```bash
# URL の確認
echo $DISCORD_WEBHOOK
```

### curl がインストールされていない

```
Command 'curl' not found
```

**解決方法（Linux）：**
```bash
sudo apt-get install curl
```

**解決方法（macOS）：**
```bash
brew install curl
```

### Permission denied

```
Permission denied: ./scripts/discord_manager.sh
```

**解決方法：**
```bash
chmod +x ./scripts/*.sh
```

## 自動投稿スケジュール（Optional）

cron を使用した定期投稿（毎日 9:00）：

```bash
# crontab を編集
crontab -e

# 以下を追加（毎日 9:00 に progress report を投稿）
0 9 * * * cd /path/to/Nantaraquad && ./scripts/discord_manager.sh progress >> /tmp/discord_notify.log 2>&1
```

## スクリプト詳細

### discord_notify.sh
- **用途**: 基本的な Discord Embed 投稿
- **入力**: WEBHOOK_URL, タイトル, メッセージ, 色
- **出力**: Discord に Embed メッセージを投稿

### progress_report.sh
- **用途**: 開発進捗レポート生成・投稿
- **入力**: WEBHOOK_URL
- **出力**: Git 統計、コード統計、フェーズ情報を含む Embed

### api_reference_post.sh
- **用途**: API リファレンス概要を投稿
- **入力**: WEBHOOK_URL
- **出力**: Core types、Game API、ファイル I/O、Common patterns を含む Embed

### discord_manager.sh
- **用途**: マスターコントローラー
- **コマンド**: progress, api, commit, status, notify
- **入力**: WEBHOOK_URL, コマンド, オプション引数
- **出力**: 指定されたスクリプトを実行

## セキュリティ注意

⚠️ **Webhook URL を公開しないでください！**

- GitHub リポジトリに Webhook URL をコミットしない
- `.env` ファイルなどで管理
- CI/CD では secrets を使用

```bash
# ❌ 悪い例
./scripts/discord_manager.sh https://discordapp.com/api/webhooks/123/456 progress

# ✅ 良い例
export DISCORD_WEBHOOK="https://discordapp.com/api/webhooks/123/456"
./scripts/discord_manager.sh progress
```

## 関連ドキュメント

- [API Reference](../docs/API_REFERENCE.md) - 完全な API ドキュメント
- [Development Plan](../.copilot/session-state/.../plan.md) - 開発計画
- [README.md](../README.md) - プロジェクト概要
