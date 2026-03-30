# microface

> ⚠️ **Work in progress** — API is unstable and subject to change.

`no_std` UI toolkit for embedded Rust, built on `embedded-graphics`. Percentage-based layout, compile-time fonts, flex stacks — runs on any `DrawTarget`.

## Quick look

```rust
use microface::{Canvas, include_font, fonts::MicroFont};
use microface::widgets::{VStack, Text, Rect};

const FONT: MicroFont = include_font!("fonts/Inter.ttf", size = 12, bpp = 2);

VStack::within(display.full())
    .padding(8)
    .gap(4)
    .child(Text::new("Hello", &FONT).center())
    .child(Rect::new().color(Rgb565::RED))
    .space()
    .paint(&mut display)?;
```

## Why microface

Most embedded UI in Rust means stitching together separate crates for layout, text, and fonts — or pulling in a heavy C binding. microface is a single `no_std` crate that handles all three.

|  | microface | embedded-layout | embedded-text | kolibri | lvgl (Rust) | u8g2-fonts |
|--|-----------|----------------|--------------|---------|-------------|------------|
| `no_std` | ✅ | ✅ | ✅ | ✅ | ⚠️ C FFI | ✅ |
| Flex stacks (H/V) | ✅ | — | — | ✅ | ✅ | — |
| Percentage layout | ✅ | — | — | — | — | — |
| Compile-time fonts | ✅ | — | — | — | — | — |
| TTF/OTF support | ✅ build-time | — | — | — | ✅ runtime | — |
| Word wrap | ✅ | — | ✅ | ✅ | ✅ | — |
| Cached measurement | ✅ | — | — | — | ✅ | — |
| Widgets + layout | ✅ | layout only | text only | ✅ | ✅ | fonts only |
| Binary size | small | small | small | medium | large | small |
| Runtime deps | none | none | none | alloc | C runtime | none |

**Compile-time fonts** are the big one. `include_font!` rasterizes TTF/OTF during `cargo build` — the binary ships pre-rendered glyph bitmaps with no font parser, no filesystem, no allocator needed at runtime. You pick the exact size and bit depth at build time.

## Features

**Layout** — `Canvas` trait gives any display percentage-based regions: `full()`, `full_row(y0, y1)`, `full_col(x0, x1)`, `region(x, y, w, h)`.

**Stacks** — `HStack` and `VStack` with flex weights, justify (Start / End / Center), cross-axis align (Stretch / Start / End / Center), padding, gap, and spacers.

**Text** — Single-line or multi-line word-wrapped via `.max_width(px)`. Alignment: `.left()`, `.center()`, `.right()`, `.justified()`. Measurement is cached.

**Fonts** — `include_font!` rasterizes TTF/OTF at compile time. Supports 1/2/4/8 bpp, proportional widths, kerning, integer scaling via `.scaled(N)`.

**Color** — Feature-gated: `Rgb565` (default), `Gray8`, `BinaryColor`. Palette constants: `FG`, `BG`, `ACCENT_PR/SC/TR`.

## Feature flags

| Flag | Color type | Target |
|------|-----------|--------|
| `color` | `Rgb565` | LCDs, AMOLEDs |
| `grayscale` | `Gray8` | Grayscale e-ink |
| `bw` | `BinaryColor` | B&W e-ink |
| `std` | — | Headless render, simulator |

## Examples

SDL2 required: `brew install sdl2` or `apt install libsdl2-dev`.

```bash
export LIBRARY_PATH="$(brew --prefix sdl2)/lib"

cargo run --example test_stacks --features std         # stack layout
cargo run --example test_text_element --features std   # text + cache bench
cargo run --example test_text_stacks --features std    # text positioning
cargo run --example test_render_display --features std # live clock
cargo run --example test_render --features std         # headless BMP
```

## Project layout

```
src/
  widgets/layout/canvas.rs     Canvas trait
  widgets/layout/stack/        HStack, VStack
  widgets/primitives/          Rect
  widgets/text/                Label, Text
  fonts/                       MicroFont, MicroFontStyle
  color.rs                     Color types + constants
  element.rs                   Element enum
  render/                      [std] BitmapTarget
microface-macros/              include_font! proc macro
fonts/                         Bundled TTF files
examples/
```

## License

MIT OR Apache-2.0
