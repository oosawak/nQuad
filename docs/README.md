# Nantaraquad Documentation Index

Complete guide to all Nantaraquad documentation resources.

---

## 📚 Organization

```
docs/
├── api/                              # API Reference
│   └── REFERENCE.md                 # Complete API documentation
│
├── architecture/                     # System Design
│   └── ARCHITECTURE.md              # Architecture & design patterns
│
├── guides/                           # How-to Guides & Tutorials
│   ├── GRAPHICS.md                  # Graphics backend & rendering
│   ├── GETTING_STARTED.md           # Quick start guide (TODO)
│   ├── GAME_DEVELOPMENT.md          # Game dev patterns (TODO)
│   └── PERFORMANCE.md               # Performance tuning (TODO)
│
└── examples/                         # Code Examples
    ├── simple_game.rs               # Minimal game (TODO)
    ├── particle_effects.rs          # Particle showcase (TODO)
    ├── text_rendering.rs            # Text API examples (TODO)
    └── input_handling.rs            # Input patterns (TODO)
```

---

## 🎯 Where to Start

### New to Nantaraquad?
1. **Start here**: `/docs/guides/GETTING_STARTED.md` (in progress)
2. Read: `/docs/architecture/ARCHITECTURE.md` (overview)
3. Reference: `/docs/api/REFERENCE.md` (API details)

### Building a Game
1. Read: `/docs/guides/GAME_DEVELOPMENT.md` (patterns & best practices)
2. Check: `/docs/examples/` (working code samples)
3. Reference: `/docs/api/REFERENCE.md` (function details)

### Graphics Questions
1. Read: `/docs/guides/GRAPHICS.md` (rendering pipeline)
2. See: `/docs/architecture/ARCHITECTURE.md` section "DrawingContext"
3. Reference: `/docs/api/REFERENCE.md` section "Drawing API"

### Performance Optimization
1. Read: `/docs/guides/PERFORMANCE.md` (optimization techniques)
2. Check: `/docs/guides/GRAPHICS.md` section "Performance Characteristics"
3. Profile: Use `/docs/api/REFERENCE.md` `stat()` function

---

## 📖 Documentation Files

### Core Documentation

#### `/docs/api/REFERENCE.md` (Phase 12)
**Complete API Reference**
- Core Game Engine API
- Drawing API (pset, rect, circle, line, print)
- Input API (btn, btnp)
- Particle System API
- Camera & Viewport
- Audio API
- Resource Management
- Pyxel Compatibility Layer
- Error Handling
- Performance Considerations
- Quick Start Examples

**Use when**: You need to know a specific function's signature, parameters, return value, or example usage.

#### `/docs/architecture/ARCHITECTURE.md` (Phase 12)
**System Design & Architecture**
- High-Level Architecture (diagrams)
- Module Structure (folder layout & responsibilities)
- Component Details (GameEngine, DrawingContext, InputState, ParticleSystem, Camera, AudioManager)
- Data Flow (rendering pipeline)
- Data Structures (pixel storage, palette, resources)
- Game Development Workflow
- Key Design Decisions (why palette-based? why frame-based lifetime?)
- Performance Characteristics
- Future Architecture Changes
- Testing & Debugging
- External Dependencies

**Use when**: You want to understand the overall system design, how components fit together, or design rationale.

#### `/docs/guides/GRAPHICS.md` (Phase 12)
**Graphics Backend & Rendering Guide**
- Quick Answer (OpenGL ES via macroquad)
- High-Level Graphics Architecture
- Platform-Specific Rendering Pipelines (Desktop/Web)
- Rendering Pipeline Details (CPU→GPU)
- Memory Layout (pixel storage, index calculation)
- Color Representation (palette-based & future RGBA)
- Optimization Techniques (current & potential)
- Performance Characteristics (Desktop/Web/Memory)
- Comparison with Alternatives (SDL, OpenGL, Raylib)
- Examples (simple drawing, particle effects)
- Future Enhancements

**Use when**: You have graphics-specific questions, or want to understand how pixels get drawn to screen.

### Supporting Documentation

#### Root Level Docs (Already Exist)

