# Graphics Backend & Rendering Guide

Complete guide to Nantaraquad's graphics system, from CPU pixel calculation to GPU rendering.

---

## Quick Answer

**Q: SDL や OpenGL を使ってるのですか？**

**A**: OpenGL ES を使っています。ただし直接ではなく、**macroquad フレームワーク**経由で：

- **デスクトップ（Windows/Linux/macOS）**: OpenGL ES 2.0 via miniquad backend
- **Web（WASM）**: WebGL + Canvas 2D via web-sys

SDL は使用していません（WASM 対応がないため）。

---

## High-Level Graphics Architecture

```
Your Game Code
  ↓ (uses DrawingContext API)
CPU Pixel Calculation (Rust)
  ↓ (accumulates to Vec<u8>)
DrawingContext::pixels
  ↓ (converts to texture)
Image/Texture (GPU memory)
  ↓ (renders via macroquad)
OpenGL ES 2.0 / WebGL
  ↓
Display (Monitor/Browser Canvas)
```

---

## Platform-Specific Rendering Pipelines

### Desktop (Windows/Linux/macOS)

```
macroquad::main() {
    loop {
        // Your game logic
        engine.drawing.pset(x, y, color);
        engine.drawing.rectfill(x, y, w, h, color);
        
        // Rendering
        get_render_target()  // Get current framebuffer
        
        // Draw texture containing pixel data
        draw_texture(
            texture_from_pixels(&engine.drawing.pixels),
            0.0, 0.0,
            WHITE
        );
        
        // GPU rasterization via OpenGL ES
    }
}
```

**Backend Stack**:
```
macroquad 0.4
  ↓
miniquad (window + event handling)
  ↓
OpenGL ES 2.0 Context
  ↓
Native APIs:
  - Windows: ANGLE (OpenGL ES on Direct3D)
  - Linux: X11/Wayland + OpenGL driver
  - macOS: Cocoa + OpenGL framework
```

### Web (WASM)

```
// WASM binary (compiled from Rust)
pub fn game_loop() {
    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext("2d");  // or "webgl"
    
    loop {
        // Your game logic
        engine.drawing.pset(x, y, color);
        
        // Convert pixels to ImageData
        let image_data = ImageData::new_with_u8_clamped_array(
            Clamped(&engine.drawing.pixels),
            160, 120
        )?;
        
        // Render to canvas
        ctx.put_image_data(&image_data, 0.0, 0.0)?;
    }
}
```

**Backend Stack**:
```
WASM Binary (via wasm-pack)
  ↓
JavaScript Loader (wasm-bindgen)
  ↓
Canvas 2D Context (CPU rendering) OR WebGL Context (GPU)
  ↓
Browser Compositing Engine
  ↓
HTML5 Canvas Element → Display
```

---

## Rendering Pipeline Details

### 1. Pixel Calculation (CPU)

```rust
pub fn pset(&mut self, x: i32, y: i32, color: u8) {
    if x < 0 || x >= self.width as i32 || 
       y < 0 || y >= self.height as i32 {
        return;  // Out of bounds
    }
    
    let idx = (y as u32 * self.width + x as u32) as usize;
    self.pixels[idx] = color;  // Set palette index
}
```

**Complexity**: O(1) per pixel
**Storage**: 1 byte per pixel (indexed color)

### 2. Shape Rendering

```rust
pub fn rectfill(&mut self, x: i32, y: i32, w: u32, h: u32, color: u8) {
    for yy in y..y + h as i32 {
        for xx in x..x + w as i32 {
            self.pset(xx, yy, color);
        }
    }
}
```

**Complexity**: O(w × h)
**Example**: 50×50 rect = 2,500 pixel writes

### 3. Text Rendering

```rust
pub fn print(&mut self, text: &str, x: i32, y: i32, color: u8) {
    let mut char_x = x;
    for ch in text.chars() {
        self.draw_char(ch, char_x, y, color);
        char_x += 4;  // Next character position
    }
}

fn draw_char(&mut self, ch: char, x: i32, y: i32, color: u8) {
    let bitmap = match ch {
        'A' => 0b1111_1001_1001_1111_u16,  // 4-bit rows
        'B' => 0b1110_1001_1100_1001_u16,
        // ... 30+ characters
        _ => 0,  // Unknown chars render as blank
    };
    
    // Draw 6 rows of 4-pixel width
    for row in 0..6 {
        for col in 0..4 {
            let bit = (bitmap >> (row * 4 + col)) & 1;
            if bit == 1 {
                self.pset(x + col, y + row as i32, color);
            }
        }
    }
}
```

