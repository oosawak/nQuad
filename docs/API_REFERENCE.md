# nQuad (Nantaraquad) Game Engine API Reference

**Version**: 1.0 (Phase 6.5 foundation)  
**Language**: Rust  
**Base Framework**: macroquad 0.4  

---

## Overview

nQuad is a sprite-based 2D game engine built on macroquad. It provides:

- **Sprite management** - Multi-layer sprite documents with animations
- **Animation system** - Frame-based animations with playback control
- **Undo/Redo** - Complete edit history for all operations
- **Pixel art tools** - Painting, color picking, bucket fill, filters
- **File persistence** - Save/load documents with metadata

### Design Philosophy

- **Game-first**: API optimized for game development, not just editing
- **Type-safe**: Strong typing with `nQ*` prefix for public API
- **Composable**: Use individual functions or high-level session management
- **Efficient**: Caching and dirty flags prevent redundant work

---

## Core Types

### `nQSpriteId`
Unique identifier for a sprite document.

```rust
pub type nQSpriteId = usize;
```

### `nQDocument`
The complete sprite document: layers, animations, edit history, cache.

```rust
pub struct SpriteDocument {
    pub id: usize,                      // Unique document ID
    pub name: String,                   // Document name
    pub layers: LayerStack,             // Multi-layer support
    pub animations: AnimationController,// Animation clips
    pub history: EditCommandHistory,    // Undo/Redo stack
    pub frame_data: HashMap<u32, ...>,  // Per-frame layer data
    pub current_frame: u32,             // Active frame for editing
    // (internal cache fields)
}
```

**Type Alias** (recommended for user code):
```rust
use nquad::nQDocument;  // Cleaner than SpriteDocument
```

### `nQPlaybackState`
Animation playback status.

```rust
pub enum PlaybackState {
    Playing,   // Animation running
    Paused,    // Paused (can resume)
    Stopped,   // Stopped at frame 0
}
```

### `nQColor`
RGBA color in linear space.

```rust
pub struct nQColor {
    pub r: f32,  // 0.0 - 1.0
    pub g: f32,
    pub b: f32,
    pub a: f32,  // 0.0 = transparent, 1.0 = opaque
}
```

**Helper Functions**:
```rust
// Create colors
nq_color(255, 0, 0);           // RGB → nQColor (opaque)
nq_color_rgba(255, 0, 0, 128); // RGBA → nQColor

// Presets
nq::RED;     // (1.0, 0.0, 0.0, 1.0)
nq::GREEN;   // (0.0, 1.0, 0.0, 1.0)
nq::BLUE;    // (0.0, 0.0, 1.0, 1.0)
nq::BLACK;   // (0.0, 0.0, 0.0, 1.0)
nq::WHITE;   // (1.0, 1.0, 1.0, 1.0)
// ... and more
```

### `nQBlendMode`
Layer compositing blend mode.

```rust
pub enum BlendMode {
    Normal,      // α = src.α + dst.α(1 - src.α)
    Add,         // Additive blend (bright)
    Multiply,    // Multiplicative blend (dark)
    Screen,      // Screen blend (light)
}
```

### `nQEditCommand`
Recorded edit operation for Undo/Redo.

```rust
pub enum EditCommand {
    PaintStroke { layer_id: u32, pixels: Vec<(u32, u32, Vec<u8>)> },
    AddLayer { layer_id: u32, name: String, sprite: SpriteData },
    DeleteLayer { layer_id: u32, ... },
    SetLayerOpacity { layer_id: u32, old_opacity: f32, new_opacity: f32 },
    SetLayerBlendMode { layer_id: u32, old_mode: BlendMode, new_mode: BlendMode },
    SetLayerVisibility { layer_id: u32, old_visible: bool, new_visible: bool },
    SetLayerLocked { layer_id: u32, old_locked: bool, new_locked: bool },
    // Frame operations (Phase 7)
    // AddFrame { ... }
    // DeleteFrame { ... }
}
```

---

