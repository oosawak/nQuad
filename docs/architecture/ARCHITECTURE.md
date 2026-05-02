# Nantaraquad Architecture

Complete architecture guide for understanding Nantaraquad's design and structure.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Your Game                                │
│                   (Lineboy, Cubeboy, etc.)                     │
└──────────────────────┬──────────────────────────────────────────┘
                       │ uses
┌──────────────────────▼──────────────────────────────────────────┐
│                    GameEngine API Layer                         │
│  ┌─────────────┬───────────────┬─────────────┬──────────────┐  │
│  │ DrawingAPI  │ InputAPI      │ ParticleAPI │ CameraAPI    │  │
│  │ (drawing)   │ (input)       │ (particles) │ (camera)     │  │
│  │ + Text      │ + Keyboard    │ + Physics   │ + Viewport   │  │
│  │ + Shapes    │ + Gamepad     │ + Lifetime  │ + Transform  │  │
│  └─────────────┴───────────────┴─────────────┴──────────────┘  │
└──────────────────────┬──────────────────────────────────────────┘
                       │ integrates
┌──────────────────────▼──────────────────────────────────────────┐
│              Core Module Layer                                  │
│  ┌────────────┬─────────┬────────────┬──────────┬────────────┐ │
│  │ Drawing    │ Input   │ Particle   │ Camera   │ Audio      │ │
│  │ Context    │ State   │ System     │ Control  │ Manager    │ │
│  └────────────┴─────────┴────────────┴──────────┴────────────┘ │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │            Resource Management (Sprites, Data)             │ │
│  └────────────────────────────────────────────────────────────┘ │
└──────────────────────┬──────────────────────────────────────────┘
                       │ depends on
┌──────────────────────▼──────────────────────────────────────────┐
│           External Framework Layer                              │
│  ┌──────────────────┬──────────────┬─────────────────────────┐ │
│  │ macroquad        │ wasm-bindgen │ web-sys (WASM)          │ │
│  │ (Game loop,      │ (Rust→JS)    │ (DOM, Canvas, Events)   │ │
│  │  Graphics,       │              │                         │ │
│  │  Input)          │              │                         │ │
│  └──────────────────┴──────────────┴─────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

---

## Module Structure

```
src/
├── api/                          # Public game development API
│   ├── mod.rs                   # API exports
│   ├── game.rs                  # GameEngine - main integration
│   ├── drawing.rs               # Drawing context + text rendering
│   ├── input.rs                 # Input state management
│   ├── particles.rs             # Generic particle system (NEW)
│   ├── camera.rs                # Camera control
│   ├── framework.rs             # Frame timing utilities
│   ├── pyxel.rs                 # Pyxel compatibility stubs
│   └── audio_compat.rs          # Audio wrapper
│
├── resource/                     # Data structures & serialization
│   ├── mod.rs
│   ├── data.rs                  # SpriteData, ColorMode, ResourcePackage
│   └── serialize.rs             # Bincode serialization
│
├── editor/                       # Sprite editor (not for game use)
│   ├── mod.rs
│   ├── document.rs              # SpriteDocument model
│   ├── celmodel.rs              # Cell data structures
│   ├── layers.rs                # Layer management
│   ├── animation.rs             # Animation system
│   ├── file.rs                  # Save/load operations
│   └── ... (other editor modules)
│
├── core/                         # Internal utilities
│   ├── mod.rs
│   └── global.rs                # Global state (INTERNAL)
│
├── audio/                        # Audio system
│   ├── mod.rs
│   └── manager.rs               # AudioManager
│
└── lib.rs                        # Root export (public API)
```

---

## Component Details

### 1. GameEngine (api/game.rs)

**Purpose**: Main integration point for all subsystems

**Responsibility**:
- Own DrawingContext, InputState, Camera, AudioManager
- Manage ParticleSystem
- Provide unified update() call
- Calculate frame timing

**Relationships**:
```
GameEngine
  ├─ uses→ DrawingContext (rendering)
  ├─ uses→ InputState (input handling)
  ├─ owns→ ParticleSystem (effects)
  ├─ uses→ Camera (viewport)
  └─ uses→ AudioManager (sound)
```

**Design Decision**: Public ownership of subsystems via `pub` fields
- ✅ Games can directly access `engine.drawing`, `engine.particles`
- ❌ Reduces encapsulation but improves usability

### 2. DrawingContext (api/drawing.rs)

**Purpose**: Pixel-level graphics rendering