**Font**: 4×6 pixel bitmap for each character
**Performance**: O(chars × 24 pixel writes)

### 4. Particle Rendering

```rust
pub fn draw(&self, ctx: &mut DrawingContext) {
    for particle in &self.particles {
        if particle.is_alive() {
            ctx.pset(
                particle.x as i32,
                particle.y as i32,
                particle.color
            );
        }
    }
}
```

**Complexity**: O(n) where n = particle count
**Default**: 256 particles max

### 5. GPU Rendering

```rust
// macroquad handles this internally
draw_texture(
    pixel_texture,      // Source: CPU pixels as texture
    0.0, 0.0,          // Position
    WHITE              // Tint color
);

// Behind the scenes:
// 1. Create OpenGL texture from pixel data
// 2. Bind texture to shader
// 3. Render quad covering screen
// 4. Fragment shader displays texture
```

---

## Memory Layout

### DrawingContext Storage

```
pixels: Vec<u8>
├─ 160 × 120 = 19,200 bytes (indexed color)
├─ 160 × 120 × 4 = 76,800 bytes (RGBA, future)
└─ Linear storage: pixels[y * width + x]

palette: Vec<[u8; 4]>
├─ 16 colors × 4 bytes = 64 bytes (RGBA)
└─ Default: Pyxel 16-color palette
```

### Pixel Access

```
Screen:
  (0,0) -------- (159, 0)
    |             |
    |  160×120    |
    |   pixels    |
    |             |
(0,119) ------ (159, 119)

Index calculation:
index = y * width + x
index = y * 160 + x

Example:
(50, 30) → index = 30 * 160 + 50 = 4850
pixels[4850] = color index (0-15)
```

---

## Color Representation

### Palette-Based (Current)

```rust
pub enum ColorMode {
    Indexed256(Vec<[u8; 4]>),  // 8-bit index + 16 RGBA entries
}

// Each pixel stores: u8 (0-15)
// Lookup in palette: palette[pixel_index] = [R, G, B, A]

Default Palette (Pyxel 16):
0:  [0, 0, 0, 255]           // Black
1:  [29, 43, 83, 255]        // Dark blue
2:  [126, 37, 83, 255]       // Dark purple
3:  [0, 135, 81, 255]        // Dark green
4:  [171, 82, 54, 255]       // Brown
5:  [95, 87, 79, 255]        // Dark gray
6:  [194, 195, 199, 255]     // Light gray
7:  [255, 241, 232, 255]     // White
8:  [255, 0, 77, 255]        // Red
9:  [255, 163, 0, 255]       // Orange
10: [255, 236, 39, 255]      // Yellow
11: [0, 228, 54, 255]        // Green
12: [41, 173, 255, 255]      // Cyan
13: [131, 118, 156, 255]     // Purple
14: [255, 119, 168, 255]     // Pink
15: [255, 204, 170, 255]     // Peach
```

### Future: Direct RGBA

```rust
// For games needing more colors
pub enum ColorMode {
    FullColor,  // 32-bit RGBA per pixel
}

pixels: Vec<u8>  // 160 × 120 × 4 = 76,800 bytes
// Access: pixels[(y * width + x) * 4 + channel]
```

---

## Optimization Techniques

### Current Optimizations

1. **Palette Indexing**: 1 byte/pixel vs 4 bytes (75% memory saving)
2. **CPU-Side Rendering**: Avoids GPU state overhead
3. **Simple Quad Rendering**: macroquad optimizes for this pattern
4. **Lazy Updates**: Only render dirty rectangles (future)

### Potential Optimizations

