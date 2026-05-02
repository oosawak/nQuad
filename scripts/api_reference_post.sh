#!/bin/bash
# nQuad API リファレンス概要を Discord に投稿

WEBHOOK_URL="$1"

if [ -z "$WEBHOOK_URL" ]; then
    echo "Error: WEBHOOK_URL が指定されていません"
    echo "使用法: $0 <WEBHOOK_URL>"
    exit 1
fi

# ==================== Discord メッセージ作成 ====================

PAYLOAD=$(cat <<'EOF'
{
  "embeds": [
    {
      "title": "📖 nQuad API Reference (Phase 6.5)",
      "description": "Complete Game Engine API Documentation",
      "color": 3447003,
      "fields": [
        {
          "name": "Core Types",
          "value": "```rust\nnQSpriteId = usize\nnQDocument = SpriteDocument\nnQColor = RGBA (0.0-1.0)\nnQBlendMode = Normal|Add|Multiply|Screen\n```",
          "inline": false
        },
        {
          "name": "Game Engine API (Phase 7)",
          "value": "**Rendering:**\n• `draw_sprite(doc, x, y)`\n• `draw_sprite_tinted(doc, x, y, color)`\n• `draw_sprite_transformed(doc, x, y, scale_x, scale_y, rotation)`\n\n**Animation:**\n• `play_animation(doc, name)`\n• `pause_animation(doc)`\n• `set_animation_speed(doc, speed)`\n• `current_animation_frame(doc) -> u32`",
          "inline": false
        },
        {
          "name": "File I/O",
          "value": "```rust\nfm.save_document(path, doc) -> Result<PathBuf>\nfm.load_document(path) -> Result<nQDocument>\n```\n\n**File Structure:**\n```\nsprite.nquad/\n  ├── metadata.json\n  ├── history.json\n  ├── cel_0_0.bin\n  └── cel_0_1.bin\n```",
          "inline": false
        },
        {
          "name": "Editing API",
          "value": "**Layers:**\n• `doc.layers.add_layer()`\n• `doc.layers.set_blend_mode()`\n• `doc.layers.set_opacity()`\n• `doc.layers.set_visible()`\n\n**Undo/Redo:**\n• `doc.record_edit(cmd)`\n• `doc.undo()` / `doc.redo()`\n• `doc.can_undo()` / `doc.can_redo()`",
          "inline": false
        },
        {
          "name": "Common Patterns",
          "value": "**Pattern 1: Simple Game**\n```rust\nlet mut doc = fm.load_document(\"player.nquad\")?;\ndoc.play_animation(\"idle\");\n```\n\n**Pattern 2: Animation State**\n```rust\nmatch state {\n  Idle => doc.play_animation(\"idle\"),\n  Running => doc.play_animation(\"run\"),\n}\n```\n\n**Pattern 3: Multi-Document**\n```rust\nlet mut docs: HashMap<String, nQDocument> = HashMap::new();\n```",
          "inline": false
        },
        {
          "name": "macroquad Integration",
          "value": "**Available via macroquad:**\n• Drawing: `draw_circle()`, `draw_rectangle()`, `draw_line()`, `draw_text()`\n• Input: `is_key_down()`, `mouse_position()`, `is_mouse_button_pressed()`\n• Window: `screen_width()`, `screen_height()`, `clear_background()`\n• Game Loop: `#[macroquad::main(...)]` with `async fn`",
          "inline": false
        },
        {
          "name": "Color Helpers",
          "value": "```rust\nnq_color(r, g, b)           // RGB 0-255 → nQColor\nnq_color_rgba(r, g, b, a)   // RGBA 0-255 → nQColor\nnq::RED, nq::GREEN, nq::BLUE, nq::BLACK, nq::WHITE\n```",
          "inline": false
        },
        {
          "name": "Phase 7 Status",
          "value": "✅ **Foundation Complete**\n• Data model ready\n• Multi-layer support\n• Animation system\n• File persistence\n• Undo/Redo\n\n🟡 **In Development**\n• Rendering functions\n• Game loop integration\n• Performance optimization",
          "inline": false
        }
      ],
      "footer": {
        "text": "See docs/API_REFERENCE.md for full documentation"
      },
      "timestamp": "2026-05-01T00:54:30Z"
    }
  ]
}
EOF
)

echo "$PAYLOAD" | curl -X POST -H 'Content-type: application/json' \
  --data @- \
  "$WEBHOOK_URL"

echo -e "\n✅ API reference posted to Discord"
