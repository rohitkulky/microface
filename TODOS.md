# Microface — Architecture TODOs

## 0. What We Have (Completed)

### Core Infrastructure
- [x] **`no_std` library** — `#![no_std]` with `extern crate alloc`, works on any embedded target
- [x] **Feature-gated color system** — `color.rs` provides `GraphicsColorMode` type alias + `FG`, `BG`, `ACCENT_PR/SC/TR` constants for `color` (Rgb565), `grayscale` (Gray8), `bw` (BinaryColor)
- [x] **`Canvas` trait** — percentage-based sizing via blanket impl on `OriginDimensions` in `widgets/layout/canvas.rs`: `wp()`, `hp()`, `w()`, `h()`, `region()`, `region_clamped()`, `canvas_bounds()` — the display itself is the canvas, no separate object needed
- [x] **Generic `Stack` layout** — `Stack<D: StackDirection>` in `widgets/layout/stack/` with flex-weighted children. `HStack` and `VStack` are type aliases for `Stack<Horizontal>` and `Stack<Vertical>`
- [x] **Layout helpers** — `hz_stack()`, `vt_stack()`, `gap()`, `even()`, `tight()` in `basis/foundation/layout.rs` wrapping `embedded-layout`
- [x] **`Element` enum** — dispatch enum (`Empty | Rect | Label`) in `element.rs` for composing UI trees
- [x] **Rustdoc cleanup** — all modules, structs, traits, and public methods have specific, neutral doc comments; zero `cargo doc` warnings

### Fonts
- [x] **`include_font!` proc macro** — compile-time TTF/OTF rasterization in `microface-macros/` via `fontdue`. Supports bpp 1/2/4/8, proportional widths, tight per-glyph bounding boxes, kerning pairs
- [x] **`MicroFont`** — bitmap font struct with `advance_width()`, `glyph_bbox()`, `kern()`, `read_alpha()`, `read_alpha_index()`
- [x] **`MicroFontStyle`** — implements `TextRenderer` + `CharacterStyle` traits. 16-entry LUT alpha blending, integer scaling via `.scaled(N)`, optional background color. Works with `embedded-text::TextBox` for word wrap + alignment

### Rendering
- [x] **`BitmapTarget`** — headless in-memory framebuffer behind `std` feature in `render/bitmap.rs`. Implements `DrawTarget`, exports 24-bit BMP files
- [x] **Simulator example** — `examples/test_render_display.rs` opens an SDL2 window via `embedded-graphics-simulator`, renders live clock with multiple font sizes and bpp levels
- [x] **Headless BMP example** — `examples/test_render.rs` renders to BMP file, gated behind `--features std`
- [x] **Stack layout example** — `examples/test_stacks.rs` demonstrates HStack/VStack with flex-weighted children in a simulator window

### Build
- [x] **Release profile** — `opt-level = "z"`, LTO, single codegen unit, symbol stripping, `panic = "abort"` for minimal binary size
- [x] **Workspace** — `microface` + `microface-macros` in a Cargo workspace

---

## 1. Component Trait

Define a `Component` trait that all UI elements must implement for consistency:

```rust
pub trait Component {
    /// Paint this component into the given bounds.
    fn paint<D: DrawTarget>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>;

    /// Minimum size this component needs (for layout negotiation).
    fn min_size(&self) -> Size;

    /// Preferred/natural size (may differ from min).
    fn preferred_size(&self) -> Size;
}
```

**Why:** Currently `Rect`, `Label`, and `HStack` all have `paint()` but with no shared trait.
The `Element` enum dispatches manually. A trait enables:
- Consistent API across all widgets
- `dyn Component` for heterogeneous collections (no enum needed)
- Layout engines can query `min_size()` / `preferred_size()` generically

---

## 2. Stack Layout Features

### Main-axis distribution — `.justify(Justify::*)`
- [x] `Start` — pack children at the start (default)
- [x] `End` — pack children at the end
- [x] `Center` — center the group along the main axis
- [ ] `SpaceBetween` — requires Component trait with `measure()` (§1)
- [ ] `SpaceAround` — requires Component trait with `measure()` (§1)
- [ ] `SpaceEvenly` — requires Component trait with `measure()` (§1)

### Cross-axis alignment — `.align(Align::*)`
- [x] `Stretch` — fill the full cross-axis extent (default)
- [ ] `Start` — enum defined, effective once Component trait provides `measure()`
- [ ] `End` — enum defined, effective once Component trait provides `measure()`
- [ ] `Center` — enum defined, effective once Component trait provides `measure()`

### Spacing
- [x] `.padding(px)` — inset from bounds on all sides before layout
- [x] `.gap(px)` — fixed pixel gap between children

### API

```rust
HStack::within(bounds)
    .justify(Justify::Center)
    .align(Align::Stretch)
    .padding(8)
    .gap(4)
    .child(Element::Rect(Rect::new()))
    .child_flex(Element::Rect(Rect::new()), 2)
    .paint(&mut display)?;
```

---

## 3. Elements to Implement