## Game Engine API (Phase 7 - Coming Soon)

### Initialization

```rust
// Create a new document for gameplay
let sprite = SpriteData::new(64, 64, ColorMode::FullColor);
let mut doc = nQDocument::new(0, "game_sprite", sprite);

// Load from file
let mut fm = FileManager::new();
let doc = fm.load_document("assets/sprites/player.nquad")?;
```

### Sprite Rendering

```rust
// Draw sprite at position
fn draw_sprite(doc: &nQDocument, x: f32, y: f32);

// Draw with color tint
fn draw_sprite_tinted(doc: &nQDocument, x: f32, y: f32, color: nQColor);

// Draw with scale/rotation
fn draw_sprite_transformed(
    doc: &nQDocument,
    x: f32, y: f32,
    scale_x: f32, scale_y: f32,
    rotation: f32,  // radians
);
```

### Animation Playback

```rust
// Play animation by name
fn play_animation(doc: &mut nQDocument, clip_name: &str);

// Control playback
fn pause_animation(doc: &mut nQDocument);
fn resume_animation(doc: &mut nQDocument);
fn stop_animation(doc: &mut nQDocument);
fn set_animation_speed(doc: &mut nQDocument, speed: f32);  // 1.0 = normal

// Query state
fn animation_playing(doc: &nQDocument) -> bool;
fn current_animation_frame(doc: &nQDocument) -> u32;
fn current_animation_name(doc: &nQDocument) -> &str;
```

### Input Handling

```rust
// Keyboard
fn is_key_pressed(key: KeyCode) -> bool;
fn is_key_released(key: KeyCode) -> bool;
fn is_key_down(key: KeyCode) -> bool;

// Mouse
fn mouse_position() -> (f32, f32);
fn is_mouse_button_down(button: MouseButton) -> bool;
fn is_mouse_button_pressed(button: MouseButton) -> bool;
```

**Note**: These are macroquad's standard functions, re-exported via nQuad.

### Game Loop Integration

```rust
use macroquad::prelude::*;
use nquad::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut doc = nQDocument::new(0, "player", SpriteData::new(64, 64, ColorMode::FullColor));
    doc.play_animation("idle");  // Start animation
    
    loop {
        // Update
        if is_key_pressed(KeyCode::Right) {
            // ... move player
        }
        
        doc.update(get_frame_time() * 1000.0);  // Update animations (ms)
        
        // Draw
        clear_background(WHITE);
        draw_sprite(&doc, 100.0, 100.0);
        
        next_frame().await;
    }
}
```

### Document Update

```rust
// Update animations and cache
fn update_document(doc: &mut nQDocument, delta_ms: f32);

// Force cache refresh (for debugging)
fn invalidate_cache(doc: &mut nQDocument);
```

---

## Editing API (Layer/Animation Management)

### Layer Operations

```rust
// Add a layer
let new_layer = Layer::new(32, 32);
doc.layers.add_layer(new_layer);

// Get active layer
if let Some(layer) = doc.layers.active_layer_mut() {
    // Paint pixels
    layer.sprite.set_pixel(x, y, color)?;
}

// Set blend mode
doc.layers.set_blend_mode(layer_id, BlendMode::Multiply)?;

// Set opacity
doc.layers.set_opacity(layer_id, 0.8)?;

// Toggle visibility
doc.layers.set_visible(layer_id, false)?;
```

### Animation Management

```rust
// Create animation clip
let mut clip = AnimationClip::new("walk", first_frame);
clip.add_frame(frame_2);
clip.add_frame(frame_3);
clip.set_looping(true);
doc.animations.add_clip(clip);

// Get current clip
if let Some(clip) = doc.animations.current_clip() {
    println!("Playing: {}", clip.name);
    println!("Frame: {}/{}", 
        doc.animations.current_frame_idx(),
        clip.frame_count());
}

// Switch animation
doc.animations.select_clip("walk")?;
```

### Undo/Redo