**Key Features**:
- Palette-based (0-15 colors)
- Pixel storage: `Vec<u8>` for indexed colors
- Bitmap font for text (4x6 pixels)
- Shape primitives (line, rect, circle)

**Text Rendering**:
```rust
// Bitmap font lookup (30+ characters)
fn draw_char(&self, ch: char, x: i32, y: i32, color: u8) {
    match ch {
        'A' => {
            // 4-bit per row bitmap
            let bitmap = 0b1111_1001_1001_1111_u16;
            // ... render 6 rows
        }
        // ... other characters
    }
}
```

**Constraints**:
- 160x120 default resolution
- No sprite blitting (yet)
- No scaling/rotation (yet)

### 3. InputState (api/input.rs)

**Purpose**: Keyboard/gamepad input handling

**State Tracking**:
```
Frame N:          Frame N+1:
┌─────────────┐   ┌─────────────┐
│ pressed[Up] │   │ pressed[Up] │
│ released[]  │   │ released[]  │
└─────────────┘   └─────────────┘
     ↓                ↓
update_frame()  update_frame()
     ↓                ↓
btnp(Up) = true  btnp(Up) = false (key held)
```

**Key Methods**:
- `btn()` - key currently held
- `btnp()` - new key press (frame edge)
- `update_frame()` - reset per-frame state

### 4. ParticleSystem (api/particles.rs)

**Purpose**: Manage particle effects with physics

**Architecture**:
```
ParticleSystem {
    particles: Vec<Particle>  ← dynamic array
    max_particles: 256        ← capacity limit
    default_lifetime: 30      ← frame-based
}

Particle {
    x, y: f32                 ← position
    dx, dy: f32               ← velocity
    color: u8                 ← palette index (0-15)
    life: u32                 ← remaining frames
}
```

**Update Loop**:
```rust
particles.update() {
    for particle in &mut particles {
        particle.x += particle.dx;      // Velocity
        particle.y += particle.dy;
        particle.dy += 0.2;             // Gravity
        particle.life -= 1;             // Decay
    }
    particles.retain_mut(|p| p.is_alive());  // Clean dead
}
```

**Physics**:
- Gravity: 0.2 per frame (tuned for visuals, not realism)
- No air resistance, no collision
- Frame-based lifetime (not time-based) for predictability

### 5. Camera (api/camera.rs)

**Purpose**: Viewport transformation for scrolling

**State**:
```
Camera {
    x, y: f32        ← world position
    width, height    ← viewport size
}
```

**Usage**:
```rust
// Center on player
camera.set_position(player.x - 80, player.y - 60);

// Screen coordinates = World coordinates - Camera position
screen_x = world_x - camera.x
screen_y = world_y - camera.y
```

### 6. AudioManager (audio/manager.rs)

**Purpose**: Sound and music playback

**API**:
```rust
load_sfx/load_bgm()   ← Load audio files
play_sfx/play_bgm()   ← Start playback
stop_sfx/stop_bgm()   ← Stop playback
set_volume()          ← Master volume control
```

**Backend**: rodio (cross-platform audio)

---

## Data Flow: Rendering Pipeline

```
Game Loop (macroquad):
    │
    ├─ Input Event (KeyDown "ArrowUp")
    │  └─→ InputState::press_key()
    │
    ├─ Game Update
    │  ├─ Check input: engine.input.btn(Key::Up)
    │  ├─ Update particles: engine.particles.update()
    │  └─ (Your game logic)
    │
    ├─ Rendering
    │  ├─ Clear: engine.drawing.cls(0)
    │  ├─ Draw shapes: engine.drawing.rectfill()
    │  ├─ Draw text: engine.drawing.print()
    │  ├─ Render particles: engine.particles.draw()
    │  └─ Get pixels: DrawingContext::pixels
    │
    └─ Display (macroquad)
       └─ Copy pixels to canvas/screen
```

---

## Data Structures

### Pixel Storage

```rust
// Indexed 256 (8-bit palette)
pixels: Vec<u8>         // 160 * 120 = 19,200 bytes

// FullColor (direct RGBA)
pixels: Vec<u8>         // 160 * 120 * 4 = 76,800 bytes
```

### Palette

```rust
palette: Vec<[u8; 4]>   // 16 colors × 4 bytes (RGBA)

// Default: Pyxel 16-color palette
[
    [0, 0, 0, 255],           // 0: Black
    [29, 43, 83, 255],        // 1: Dark blue
    // ... 14 more colors
    [255, 255, 255, 255],     // 15: White
]
```

### Resource Organization

