#!/bin/bash
# Nantaraquad 開発進捗レポート生成 + Discord 投稿
# 使用法: ./progress_report.sh <WEBHOOK_URL>

WEBHOOK_URL="$1"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -z "$WEBHOOK_URL" ]; then
    echo "Error: WEBHOOK_URL が指定されていません"
    echo "使用法: $0 <WEBHOOK_URL>"
    exit 1
fi

cd "$PROJECT_DIR"

# ==================== レポート作成 ====================

echo "📊 進捗レポートを生成中..."

# Git 情報取得
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
TOTAL_COMMITS=$(git rev-list --count HEAD)
LATEST_COMMIT=$(git log -1 --pretty=format:"%h - %s (%ar)")

# ファイル統計
RUST_FILES=$(find src -name "*.rs" 2>/dev/null | wc -l)
DOC_FILES=$(find docs -name "*.md" 2>/dev/null | wc -l)
EXAMPLE_FILES=$(find examples -name "*.rs" 2>/dev/null | wc -l)

# コード行数
TOTAL_LINES=$(find src -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')

# Phase 6.5 完了情報
PHASE_COMMITS=$(git log --oneline --grep="Phase 6.5" | wc -l)

# ==================== Discord メッセージ作成 ====================

REPORT=$(cat <<EOF
🚀 **Nantaraquad Phase 6.5 完了レポート**

**Repository Status**
• Branch: \`$CURRENT_BRANCH\`
• Total Commits: $TOTAL_COMMITS
• Latest: $LATEST_COMMIT

**Code Statistics**
• Rust Files: $RUST_FILES
• Documentation: $DOC_FILES  
• Examples: $EXAMPLE_FILES
• Total Lines: $TOTAL_LINES

**Phase 6.5 Achievements**
• Commits: $PHASE_COMMITS
• Features: nQuad API, SpriteDocument, EditCommand, File I/O
• Architecture: Complete multi-layer, animation-ready engine
• Testing: Unit tests + validation ready

**Current Status**
✅ Phase 6.5: Architecture Redesign - COMPLETE
🟡 Phase 7: Game Engine API - READY TO START

**Next Phase**
Phase 7 (Game Engine API) will implement:
• Sprite rendering (draw_sprite, draw_sprite_ex)
• Animation playback
• Game loop integration
• Input handling wrapper
• Performance statistics

**Documentation**
📖 API Reference: docs/API_REFERENCE.md
📋 Architecture Plan: .copilot/session-state/.../plan.md
EOF
)

# ==================== 色コード ====================
# 0x00ff00 = 緑 (完了)
COLOR_CODE="65280"  # 16進数 00FF00 → 10進数 65280

# ==================== Discord 投稿 ====================

PAYLOAD=$(cat <<'PAYLOAD_END'
{
  "embeds": [
    {
      "title": "🚀 Nantaraquad Phase 6.5 完了レポート",
      "description": "Architecture Redesign Complete - Ready for Phase 7",
      "color": 65280,
      "fields": [
        {
          "name": "Repository Status",
          "value": "• Branch: `master`\n• Total Commits: '"$TOTAL_COMMITS"'\n• Latest: '"$LATEST_COMMIT"'",
          "inline": false
        },
        {
          "name": "Code Statistics",
          "value": "• Rust Files: '"$RUST_FILES"'\n• Documentation: '"$DOC_FILES"'\n• Examples: '"$EXAMPLE_FILES"'\n• Total Lines: '"$TOTAL_LINES"'",
          "inline": false
        },
        {
          "name": "Phase 6.5 Achievements",
          "value": "✅ nQuad API Design\n✅ SpriteDocument Model\n✅ EditCommand System (Undo/Redo)\n✅ File I/O (save/load)\n✅ Complete Architecture",
          "inline": false
        },
        {
          "name": "Current Status",
          "value": "✅ Phase 6.5: Architecture Redesign - **COMPLETE**\n🟡 Phase 7: Game Engine API - **READY**",
          "inline": false
        },
        {
          "name": "Next Phase",
          "value": "**Phase 7: Game Engine API**\n• Sprite rendering\n• Animation playback\n• Game loop integration\n• Input handling\n• Performance optimization",
          "inline": false
        },
        {
          "name": "Documentation",
          "value": "📖 [API Reference](https://github.com/...)\n📋 [Architecture Plan](...)\n🏗️ [Development Notes](...)",
          "inline": false
        }
      ],
      "footer": {
        "text": "Nantaraquad Development"
      },
      "timestamp": "'"$(date -u +'%Y-%m-%dT%H:%M:%SZ')"'"
    }
  ]
}
PAYLOAD_END
)

echo "$PAYLOAD" | curl -X POST -H 'Content-type: application/json' \
  --data @- \
  "$WEBHOOK_URL"

echo -e "\n✅ Progress report posted to Discord"