- **README.md** - Project overview & quick start
- **API_COMPREHENSIVE.md** - Pyxel API status table
- **DEVELOPMENT_PLAN.md** - Phase planning
- **TEXT_PARTICLE_IMPLEMENTATION.md** - Feature details
- **TEXT_PARTICLE_SPEC.md** - Specifications
- **WEB_MOBILE_CONTROLLER.md** - Mobile UI guide
- **TOUCH_INPUT_SPEC.md** - Touch input details
- **PYXEL_COMPAT_API.md** - Pyxel API guide
- **PHASE_*.md** - Completion reports

---

## 🗺️ Topic-Based Navigation

### By Topic

**Graphics & Rendering**
- `/docs/guides/GRAPHICS.md` - Rendering pipeline
- `/docs/api/REFERENCE.md` - Drawing API
- `/docs/architecture/ARCHITECTURE.md` - Component design

**Game Development**
- `/docs/guides/GAME_DEVELOPMENT.md` (TODO)
- `/docs/examples/` - Code samples
- `/docs/api/REFERENCE.md` - Complete API

**Input Handling**
- `/docs/api/REFERENCE.md` - Input API section
- `/docs/examples/input_handling.rs` (TODO)
- TOUCH_INPUT_SPEC.md - Mobile input details

**Particle Effects**
- `/docs/api/REFERENCE.md` - Particle System API
- `/docs/examples/particle_effects.rs` (TODO)
- TEXT_PARTICLE_IMPLEMENTATION.md - Implementation details

**Text Rendering**
- `/docs/api/REFERENCE.md` - Text API (print)
- `/docs/examples/text_rendering.rs` (TODO)
- TEXT_PARTICLE_IMPLEMENTATION.md - Implementation details

**Web Deployment**
- `/docs/guides/GETTING_STARTED.md` (TODO)
- `/web/README.md` - Web version details
- WEB_MOBILE_CONTROLLER.md - Mobile controller UI

**Performance**
- `/docs/guides/PERFORMANCE.md` (TODO)
- `/docs/guides/GRAPHICS.md` - Performance characteristics
- `/docs/architecture/ARCHITECTURE.md` - Performance section

### By Audience

**Game Developers**
1. README.md (overview)
2. `/docs/guides/GAME_DEVELOPMENT.md` (patterns)
3. `/docs/examples/` (code)
4. `/docs/api/REFERENCE.md` (details)

**Engine Contributors**
1. `/docs/architecture/ARCHITECTURE.md` (design)
2. Source code in `src/api/`
3. `/docs/api/REFERENCE.md` (public API)

**Graphics Programmers**
1. `/docs/guides/GRAPHICS.md` (rendering)
2. `/docs/architecture/ARCHITECTURE.md` (component details)
3. `src/api/drawing.rs` (implementation)

**Web Developers**
1. `/web/README.md` (Web setup)
2. `/docs/guides/GRAPHICS.md` (WebGL section)
3. WEB_MOBILE_CONTROLLER.md (UI)

**Performance Optimizers**
1. `/docs/guides/PERFORMANCE.md` (techniques)
2. `/docs/guides/GRAPHICS.md` (characteristics)
3. `/docs/architecture/ARCHITECTURE.md` (design decisions)

---

## 📋 Quick Reference

### Most Common Questions

**"How do I draw a rectangle?"**
→ `/docs/api/REFERENCE.md` section "Shape Drawing"
```rust
engine.drawing.rectfill(x, y, w, h, color);
```

**"What colors are available?"**
→ `/docs/api/REFERENCE.md` section "Palette Management"
→ `/docs/guides/GRAPHICS.md` section "Color Representation"
→ Palette indices 0-15 (Pyxel default palette)

**"How do particles work?"**
→ `/docs/api/REFERENCE.md` section "Particle System"
→ `/docs/architecture/ARCHITECTURE.md` section "ParticleSystem"

**"How do I check for keyboard input?"**
→ `/docs/api/REFERENCE.md` section "Input API"
```rust
if engine.input.btn(Key::Up) { ... }
if engine.input.btnp(Key::Space) { ... }
```

**"What's the rendering pipeline?"**
→ `/docs/guides/GRAPHICS.md` section "High-Level Graphics Architecture"
→ `/docs/architecture/ARCHITECTURE.md` section "Data Flow"

