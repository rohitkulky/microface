# microface

`no_std` UI library for embedded Rust. Widgets, layout, compile-time fonts, percentage-based sizing — all on top of `embedded-graphics`. Works on any `DrawTarget`: AMOLED, LCD, e-ink.

## What's in the box

- **Screen** — percentage-based sizing so layouts port across resolutions without pixel math
- **Widgets** — `Rect`, `Label`, `HStack` with flex-weighted children
- **Element** — unified enum (`Rect | Label | Empty`) for composing UI trees
- **Compile-time fonts** — `include_font!` rasterizes TTF/OTF at build time, no filesystem or runtime parsing
- **Color modes** — `Rgb565`, `Gray8`, `BinaryColor` via feature flags. Comes with `FG`, `BG`, `ACCENT_*` constants
- **Layout helpers** — `hz_stack()`, `vt_stack()`, `gap()`, `even()` wrapping `embedded-layout`

## Usage

```toml
[dependencies]
microface = { path = "." }
embedded-graphics = "0.8"
```

### Screen & layout

```rust
use microface::Screen;

let screen = Screen::new(368, 448);

let header_h = screen.hp(10);                // 10% of height
let content  = screen.region(5, 10, 90, 75); // x=5%, y=10%, w=90%, h=75%
```

### Widgets

```rust
use microface::widgets::{HStack, primitives::Rect, text::Label};
use microface::element::Element;

HStack::new()
    .child(Element::Rect(Rect::new()))                     // flex 1
    .child_flex(Element::Label(Label::new("Hi", &font)), 2) // flex 2
    .child(Element::Rect(Rect::new()))                     // flex 1
    .paint(screen.bounds(), &mut display)?;
```

### Compile-time fonts

```rust
use microface::{include_font, fonts::{MicroFont, MicroFontStyle}};

const DIN: MicroFont = include_font!("fonts/dinroundpro.otf", size = 16, bpp = 4);

let style = MicroFontStyle::new(&DIN, Gray4::WHITE);
// works with embedded-text TextBox for word wrap + alignment
```

`bpp`: 1 (e-ink), 2, 4 (recommended), 8. Integer scaling via `.scaled(N)`.

## Feature Flags

| Feature | Color type | Use case |
|---------|-----------|----------|
| `color` (default) | `Rgb565` | Color LCDs, AMOLEDs |
| `grayscale` | `Gray8` | Grayscale e-ink |
| `bw` | `BinaryColor` | B&W e-ink |
| `std` | — | Enables headless `render` module |

## Examples

Requires SDL2 (`brew install sdl2` / `apt install libsdl2-dev`).

```bash
# Live simulator window
LIBRARY_PATH="$(brew --prefix sdl2)/lib" cargo run --example test_render_display

# Headless BMP export
cargo run --example test_render --features std
```

## Structure

```
microface/
├── microface-macros/          # proc macro: include_font!
├── src/
│   ├── basis/foundation/      # Screen, layout helpers (hz_stack, vt_stack, gap, even)
│   ├── widgets/               # HStack, Rect, Label
│   ├── fonts/                 # MicroFont, MicroFontStyle, TextRenderer impl
│   ├── color.rs               # feature-gated color types + constants
│   ├── element.rs             # Element enum
│   └── render/                # [std] BitmapTarget for headless rendering
├── fonts/                     # bundled TTF/OTF files
└── examples/
```

## License

Apache-2.0
