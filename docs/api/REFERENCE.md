# Nantaraquad API Reference

Complete API documentation for Nantaraquad game engine and Pyxel-compatible APIs.

**Last Updated**: Phase 12 (Text Rendering & Particle Systems)

---

## 📚 Table of Contents

1. [Core Game Engine](#core-game-engine)
2. [Drawing API](#drawing-api)
3. [Input API](#input-api)
4. [Particle System](#particle-system)
5. [Camera & Viewport](#camera--viewport)
6. [Audio API](#audio-api)
7. [Resource Management](#resource-management)
8. [Pyxel Compatibility Layer](#pyxel-compatibility-layer)

---

## Core Game Engine

### GameEngine

Main game engine structure for all games.

```rust
pub struct GameEngine {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub drawing: DrawingContext,
    pub input: InputState,
    pub camera: Camera,
    pub audio: AudioManager,
    pub particles: ParticleSystem,
}
```

#### Methods

**`new(width: u32, height: u32, fps: u32) -> Self`**
- Creates new game engine with specified dimensions and FPS
- Default: 160x120, 60 FPS
- Example:
  ```rust
  let engine = GameEngine::new(160, 120, 60);
  ```

**`update(&mut self, delta_ms: f32)`**
- Updates input state and particle system
- Call once per frame
- Parameter: time since last frame (milliseconds)

**`clear(&mut self, color: u8)`**
- Clears screen to solid color
- Parameter: palette color index (0-15)

**`frame_time_ms() -> f32`**
- Returns milliseconds per frame based on FPS
- Example: 60 FPS = 16.667ms

**`frames_for_duration(duration_ms: f32) -> u32`**
- Calculates frame count for given duration
- Example: `engine.frames_for_duration(1000.0)` returns 60 frames at 60 FPS

#### Integration Example

```rust
use nantaraquad::api::game::GameEngine;

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    
    loop {
        engine.update(16.667);  // 60 FPS
        
        // Your game logic here
        engine.drawing.pset(50, 50, 7);  // Draw white pixel
        engine.input.btn(Key::Up);        // Check input
        engine.particles.emit(x, y, dx, dy, color);  // Emit particle
    }
}
```

---

## Drawing API

### DrawingContext

High-level drawing API with palette support.

```rust
pub struct DrawingContext {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    palette: Vec<[u8; 4]>,  // RGBA colors
}
```

#### Basic Drawing

**`pset(x: i32, y: i32, col: u8)`**
- Draw single pixel
- Parameters:
  - `x, y`: Screen coordinates
  - `col`: Palette color (0-15)
- Bounds checking: pixels outside screen are ignored

**`pget(x: i32, y: i32) -> u8`**
- Read pixel color
- Returns palette index (0-15) or 0 if out of bounds

#### Shape Drawing

**`line(x1: i32, y1: i32, x2: i32, y2: i32, col: u8)`**
- Draw line using Bresenham algorithm
- Parameters: start point, end point, color

**`rect(x: i32, y: i32, w: u32, h: u32, col: u8)`**
- Draw rectangle outline
- Parameters: top-left position, dimensions, color

**`rectfill(x: i32, y: i32, w: u32, h: u32, col: u8)`**
- Draw filled rectangle
- Parameters: top-left position, dimensions, color

**`circle(x: i32, y: i32, r: i32, col: u8)`**
- Draw circle outline
- Parameters: center, radius, color

**`circfill(x: i32, y: i32, r: i32, col: u8)`**
- Draw filled circle
- Parameters: center, radius, color

#### Text Rendering

**`print(text: &str, x: i32, y: i32, color: u8)`**
- Draw text using bitmap font
- Parameters:
  - `text`: String to display
  - `x, y`: Top-left position
  - `color`: Text color (0-15)
- Character size: 4x6 pixels
- Supported: ASCII letters, numbers, punctuation
- Non-ASCII characters render as blank

```rust
ctx.print("Score: 100", 10, 10, 7);  // White text
ctx.print("Level: 5", 10, 20, 3);    // Green text
```

#### Screen Management

**`cls(col: u8)`**
- Clear entire screen to solid color
- Parameter: palette color index

**`clip(x: i32, y: i32, w: u32, h: u32)`**
- Set clipping rectangle (future feature)

#### Palette Management

**`set_palette_color(idx: u8, r: u8, g: u8, b: u8)`**
- Modify palette entry
- Parameters:
  - `idx`: Color index (0-15)
  - `r, g, b`: RGB values (0-255)
- Default palette: Pyxel 16-color

#### Drawing Modes (Future)

- `pal(col1: u8, col2: u8)` - Color substitution
- `blt()` - Sprite blitting with transformations
- `map()` - Tilemap rendering

---

## Input API

### InputState

Keyboard and input handling.

```rust
pub struct InputState {
    pressed: Vec<bool>,
    released: Vec<bool>,
}
```

#### Input Methods

**`btn(key: Key) -> bool`**
- Check if key is currently pressed
- Returns: true if held down
- Example:
  ```rust
  if input.btn(Key::Up) {
      player_y -= 1;
  }
  ```

**`btnp(key: Key) -> bool`**
- Check if key was pressed this frame
- Returns: true only on new key press (not held)
- Example:
  ```rust
  if input.btnp(Key::Space) {
      fire_bullet();  // Only once per press
  }
  ```

**`press_key(key: Key)`**
- Simulate key press (for testing)

**`release_key(key: Key)`**
- Simulate key release (for testing)

**`update_frame()`**
- Update input state for new frame
- Call once per game loop

### Key Codes

```rust
pub enum Key {
    // Direction
    Up, Down, Left, Right,
    
    // Main buttons
    Space, Enter, Escape, Tab,
    
    // Character keys
    A, B, C, D, E, // ... Z
    
    // Gamepad
    GamepadButtonA, GamepadButtonB,
    GamepadButtonX, GamepadButtonY,
}
```

#### Input Binding Example

```rust
use nantaraquad::api::input::Key;

// In update loop
if input.btn(Key::Left) || input.btn(Key::A) {
    player.move_left();
}

if input.btnp(Key::Space) {
    player.jump();
}

if input.btn(Key::Up) && input.btn(Key::Z) {
    player.dash_up();  // Combined input
}
```

---

## Particle System

### Particle

Individual particle structure.

```rust
pub struct Particle {
    pub x: f32, pub y: f32,        // Position
    pub dx: f32, pub dy: f32,      // Velocity
    pub color: u8,                 // Palette color
    pub life: u32,                 // Remaining lifetime (frames)
}
```

#### Particle Methods

**`new(x: f32, y: f32, dx: f32, dy: f32, color: u8, lifetime: u32) -> Self`**
- Create new particle
- Physics: velocity-based movement + gravity (0.2/frame)
- Lifetime: frame counter (decrements each update)

**`update(&mut self)`**
- Update particle position and apply physics
- Applies gravity and velocity
- Decrements lifetime
- Call from ParticleSystem::update()

**`is_alive() -> bool`**
- Check if particle is still active
- Returns: true if lifetime > 0

### ParticleSystem

Container for managing multiple particles.

```rust
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    max_particles: usize,
    default_lifetime: u32,
}
```

#### ParticleSystem Methods

**`new(capacity: usize) -> Self`**
- Create system with particle capacity limit
- Default capacity: 256
- Prevents unbounded memory growth

**`emit(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: u8)`**
- Emit particle with default lifetime
- Ignored if at capacity
- Example:
  ```rust
  particles.emit(player.x, player.y, 1.0, -2.0, 6);
  ```

**`emit_with_lifetime(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: u8, lifetime: u32)`**
- Emit particle with custom lifetime
- Allows fine-grained effect control
- Example:
  ```rust
  // Short-lived explosion effect
  for i in 0..8 {
      let angle = (i as f32 / 8.0) * 6.28;
      particles.emit_with_lifetime(
          x, y,
          angle.cos() * 50.0,
          angle.sin() * 50.0,
          8,  // Color
          20  // 20 frame lifetime
      );
  }
  ```

**`update(&mut self)`**
- Update all particles
- Removes dead particles automatically
- Call once per frame

**`draw(&self, ctx: &mut DrawingContext)`**
- Render all particles to screen
- Uses `pset()` for pixel-perfect rendering
- Call after drawing game world

**`count() -> usize`**
- Get current particle count
- Useful for debugging/profiling

**`set_default_lifetime(&mut self, lifetime: u32)`**
- Set default lifetime for `emit()` calls
- Default: 30 frames

**`clear(&mut self)`**
- Remove all particles immediately

#### Particle Physics

- **Gravity**: Constant 0.2 per frame (not physically accurate, tuned for visuals)
- **Velocity**: Applied per frame `x += dx`, `y += dy`
- **Lifetime**: Frame-based counter (not time-based)
- **Removal**: Automatic via `retain_mut()` when lifetime reaches 0

#### Particle Example

```rust
use nantaraquad::api::particles::ParticleSystem;

let mut particles = ParticleSystem::new(256);

// In game loop
particles.emit(100.0, 100.0, 0.5, -2.0, 7);
particles.update();
particles.draw(&mut drawing_context);
```

---

## Camera & Viewport

### Camera

View transformation for scrolling/panning.

```rust
pub struct Camera {
    pub x: f32, pub y: f32,        // Position
    width: u32, height: u32,
}
```

#### Camera Methods

**`new(width: u32, height: u32) -> Self`**
- Create camera for screen dimensions

**`set_position(x: f32, y: f32)`**
- Set camera position (world coordinates)

**`follow(target_x: f32, target_y: f32, speed: f32)`**
- Smoothly follow target
- Speed: 0.0-1.0 (0 = instant, 1 = no movement)

**`get_screen_coords(world_x: f32, world_y: f32) -> (i32, i32)`**
- Convert world to screen coordinates

#### Camera Usage

```rust
// Center camera on player
camera.set_position(player.x - 80.0, player.y - 60.0);

// Smoothly follow
camera.follow(player.x, player.y, 0.1);
```

---

## Audio API

### AudioManager

Sound and music playback.

```rust
pub struct AudioManager {
    // Internal state
}
```

#### Audio Methods

**`new() -> Self`**
- Create audio manager

**`load_sfx(name: &str, path: &str) -> Result<(), Box<dyn Error>>`**
- Load sound effect file
- Formats: WAV, OGG (via rodio)

**`load_bgm(name: &str, path: &str) -> Result<(), Box<dyn Error>>`**
- Load background music file

**`play_sfx(name: &str) -> Result<(), Box<dyn Error>>`**
- Play loaded sound effect

**`play_bgm(name: &str) -> Result<(), Box<dyn Error>>`**
- Play loaded music (loops)

**`stop_sfx()`**
- Stop all sound effects

**`stop_bgm()`**
- Stop background music

**`set_volume(volume: f32)`**
- Set master volume (0.0-1.0)

#### Audio Example

```rust
let mut audio = AudioManager::new();

// Load assets
audio.load_sfx("jump", "assets/jump.wav")?;
audio.load_bgm("level1", "assets/music.ogg")?;

// Play
audio.play_sfx("jump")?;
audio.play_bgm("level1")?;

// Control
audio.set_volume(0.8);
```

---

## Resource Management

### SpriteData

Sprite pixel data with color mode support.

```rust
pub struct SpriteData {
    width: u32, height: u32,
    pixels: Vec<u8>,  // or Vec<[u8; 4]> for RGBA
    color_mode: ColorMode,
}
```

#### ColorMode

```rust
pub enum ColorMode {
    Indexed256(Vec<[u8; 4]>),  // 8-bit palette
    FullColor,                  // Direct RGBA
}
```

### ResourcePackage

Bundle of sprites for serialization.

```rust
pub struct ResourcePackage {
    pub sprites: Vec<SpriteData>,
}
```

#### Resource Methods

**`save_package(path: &str, package: &ResourcePackage) -> Result<()>`**
- Serialize sprites to binary file (bincode)

**`load_package(path: &str) -> Result<ResourcePackage>`**
- Deserialize sprites from file

---

## Pyxel Compatibility Layer

Pyxel-compatible function stubs for game development.

### Available Functions

```rust
// Drawing
cls(col: u8)
pset(x: i32, y: i32, col: u8)
pget(x: i32, y: i32) -> Option<u8>
line(x1: i32, y1: i32, x2: i32, y2: i32, col: u8)
rect(x: i32, y: i32, w: u32, h: u32, col: u8)
rectfill(x: i32, y: i32, w: u32, h: u32, col: u8)
circle(x: i32, y: i32, r: i32, col: u8)
circfill(x: i32, y: i32, r: i32, col: u8)
spr(n: usize, x: f32, y: f32)

// Input
btn(key: Key) -> bool
btnp(key: Key) -> bool

// Camera
camera(x: f32, y: f32)
zoom(scale: f32)

// Audio
sfx(n: usize)
music(n: usize)
stop()
music_stop()

// Text
print(text: &str, x: i32, y: i32, col: u8)

// Utilities
frame_time() -> f32
frames_for_ms(ms: f32) -> u32
stat() -> String
```

### Implementation Status

| Category | Status | Notes |
|----------|--------|-------|
| Drawing | ✅ Core | pset, rect, circle, line, print implemented |
| Input | ✅ Full | btn, btnp working |
| Camera | ⏳ Planned | API defined, implementation pending |
| Audio | ✅ Working | Through AudioManager |
| Text | ✅ Full | Bitmap font rendering |
| Sprites | ⏳ Planned | Core data structures ready |

---

## Error Handling

### Error Types

```rust
// User-facing operations
Result<T, String>  // Clear error messages

// I/O operations  
Result<T, Box<dyn std::error::Error>>  // Detailed errors
```

### Common Error Cases

| Operation | Error Case | Recovery |
|-----------|-----------|----------|
| pset | Out of bounds | Ignored silently |
| draw_text | Non-ASCII char | Renders as blank |
| load_sfx | File not found | Returns Err |
| emit | Particle cap reached | Silently drops |

---

## Performance Considerations

### Memory

- **ParticleSystem**: Each particle ≈ 32 bytes
- **256 particles** ≈ 8 KB
- **DrawingContext**: 160x120 = 19.2 KB (indexed)

### CPU

- **Particle update**: O(n) per frame
- **Drawing**: O(pixels) for shapes
- **Text rendering**: O(chars) for string length

### Optimization Tips

1. **Limit particle count**: Use capacity control
2. **Batch drawing**: Combine similar operations
3. **Reuse buffers**: Avoid allocations in hot loops
4. **Profile**: Use `stat()` for performance data

---

## Migration Guide: Pyxel to Nantaraquad

For Pyxel game developers:

```rust
// Pyxel
pyxel.pset(10, 10, 7)

// Nantaraquad (via GameEngine)
engine.drawing.pset(10, 10, 7)

// Or use compatibility layer
use nantaraquad::api::pyxel;
pyxel::pset(10, 10, 7);  // Stubs - use DrawingContext instead
```

---

## Quick Start Examples

See `/docs/examples/` for complete samples:
- `simple_game.rs` - Minimal game loop
- `particle_effects.rs` - Particle system showcase
- `text_rendering.rs` - Text API examples
- `input_handling.rs` - Input patterns

---

## API Changelog

### Phase 12 (Current)
- ✅ Text rendering with bitmap font
- ✅ Generic ParticleSystem with physics
- ✅ GameEngine integration complete

### Phase 11
- ✅ WASM support
- ✅ Web deployment

### Phase 10  
- ✅ Editor core architecture

---

**Need help?** See `/docs/guides/` for tutorials and best practices.
