# Microface — Architecture TODOs

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

## 2. LayoutStack Trait

Wrap `HStack` and (future) `VStack` in a shared `LayoutStack` trait:

```rust
pub trait LayoutStack {
    fn child(self, element: impl Component) -> Self;
    fn child_flex(self, element: impl Component, flex: u32) -> Self;
    fn spacing(self, px: u32) -> Self;
    fn padding(self, px: u32) -> Self;
}
```

**Common functions to define:**
- `child()` / `child_flex()` — add children with optional flex weight
- `spacing()` — gap between children
- `padding()` — inner padding
- `alignment()` — cross-axis alignment (start, center, end)
- `paint()` — shared layout algorithm (only axis differs: H vs V)

**Implementation idea:** Single `Stack` struct with an `Axis` enum (`Horizontal` / `Vertical`),
so `HStack` and `VStack` are just constructors:
```rust
pub fn hstack() -> Stack { Stack::new(Axis::Horizontal) }
pub fn vstack() -> Stack { Stack::new(Axis::Vertical) }
```

---

## 3. Elements to Implement

### Primitives
- [x] **Rect** — filled/stroked rectangle with color
- [ ] **RoundedRect** — rectangle with corner radius
- [ ] **Circle** — filled/stroked circle
- [ ] **Line** — line between two points
- [ ] **Spacer** — invisible flex element for pushing content apart

### Text
- [x] **Label** — single-line text (currently uses MonoFont, should use MicroFont)
- [ ] **TextBlock** — multi-line wrapped text via MicroFontStyle + embedded-text
- [ ] **Badge** — text with background pill/rounded rect

### Layout
- [x] **HStack** — horizontal flex layout
- [ ] **VStack** — vertical flex layout
- [ ] **ZStack** — overlay/layered layout
- [ ] **Grid** — row×column grid layout
- [ ] **ScrollView** — vertical scrolling container (for e-ink/LCD)

### Data Display
- [ ] **ProgressBar** — horizontal/vertical fill bar
- [ ] **Gauge** — circular/arc gauge (for dashboards)
- [ ] **Icon** — bitmap icon from embedded-iconoir or tinybmp
- [ ] **Image** — BMP/TGA image display

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

Set up `embedded-graphics-simulator` for desktop rendering — iterate on UI
without flashing hardware.

```toml
[dev-dependencies]
embedded-graphics-simulator = "0.6"
```

**Goals:**
- [ ] Add simulator example that opens a window and renders the current view
- [ ] Support hot-reload workflow: edit code → `cargo run` → see result instantly
- [ ] Render multiple views side-by-side for comparison
- [ ] Capture view transitions (view A → view B) in the simulator window

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