```rust
// Record operation
let cmd = EditCommand::SetLayerOpacity {
    layer_id: 0,
    old_opacity: 1.0,
    new_opacity: 0.5,
};
doc.record_edit(cmd);

// Undo
if doc.can_undo() {
    doc.undo();
}

// Redo
if doc.can_redo() {
    doc.redo();
}
```

---

## File I/O

### Save Document

```rust
use nquad::FileManager;

let mut fm = FileManager::new();
let path = fm.save_document("assets/player.nquad", &doc)?;

// Directory structure created:
// player.nquad/
//   ├── metadata.json       (layers, animations, history)
//   ├── history.json        (edit command log)
//   ├── cel_0_0.bin        (frame 0, layer 0 pixels)
//   └── cel_0_1.bin        (frame 0, layer 1 pixels)
```

### Load Document

```rust
let mut fm = FileManager::new();
let doc = fm.load_document("assets/player.nquad")?;

// All state restored:
// - Layer structure and metadata
// - Animation clips
// - Edit history (for undo/redo)
// - Pixel data
```

### Recent Files

```rust
for path in &fm.recent_files {
    println!("Recent: {:?}", path);
}

let current = fm.current_filename();
```

---

## macroquad Integration

### Drawing Functions (from macroquad)

```rust
use macroquad::prelude::*;

// Clear screen
clear_background(BLACK);

// Draw primitives
draw_circle(x, y, radius, color);
draw_rectangle(x, y, w, h, color);
draw_line(x1, y1, x2, y2, thickness, color);
draw_text("Hello", x, y, font_size, color);

// Draw textures
draw_texture(texture, x, y, color);
draw_texture_ex(texture, x, y, color, params);
```

### Input (from macroquad)

```rust
use macroquad::prelude::*;

// Keyboard
is_key_down(KeyCode::W);
is_key_pressed(KeyCode::Space);

// Mouse
mouse_position();
is_mouse_button_down(MouseButton::Left);

// Window
screen_width();
screen_height();
```

### Game Loop Pattern (from macroquad)

```rust
#[macroquad::main("Window Title")]
async fn main() {
    loop {
        // Your game logic here
        
        next_frame().await;  // Required for async
    }
}
```

---

## Color Conversion Table

| Function | Input | Output | Example |
|----------|-------|--------|---------|
| `nq_color()` | R, G, B (0-255) | nQColor | `nq_color(255, 0, 0)` → RED |
| `nq_color_rgba()` | R, G, B, A (0-255) | nQColor | `nq_color_rgba(255, 0, 0, 128)` → Red 50% |
| `to_macroquad_color()` | nQColor | macroquad::Color | For drawing |

---

## Performance Tips

1. **Use CompositeCache**: Automatic (dirty flag prevents recompositing)
2. **Batch operations**: Combine multiple edits before Undo/Redo
3. **Limit animation updates**: Only call `update_document()` when playing
4. **Reuse documents**: Load once, reuse for multiple game objects
5. **Monitor cache stats**: Use `get_cache_stats()` to optimize hit rate

---

## Common Patterns

### Pattern 1: Simple Sprite Game

```rust
use macroquad::prelude::*;
use nquad::*;

#[macroquad::main("Sprite Game")]
async fn main() {
    let mut fm = FileManager::new();
    let mut player = fm.load_document("player.nquad")?;
    
    let mut x = 100.0;
    let mut y = 100.0;
    
    loop {
        // Input
        if is_key_down(KeyCode::Right) { x += 2.0; }
        if is_key_down(KeyCode::Left) { x -= 2.0; }
        
        // Update
        player.update(get_frame_time() * 1000.0);
        
        // Draw
        clear_background(WHITE);
        draw_sprite(&player, x, y);
        
        next_frame().await;
    }
}
```

### Pattern 2: Animation State Machine