### Primitives
- [x] **Rect** — filled/stroked rectangle with color (`.color()`, `.stroke()`)
- [ ] **RoundedRect** — rectangle with corner radius
- [ ] **Circle** — filled/stroked circle
- [ ] **Line** — line between two points
- [ ] **Spacer** — invisible flex element for pushing content apart

### Text
- [x] **Label** — single-line text (currently uses `MonoFont`; see §7 for MicroFont migration)
- [ ] **TextBlock** — multi-line wrapped text via `MicroFontStyle` + `embedded-text::TextBox`
- [ ] **Badge** — text with background pill/rounded rect

### Layout
- [x] **HStack** — horizontal flex layout (`.child()`, `.child_flex()`)
- [x] **VStack** — vertical flex layout (same generic `Stack` with `Vertical` direction)
- [ ] **ZStack** — overlay/layered layout
- [ ] **Grid** — row×column grid layout
- [ ] **ScrollView** — vertical scrolling container (for e-ink/LCD)

### Data Display
- [ ] **ProgressBar** — horizontal/vertical fill bar
- [ ] **Gauge** — circular/arc gauge (for dashboards)
- [ ] **Icon** — bitmap icon from `embedded-iconoir` or `tinybmp`
- [ ] **Image** — BMP/TGA image display via `tinybmp`/`tinytga`

### Interactive (future)
- [ ] **Button** — pressable area with label + callback
- [ ] **Toggle** — on/off switch
- [ ] **Slider** — value selector

### Features per element:
Each element should support (via `Component` trait + builder pattern):
- `.color()` — foreground color
- `.background()` — background color
- `.padding()` — inner spacing
- `.border()` — border width + color
- `.corner_radius()` — rounded corners (where applicable)
- `.opacity()` — alpha transparency (0.0–1.0)
- `.min_size()` / `.max_size()` — size constraints

---

## 4. Display Simulator

- [x] Add simulator example that opens a window and renders the current view (`test_render_display.rs`)
- [ ] Support hot-reload workflow: edit code → `cargo run` → see result instantly
- [ ] Render multiple views side-by-side for comparison
- [ ] Capture view transitions (view A → view B) in the simulator window

```toml
[dev-dependencies]
embedded-graphics-simulator = "0.8"  # ✅ already in Cargo.toml
```

---

## 5. View Transitions & Animation

Use the `keyframes` crate for smooth, eased transitions between views.

```toml
[dependencies]
keyframes = "1"
```

**Architecture:**
- [ ] Define a `View` trait — a full-screen composable UI state
- [ ] `ViewManager` holds the current view and handles transitions
- [ ] Transitions: slide, fade, crossfade, push (configurable per navigation)
- [ ] `keyframes` provides easing functions (ease-in-out, spring, linear, etc.)
- [ ] Each frame: interpolate between old view and new view by `t` (0.0→1.0)
- [ ] On embedded: render at display refresh rate; on simulator: 60fps

**Example flow:**
```rust
let mut vm = ViewManager::new(HomeView::new());
vm.transition_to(SettingsView::new(), Transition::SlideLeft, Duration::from_millis(300));

// In render loop:
vm.tick(elapsed);
vm.paint(&mut display)?;
```

**Easing via keyframes:**
```rust
use keyframes::{Keyframe, AnimationSequence};
let anim = AnimationSequence::from(vec![
    Keyframe::new(0.0, 0.0, keyframes::functions::EaseInOut),
    Keyframe::new(1.0, 0.3, keyframes::functions::EaseInOut),
]);
let t = anim.value_at(elapsed_secs); // smooth 0.0 → 1.0
```

---

## 6. Known Bugs

- [x] **`examples/test_render.rs`** — ~~referenced `DIN_4` which didn't exist~~ → fixed to `INTER_4`, gated behind `required-features = ["std"]`

---

## 7. Architecture Improvements

- [ ] **Label → MicroFont migration** — `Label` currently takes `&MonoFont`. Add a variant or new widget that uses `MicroFontStyle` for anti-aliased, proportional text with the compile-time font system
- [ ] **Replace `Element` enum with `Component` trait** — the manual dispatch in `Element::paint()` doesn't scale. Once the `Component` trait (§1) is implemented, `Element` can be replaced with `Box<dyn Component>` or trait-based generics
- [ ] **Populate `basis/components/`** — the directory exists but is empty. Intended for reusable composed components (e.g., `StatusBar`, `ListItem`, `Card`) built from primitives + layout
- [x] **Stack gap/padding** — `.gap(px)` and `.padding(px)` on `Stack` (applies to both HStack and VStack)
- [ ] **Stroke width on Rect** — `Rect::stroke()` hardcodes `stroke_width(1)`. Add `.stroke_width(px)` builder method
- [ ] **Label vertical alignment** — `Label::paint()` draws at `bounds.top_left` with no vertical centering. Add baseline/alignment options
- [ ] **Error handling in `Element`** — `Element::paint()` requires `D::Error` to match across all variants. Consider a unified error type or `Component` trait with associated error type
