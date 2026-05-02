#!/bin/bash
# Discord Webhook 投稿ツール
# 使用法: ./discord_notify.sh "WEBHOOK_URL" "タイトル" "メッセージ" "色コード"

set -e

WEBHOOK_URL="$1"
TITLE="$2"
MESSAGE="$3"
COLOR="${4:-3447003}"  # デフォルト色: 青

if [ -z "$WEBHOOK_URL" ]; then
    echo "Error: WEBHOOK_URL が指定されていません"
    echo "使用法: $0 <WEBHOOK_URL> <TITLE> <MESSAGE> [COLOR]"
    exit 1
fi

# Discord Embed JSON を作成
PAYLOAD=$(cat <<EOF
{
  "embeds": [
    {
      "title": "$TITLE",
      "description": "$MESSAGE",
      "color": $COLOR,
      "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
    }
  ]
}
EOF
)

# Webhook に POST
curl -X POST -H 'Content-type: application/json' \
  --data "$PAYLOAD" \
  "$WEBHOOK_URL"

echo -e "\n✅ Discord に投稿しました"
