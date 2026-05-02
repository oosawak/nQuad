#!/bin/bash
# Nantaraquad API Lookup Tool
# プログラムから API 情報をテキストベースで取得できるスクリプト

DOCS_DIR="$(dirname "$0")/../docs/api"
API_REF="$DOCS_DIR/REFERENCE.md"

if [ ! -f "$API_REF" ]; then
    echo "Error: API documentation not found at $API_REF"
    exit 1
fi

if [ $# -eq 0 ]; then
    # API 一覧表示
    echo "Nantaraquad API Reference"
    echo "========================="
    echo ""
    grep "^## " "$API_REF" | sed 's/^## //' | head -20
    echo ""
    echo "Usage: api_lookup <function_name> [format]"
    echo "Example: api_lookup pset"
    echo "Example: api_lookup pset json"
    exit 0
fi

FUNCTION=$1
FORMAT=${2:-text}

if [ "$FORMAT" = "json" ]; then
    # JSON 風フォーマット で出力
    awk "/^### \`$FUNCTION/,/^###[^`]/ {
        if (/^### / && !/\`$FUNCTION/) exit
        print
    }" "$API_REF" | sed '$d'  # 最後の行削除
else
    # テキストフォーマット
    awk "/^### \`$FUNCTION/,/^###[^`]/ {
        if (/^### / && !/\`$FUNCTION/) exit
        print
    }" "$API_REF" | sed '$d'
fi
