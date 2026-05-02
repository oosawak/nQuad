# Lineboy WASM Game

Pure Rust implementation of Lineboy compiled to WebAssembly.

## Build

```bash
./build.sh
```

This generates:
- `lineboy.js` - JavaScript wrapper
- `lineboy_bg.wasm` - WebAssembly binary (~15KB)

## Usage

```html
<script type="module">
  import init, { render, update } from './wasm-builds/lineboy/lineboy.js';
  
  await init();
  
  // Game loop
  setInterval(() => {
    update(left, right, jump);
    const pixels = render();
    // Draw to canvas...
  }, 1000/60);
</script>
```

## Design

- **No external crate dependencies** - Only `wasm-bindgen`
- **Fixed game loop** - Framebuffer-based rendering (160x120)
- **Pyxel palette** - 16-color indexed palette
- **Pure WASM** - All game logic in Rust, no JavaScript workarounds
