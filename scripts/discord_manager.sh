#!/bin/bash
# Nantaraquad Discord 投稿マネージャー
# 作業報告、進捗レポート、API ドキュメントなどを Discord に投稿

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# ==================== ヘルプ表示 ====================

show_help() {
    cat <<EOF
🤖 Nantaraquad Discord 投稿ツール

使用法:
  $0 <WEBHOOK_URL> <command> [options]

コマンド:
  progress     - 開発進捗レポートを投稿
  api          - API リファレンス概要を投稿
  commit       - 最新のコミット情報を投稿
  status       - 開発状況サマリーを投稿
  notify       - カスタムメッセージを投稿
               (例: notify "タイトル" "メッセージ" "色")

環境変数:
  DISCORD_WEBHOOK  - Webhook URL を環境変数で指定可能

例:
  export DISCORD_WEBHOOK="https://discordapp.com/api/webhooks/..."
  $0 progress
  $0 api
  $0 notify "フェーズ開始" "Phase 7 を開始します" "3447003"

カラーコード:
  3447003  - 青 (デフォルト)
  65280    - 緑
  16711680 - 赤
  16776960 - 黄
  9437184  - 灰色

EOF
}

if [ $# -lt 2 ]; then
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi
fi

# ==================== Webhook URL 処理 ====================

WEBHOOK_URL="${1:-$DISCORD_WEBHOOK}"
COMMAND="${2:-}"

if [ -z "$WEBHOOK_URL" ]; then
    echo "❌ Error: WEBHOOK_URL が指定されていません"
    echo "使用法: $0 <WEBHOOK_URL> <command>"
    echo ""
    echo "または環境変数を設定してください:"
    echo "  export DISCORD_WEBHOOK='https://discordapp.com/api/webhooks/...'"
    exit 1
fi

if [ -z "$COMMAND" ]; then
    show_help
    exit 0
fi

# ==================== コマンド処理 ====================

case "$COMMAND" in
    progress)
        echo "📊 進捗レポートを投稿中..."
        "$SCRIPT_DIR/progress_report.sh" "$WEBHOOK_URL"
        ;;
    api)
        echo "📖 API リファレンスを投稿中..."
        "$SCRIPT_DIR/api_reference_post.sh" "$WEBHOOK_URL"
        ;;
    commit)
        echo "📝 最新コミットを投稿中..."
        cd "$PROJECT_DIR"
        LATEST=$(git log -1 --pretty=format:"%h - %s (%ar)")
        "$SCRIPT_DIR/discord_notify.sh" "$WEBHOOK_URL" "最新コミット" "$LATEST" "3447003"
        ;;
    status)
        echo "📈 開発状況を投稿中..."
        cd "$PROJECT_DIR"
        
        # 統計情報取得
        TOTAL_COMMITS=$(git rev-list --count HEAD)
        TOTAL_LINES=$(find src -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
        PHASE_COMMITS=$(git log --oneline --grep="Phase 6.5" | wc -l)
        
        STATUS_MSG="**Repository Status**
Total Commits: $TOTAL_COMMITS
Phase 6.5 Commits: $PHASE_COMMITS
Code Lines: $TOTAL_LINES
Branch: $(git rev-parse --abbrev-ref HEAD)

**Current Phase:** Phase 6.5 Complete ✅
**Next Phase:** Phase 7 Ready 🟡"
        
        "$SCRIPT_DIR/discord_notify.sh" "$WEBHOOK_URL" "開発状況サマリー" "$STATUS_MSG" "65280"
        ;;
    notify)
        TITLE="${3:-通知}"
        MESSAGE="${4:-}"
        COLOR="${5:-3447003}"
        
        if [ -z "$MESSAGE" ]; then
            echo "❌ Error: メッセージが指定されていません"
            echo "使用法: $0 $WEBHOOK_URL notify \"タイトル\" \"メッセージ\" [色コード]"
            exit 1
        fi
        
        echo "🔔 カスタムメッセージを投稿中..."
        "$SCRIPT_DIR/discord_notify.sh" "$WEBHOOK_URL" "$TITLE" "$MESSAGE" "$COLOR"
        ;;
    help|-h|--help)
        show_help
        ;;
    *)
        echo "❌ Error: 不明なコマンド: $COMMAND"
        echo ""
        show_help
        exit 1
        ;;
esac

exit 0