```
ResourcePackage {
    sprites: Vec<SpriteData>
}

SpriteData {
    width, height: u32
    pixels: Vec<u8>          // Pixel data
    color_mode: ColorMode
}
```

---

## Game Development Workflow

### 1. Setup

```rust
use nantaraquad::api::game::GameEngine;

let mut engine = GameEngine::new(160, 120, 60);
```

### 2. Game Loop

```rust
fn update(&mut self, input: &InputState) {
    if input.btnp(Key::Space) {
        self.state = Playing;
    }
}

fn render(&self, drawing: &mut DrawingContext) {
    drawing.cls(0);  // Clear
    drawing.rectfill(10, 10, 50, 50, 7);  // Draw
}

// Main loop (macroquad)
loop {
    engine.update(dt);
    game.update(&engine.input);
    game.render(&mut engine.drawing);
    engine.particles.draw(&mut engine.drawing);
}
```

### 3. Publishing

```bash
# Desktop
cargo run --example myGame

# Web (WASM)
wasm-pack build --target web
python3 -m http.server 8000
```

---

## Key Design Decisions

### 1. Palette-Based Graphics

**Why**: Retro aesthetic, reduced memory, simpler rendering

**Trade-off**: Limited colors (16) vs. realistic graphics

### 2. Frame-Based Particle Lifetime

**Why**: Predictable, easy to debug, matches game loop frequency

**Trade-off**: Not time-based (but `dt` can adjust in future)

### 3. Public Field Access

**Why**: Simpler API for games, less boilerplate

**Trade-off**: Less encapsulation, harder to change internals

### 4. Vec-Based Particles

**Why**: Simple, cache-friendly, auto-cleanup

**Trade-off**: Not suitable for extremely large counts (but 256 sufficient)

### 5. Pyxel Compatibility Layer

**Why**: Easy migration for existing Pyxel games

**Trade-off**: Stubs require games to use DrawingContext directly

---

## Performance Characteristics

### Memory Usage

| Component | Size | Notes |
|-----------|------|-------|
| DrawingContext (160×120) | 19.2 KB | Indexed, 1 byte/pixel |
| ParticleSystem (256 cap) | 8 KB | 32 bytes per particle |
| GameEngine | ~50 KB | All subsystems combined |

### CPU Usage

| Operation | Complexity | Per Frame |
|-----------|------------|-----------|
| Particle update | O(n) | ~256 particles |
| Drawing pixel | O(1) | Constant time |
| Text rendering | O(chars) | 4×6 per char |
| Input check | O(1) | Constant |

### Optimization Opportunities

1. **SIMD**: Vectorize particle updates
2. **Batching**: Combine shape draws
3. **Caching**: Pre-render static sprites
4. **Dirty rect**: Only update changed pixels

---

## Future Architecture Changes

### Planned Enhancements

1. **Sprite Blitting**
   - Source rectangle clipping
   - Rotation/scaling
   - Transparency modes

2. **Advanced Text**
   - Custom font support
   - Unicode rendering
   - Text metrics (width, height)

3. **Tile Mapping**
   - map() API
   - Infinite scrolling
   - Layer ordering

4. **Scene Management**
   - Scene trait implementation
   - Transition system
   - State machines

### Potential Refactoring

- Extract traits for drawing, input (easier testing)
- Use Arc<Mutex<>> for shared state (if needed)
- Streaming resource loading (for large games)

---

## Testing & Debugging

### Unit Tests

Located in each module (`#[cfg(test)]`):
- ParticleSystem: 8 tests
- GameEngine: 3 tests
- Input handling: (future)

### Integration Tests

Located in `tests/`:
- Full game loop simulation
- Rendering pipeline verification
- Cross-platform compatibility

### Debugging Tools

```rust
// Performance stats
println!("Particles: {}", engine.particles.count());

// Input tracing
if input.btnp(Key::Up) { println!("Pressed Up"); }

// Visual debugging
engine.drawing.circle(x, y, 5, 15);  // Hitbox visualization
```

---

## External Dependencies

### Runtime

- **macroquad 0.4**: Game framework
- **rodio 0.17**: Audio playback
- **serde 1.0**: Serialization
- **bincode 1.3**: Binary format

### Build (WASM)

- **wasm-bindgen 0.2**: Rust↔JavaScript
- **web-sys 0.3**: Web APIs
- **gloo-timers 0.3**: Async timers

### Development

- **tempfile**: Testing utilities

---

**For implementation details, see the source code in `src/api/`.**