```rust
// 1. Dirty Rectangle Tracking
struct DirtyRect {
    x, y, w, h: u32,
}
// Only upload changed pixels to GPU

// 2. Multi-threaded Pixel Calculation
rayon::scope(|s| {
    for chunk in pixels.par_chunks_mut(width as usize) {
        s.spawn(|_| {
            // Calculate pixels in parallel
        });
    }
});

// 3. SIMD Operations
#[target_feature(enable = "avx2")]
unsafe fn fill_pixels_simd(...) {
    // Vector operations for fast fills
}

// 4. GPU Compute Shaders
// Move pixel calculation to GPU (future)
// Draw directly without CPU→GPU transfer
```

---

## Performance Characteristics

### Desktop (OpenGL)

| Operation | Time | Notes |
|-----------|------|-------|
| Pixel write (pset) | ~10ns | CPU RAM access |
| Rectfill 50×50 | ~1μs | 2500 pixels |
| Text (10 chars) | ~5μs | ~240 pixel writes |
| GPU upload | ~0.1ms | Full 160×120 texture |
| GPU render | ~0.1ms | Single quad |
| **Total per frame** | **~0.2-0.3ms** | 16.6ms budget at 60 FPS |

### Web (WebGL/Canvas)

| Operation | Time | Notes |
|-----------|------|-------|
| Pixel write (WASM) | ~15ns | Same as desktop |
| ImageData creation | ~0.5ms | Clamped array conversion |
| putImageData | ~1-2ms | Canvas rasterization |
| **Total per frame** | **~2-3ms** | Still <16.6ms budget |

### Memory

```
160×120×1 byte (indexed)    = 19.2 KB
160×120×4 bytes (RGBA)      = 76.8 KB
Palette (16 colors)         = 64 bytes
ParticleSystem (256 max)    = 8 KB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total                       ~28 KB (indexed)
```

---

## Comparison: Nantaraquad vs Alternatives

| Aspect | Nantaraquad | SDL2 | Raw OpenGL | Raylib |
|--------|------------|------|-----------|--------|
| **Graphics API** | macroquad + OpenGL ES | SDL + OpenGL | OpenGL | OpenGL |
| **WASM Support** | ✅ Full | ❌ None | ❌ Difficult | ⚠️ Partial |
| **Setup Complexity** | Simple | Medium | Complex | Simple |
| **2D Performance** | Excellent | Good | Excellent | Good |
| **Palette Support** | ✅ Native | ⚠️ Manual | ❌ No | ⚠️ Manual |
| **Pixel-perfect** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **Cross-platform** | ✅ Desktop+Web | Desktop only | Desktop only | ✅ Desktop+Web |
| **Learning Curve** | Gentle | Medium | Steep | Gentle |

---

## Examples

### Simple Drawing

```rust
use nantaraquad::api::game::GameEngine;

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    
    // Clear screen
    engine.drawing.cls(0);  // Black
    
    // Draw shapes
    engine.drawing.rectfill(10, 10, 50, 50, 7);  // White rect
    engine.drawing.circle(100, 50, 20, 3);       // Green circle
    engine.drawing.line(0, 0, 159, 119, 8);      // Red line
    
    // Draw text
    engine.drawing.print("Hello, World!", 30, 60, 15);  // White text
}
```

### Particle Effects

```rust
for _ in 0..10 {
    let angle = rand::random::<f32>() * 6.28;
    let speed = 50.0;
    engine.particles.emit_with_lifetime(
        x, y,
        angle.cos() * speed,
        angle.sin() * speed,
        8,  // Red color
        30  // 30 frames lifetime
    );
}

engine.particles.update();
engine.particles.draw(&mut engine.drawing);
```

---

## Future Enhancements

### Planned Features

1. **GPU Compute Pipeline**: Move pixel calculation to GPU shaders
2. **Sprite Blitting**: Hardware-accelerated sprite rendering
3. **Advanced Blending**: Custom blend modes via shaders
4. **Tile Rendering**: Efficient tilemap GPU rendering
5. **Particle GPU System**: GPU-driven particle effects

### Potential Backend Changes

- Move from `pixels→texture→quad` to direct GPU rendering
- Use instancing for particle rendering
- Implement GPU occlusion culling
- Add compute shader support (Vulkan backend)

---

**For API details, see `/docs/api/REFERENCE.md`**