```rust
enum PlayerState {
    Idle,
    Running,
    Jumping,
}

let mut state = PlayerState::Idle;

loop {
    // Update state based on input
    let new_state = match (state, is_key_down(KeyCode::Right)) {
        (PlayerState::Idle, true) => PlayerState::Running,
        (PlayerState::Running, false) => PlayerState::Idle,
        (_, true) if is_key_pressed(KeyCode::Space) => PlayerState::Jumping,
        _ => state,
    };
    
    if new_state != state {
        state = new_state;
        match state {
            PlayerState::Idle => doc.play_animation("idle"),
            PlayerState::Running => doc.play_animation("run"),
            PlayerState::Jumping => doc.play_animation("jump"),
        }
    }
    
    doc.update(get_frame_time() * 1000.0);
    draw_sprite(&doc, x, y);
}
```

### Pattern 3: Multi-Document Management

```rust
use std::collections::HashMap;

struct Game {
    documents: HashMap<String, nQDocument>,
    active_doc: String,
}

impl Game {
    fn load_sprite(&mut self, name: &str, path: &str) -> Result<(), String> {
        let mut fm = FileManager::new();
        let doc = fm.load_document(path)?;
        self.documents.insert(name.to_string(), doc);
        Ok(())
    }
    
    fn update(&mut self, delta_ms: f32) {
        if let Some(doc) = self.documents.get_mut(&self.active_doc) {
            doc.update(delta_ms);
        }
    }
    
    fn draw(&self, x: f32, y: f32) {
        if let Some(doc) = self.documents.get(&self.active_doc) {
            draw_sprite(doc, x, y);
        }
    }
}
```

---

## API Completeness

### ✅ Implemented (Phase 6.5)

- Document model (SpriteDocument)
- Layer management (LayerStack)
- Animation system (AnimationController)
- Edit commands and Undo/Redo
- File I/O (save/load)
- Color utilities
- Cache management

### 🟡 Planned (Phase 7)

- Game rendering functions (draw_sprite, draw_sprite_ex, etc.)
- Animation playback during gameplay
- Document update loop integration
- Input event handling wrapper
- Performance statistics

### 🟢 Available (via macroquad)

- Drawing primitives (circle, rectangle, line)
- Text rendering
- Texture management
- Input handling (keyboard, mouse)
- Window management
- Game loop with async/await

---

## Migration Guide (Phase 7)

If you're upgrading from Phase 6:

**Before (Phase 6)**:
```rust
let sprite = SpriteData::new(64, 64, ColorMode::FullColor);
// Direct sprite manipulation
sprite.set_pixel(x, y, color)?;
```

**After (Phase 7)**:
```rust
let mut doc = nQDocument::new(0, "sprite", SpriteData::new(64, 64, ColorMode::FullColor));
// Work with documents and layers
if let Some(layer) = doc.layers.active_layer_mut() {
    layer.sprite.set_pixel(x, y, color)?;
}
// Enjoy Undo/Redo and animations!
doc.undo();
doc.play_animation("idle");
```

---

## FAQ

**Q: Can I use nQuad without macroquad?**  
A: Not directly. nQuad is built on macroquad's window/rendering layer. You can use `SpriteData` alone for headless sprite manipulation.

**Q: How do I do 3D rendering with nQuad?**  
A: nQuad is 2D sprite-focused. Use macroquad's 3D functions (`draw_cube`, `draw_mesh`, etc.) alongside nQuad for hybrid rendering.

**Q: Can I batch multiple sprites efficiently?**  
A: Yes! Render multiple documents in a loop. CompositeCache will optimize repeated frames automatically.

**Q: Is Undo/Redo unlimited?**  
A: Default max history is 1000 commands. Adjust via `EditCommandHistory::max_history_size`.

---

## Version History

| Version | Phase | Release | Notes |
|---------|-------|---------|-------|
| 1.0 | 6.5 | 2026-05 | Initial game engine foundation |
| 0.9 | 6 | 2026-04 | Layer and animation systems |
| 0.8 | 5 | Earlier | Sprite editor with tools |

---

**Last Updated**: Phase 6.5  
**Next Phase**: Phase 7 (Game Engine API Implementation)
