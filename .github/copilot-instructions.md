# Nantaraquad - Copilot Instructions

Nantaraquad is a Rust library for game graphics and sprite resource management, built on top of the macroquad framework. It provides a structured way to manage sprite data, handle different color modes (indexed 256-color and full RGBA), and serialize/deserialize sprite resources.

## Build & Test Commands

### Build
```bash
cargo build                    # Debug build
cargo build --release         # Optimized release build
```

### Testing
```bash
cargo test                     # Run all tests (none exist yet)
cargo test --lib             # Run library tests only
cargo test -- --nocapture    # Run with output visible
```

### Linting & Code Quality
```bash
cargo clippy                   # Lint check (catches common mistakes and style issues)
cargo clippy --fix            # Auto-fix some issues
cargo check                    # Fast syntax/type check without full compilation
cargo fmt                      # Format code with rustfmt
cargo fmt --check             # Check formatting without modifying
```

### Documentation
```bash
cargo doc --open              # Generate and open HTML docs for the project and dependencies
cargo test --doc              # Run doc tests
```

## Architecture

### Module Structure
- **`resource/`** - Sprite data management and serialization
  - `data.rs` - Core data structures: `ColorMode`, `SpriteData`, `ResourcePackage`
  - `serialize.rs` - Binary serialization (bincode) for resource packages
- **`api/`** - High-level API for sprite drawing (planned)
- **`render/`** - Graphics rendering pipeline (planned, likely macroquad integration)
- **`core/`** - Core utilities and game loop (planned)
- **`editor/`** - Editor interface (planned)

### Key Data Types

#### ColorMode
Two color space options:
- `Indexed256(Vec<[u8; 4]>)` - 8-bit indexed palette mode with RGB+Alpha palette
- `FullColor` - Direct RGBA color, 4 bytes per pixel

#### SpriteData
Stores pixel data for a single sprite:
- Dimensions: `width` and `height` (u32)
- Pixel storage: 1 byte per pixel (indexed) or 4 bytes per pixel (full color)
- Memory layout: row-major (y * width + x)

#### ResourcePackage
Container for multiple sprites, serializable to binary format:
- Future: planned support for tilemaps, sounds, fonts

### Data Flow
1. Create `SpriteData` with dimensions and color mode
2. Use `set_pixel()` to write pixel data (validates bounds and color size)
3. Use `get_pixel()` to read pixel data
4. Add sprites to `ResourcePackage`
5. Save/load packages with `save_package()` / `load_package()`

## Key Conventions

### Error Handling
- Use `Result<T, String>` for user-facing operations (e.g., `set_pixel()`)
- Use `Result<T, Box<dyn std::error::Error>>` for I/O operations (e.g., serialization)
- Provide context in error messages with `format!()`

### Module Organization
- Modules are declared in `mod.rs` and publicly re-exported (`pub use`)
- Keep implementation details in separate files (e.g., `data.rs`, `serialize.rs`)
- Use `pub use` in module root to define the public API

### Comments & Documentation
- Japanese comments are acceptable (used throughout the codebase)
- Use doc comments (`///`) for public APIs
- Explain non-obvious behavior like pixel memory layout

### Pixel Access
- Always check bounds before accessing pixels
- Remember the byte size varies: Indexed256 = 1 byte, FullColor = 4 bytes
- Offset calculation: `offset = y * width + x` (row-major order)

### Dependencies
- **macroquad** - Game framework (not yet used in core)
- **serde/bincode** - Serialization
- **lazy_static** - Lazy initialization (currently unused)
- **tempfile** - Testing (dev-dependency)

## Development Notes

### Current State
- Core data structures and serialization are implemented
- No tests yet (good opportunity to add comprehensive test coverage)
- Examples directory is empty (good place for usage examples)
- Editor and render modules are placeholders

### Adding Tests
Tests should go in the `tests/` directory (integration tests) or as module tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_creation() {
        // test code
    }
}
```

### Adding Examples
Add executable examples to `examples/` directory. They can be run with:
```bash
cargo run --example example_name
```