**"How do I deploy to Web?"**
→ `/web/README.md`
→ `/docs/guides/GETTING_STARTED.md` (TODO)

**"What game engine features exist?"**
→ `/docs/api/REFERENCE.md` section "Core Game Engine"
→ `/docs/architecture/ARCHITECTURE.md` section "GameEngine"

**"How do I add text to my game?"**
→ `/docs/api/REFERENCE.md` section "Text Rendering"
```rust
engine.drawing.print("Hello", x, y, color);
```

---

## 🔄 Documentation Maintenance

### Last Updated (Phase 12)

- ✅ `/docs/api/REFERENCE.md` - Complete (text, particles)
- ✅ `/docs/architecture/ARCHITECTURE.md` - Complete (all components)
- ✅ `/docs/guides/GRAPHICS.md` - Complete (OpenGL/WebGL)
- ⏳ `/docs/guides/GETTING_STARTED.md` - In Progress
- ⏳ `/docs/guides/GAME_DEVELOPMENT.md` - In Progress
- ⏳ `/docs/guides/PERFORMANCE.md` - In Progress
- ⏳ `/docs/examples/*.rs` - In Progress

### Contributing to Docs

1. **Update when adding features**: Modify corresponding API/Architecture doc
2. **Keep examples current**: Test code samples work before committing
3. **Link across docs**: Use relative paths and section headers
4. **Use diagrams**: ASCII diagrams aid understanding
5. **Provide rationale**: Explain design decisions, not just what

---

## 📞 Getting Help

**Not finding what you need?**

1. Check this index (you are here)
2. Search relevant docs with `grep "your query" docs/**/*.md`
3. Check source code: `src/api/*.rs` for implementations
4. Read completion reports: `PHASE_*.md` for feature details

**Have a question about...**

- **API usage**: See `/docs/api/REFERENCE.md`
- **Architecture**: See `/docs/architecture/ARCHITECTURE.md`
- **Graphics**: See `/docs/guides/GRAPHICS.md`
- **Game dev patterns**: See `/docs/guides/GAME_DEVELOPMENT.md` (TODO) or examples
- **Performance**: See `/docs/guides/PERFORMANCE.md` (TODO)

---

## 📂 Full File Tree

```
Nantaraquad/
├── docs/
│   ├── README.md                          # This file
│   ├── api/
│   │   └── REFERENCE.md                  # Complete API documentation
│   ├── architecture/
│   │   └── ARCHITECTURE.md               # System design & components
│   ├── guides/
│   │   ├── GRAPHICS.md                   # Graphics backend guide
│   │   ├── GETTING_STARTED.md            # Quick start (TODO)
│   │   ├── GAME_DEVELOPMENT.md           # Game dev patterns (TODO)
│   │   └── PERFORMANCE.md                # Performance tuning (TODO)
│   └── examples/
│       ├── simple_game.rs                # Minimal example (TODO)
│       ├── particle_effects.rs           # Particle showcase (TODO)
│       ├── text_rendering.rs             # Text API examples (TODO)
│       └── input_handling.rs             # Input patterns (TODO)
├── src/
│   ├── api/
│   │   ├── mod.rs                        # API exports
│   │   ├── game.rs                       # GameEngine
│   │   ├── drawing.rs                    # Drawing API + text
│   │   ├── input.rs                      # Input handling
│   │   ├── particles.rs                  # Particle system
│   │   ├── camera.rs                     # Camera control
│   │   ├── pyxel.rs                      # Pyxel stubs
│   │   └── ...
│   ├── resource/
│   ├── editor/
│   ├── core/
│   ├── audio/
│   └── lib.rs
├── examples/
│   ├── lineboy.rs                        # Drawing game with text
│   ├── cubeboy.rs                        # Platformer with particles
│   └── ...
├── web/
│   ├── index.html                        # Web launcher
│   └── README.md
├── README.md                             # Project overview
├── API_COMPREHENSIVE.md                  # Pyxel API status
├── TEXT_PARTICLE_IMPLEMENTATION.md       # Feature details
└── ... (other completion reports)
```

---

**Start with `/docs/api/REFERENCE.md` for API details or `/docs/architecture/ARCHITECTURE.md` for system overview.**
