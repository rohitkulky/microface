Embedded-Graphics Ecosystem — Useful Libraries for ESP32 Display Development
============================================================================

> **Generated:** 2026-03-11
> **Source:** All 238 reverse dependencies of [`embedded-graphics`](https://crates.io/crates/embedded-graphics) on crates.io
> **Context:** ESP32-S3 + SH8601 AMOLED, using `embedded-graphics` `DrawTarget`

This document catalogues every useful library from the `embedded-graphics` reverse-dependency tree that provides **drawing utilities, fonts, UI frameworks, image decoders, or ESP32-relevant wrappers**. Board-specific BSPs (nucleo, feather, pybadge, etc.) and niche/unmaintained crates have been excluded.

* * *

Currently Used in This Project
------------------------------

| Crate | Version | Role |
| --- | --- | --- |
| `embedded-graphics` | 0.8 | Core drawing primitives |
| `embedded-graphics-core` | 0.4.0 | Core traits (`DrawTarget`, `Pixel`, etc.) |
| `profont` | 0.7 | ProFont bitmap font family |
| `buoyant` | 0.6.1 | Flexbox-like layout engine |

* * *

🎨 UI Frameworks & Layout
-------------------------

| Crate | Description | Notes |
| --- | --- | --- |
| [`embedded-layout`](https://crates.io/crates/embedded-layout) | Layout/alignment helpers (center, align, distribute views) | Lightweight positioning primitives |
| [`embedded-menu`](https://crates.io/crates/embedded-menu) | Menu system for embedded displays | Scrollable menus with selection |
| [`embedded-text`](https://crates.io/crates/embedded-text) | Rich text rendering (word wrap, alignment, styling) | Very popular, great for multi-line text |
| [`kolibri-embedded-gui`](https://crates.io/crates/kolibri-embedded-gui) | Full GUI widget toolkit (buttons, sliders, checkboxes) | Reactive widget system |
| [`embedded-ui`](https://crates.io/crates/embedded-ui) | UI component library | Widgets and layout |
| [simple-layout](https://crates.io/crates/simple-layout) | Simple layout primitives | Minimal layout helpers |
| [`lvgl`](https://crates.io/crates/lvgl) | Rust bindings for LVGL graphics library | Full-featured GUI framework (heavy) |
| [`embedded-canvas`](https://crates.io/crates/embedded-canvas) | Off-screen canvas/layer for compositing | Draw to buffer, then blit |
| [`embedded-list`](https://crates.io/crates/embedded-list) | Scrollable list widget | List rendering |
| [`iris-ui`](https://crates.io/crates/iris-ui) | UI framework for embedded displays | Newer UI toolkit |

* * *

🔤 Fonts
--------

| Crate | Description | Notes |
| --- | --- | --- |
| [`u8g2-fonts`](https://crates.io/crates/u8g2-fonts) | Huge collection of fonts from u8g2 library | 100s of fonts, very popular |
| [`bitmap-font`](https://crates.io/crates/bitmap-font) | Bitmap font rendering | Multiple font sizes |
| [`embedded-graphics-unicodefonts`](https://crates.io/crates/embedded-graphics-unicodefonts) | Unicode/CJK font support | For international text |
| [`ibm437`](https://crates.io/crates/ibm437) | IBM CP437 character set font | Retro/terminal style |
| [`embedded-vintage-fonts`](https://crates.io/crates/embedded-vintage-fonts) | Retro/vintage bitmap fonts | Classic computer fonts |
| [`eg-seven-segment`](https://crates.io/crates/eg-seven-segment) | Seven-segment display style font | Great for clocks/numbers |
| [`font_7seg`](https://crates.io/crates/font_7seg) | Another seven-segment font | Alternative 7-seg |
| [`multi-mono-font`](https://crates.io/crates/multi-mono-font) | Multiple monospace fonts | Mono font collection |
| [`mplusfonts`](https://crates.io/crates/mplusfonts) | M+ font family for embedded | Japanese-friendly fonts |
| [`embedded-mogeefont`](https://crates.io/crates/embedded-mogeefont) | Tiny pixel font | Very small displays |
| [`embedded-bitmap-fonts`](https://crates.io/crates/embedded-bitmap-fonts) | Bitmap font collection | Various bitmap fonts |
| [`embedded-ttf`](https://crates.io/crates/embedded-ttf) | TrueType font rendering | Vector fonts on embedded! |
| [`minitype`](https://crates.io/crates/minitype) | Minimal TrueType renderer | Lightweight TTF |

* * *

🖼️ Image Decoding
------------------

| Crate | Description | Notes |
| --- | --- | --- |
| [`tinybmp`](https://crates.io/crates/tinybmp) | BMP image decoder for embedded-graphics | Official, very popular |
| [`tinytga`](https://crates.io/crates/tinytga) | TGA image decoder for embedded-graphics | Official, popular |
| [`tinygif`](https://crates.io/crates/tinygif) | GIF decoder for embedded-graphics | Animated GIF support! |
| [`tinyqoi`](https://crates.io/crates/tinyqoi) | QOI image decoder | Fast, simple image format |
| [`embedded-iconoir`](https://crates.io/crates/embedded-iconoir) | Iconoir icon set for embedded-graphics | 1000+ SVG icons as bitmaps |
| [`embedded-icon`](https://crates.io/crates/embedded-icon) | Icon rendering | Icon support |

* * *

📊 Drawing Utilities & Extensions
---------------------------------

| Crate | Description | Notes |
| --- | --- | --- |
| [`embedded-graphics-framebuf`](https://crates.io/crates/embedded-graphics-framebuf) | Framebuffer abstraction for embedded-graphics | Off-screen rendering, compositing |
| [`gfx-xtra`](https://crates.io/crates/gfx-xtra) | Extra drawing primitives (rounded rects, etc.) | Extended shape drawing |
| [`embedded-plots`](https://crates.io/crates/embedded-plots) | Plot/chart rendering | Line charts, bar charts |
| [`embedded-graphics-sparklines`](https://crates.io/crates/embedded-graphics-sparklines) | Sparkline chart rendering | Tiny inline charts |
| [`embedded-charts`](https://crates.io/crates/embedded-charts) | Chart/graph rendering | Data visualization |
| [`embedded-counters`](https://crates.io/crates/embedded-counters) | Animated counter displays | Number animations |
| [`embedded-fps`](https://crates.io/crates/embedded-fps) | FPS counter overlay | Performance monitoring |
| [`embedded-sprites`](https://crates.io/crates/embedded-sprites) | Sprite rendering system | Game-like sprite support |
| [`embedded-graphics-transform`](https://crates.io/crates/embedded-graphics-transform) | Geometric transforms (rotate, scale) | Transform drawing operations |
| [`embedded-graphics-colorcast`](https://crates.io/crates/embedded-graphics-colorcast) | Color space conversion | Convert between color types |
| [`embedded-graphics-coordinate-transform`](https://crates.io/crates/embedded-graphics-coordinate-transform) | Coordinate system transforms | Rotation/mirroring |
| [`embedded-3dgfx`](https://crates.io/crates/embedded-3dgfx) | Basic 3D graphics | 3D rendering on embedded |
| [`blitty`](https://crates.io/crates/blitty) | Fast blitting/sprite operations | Efficient pixel copying |
| [`embedded-term`](https://crates.io/crates/embedded-term) | Terminal emulator widget | Terminal-style text display |
| [`mcumeter`](https://crates.io/crates/mcumeter) | Gauge/meter widget | Analog meter rendering |

* * *

🔌 ESP32-Specific / Relevant
----------------------------

| Crate | Description | Notes |
| --- | --- | --- |
| [`esp-display-interface-spi-dma`](https://crates.io/crates/esp-display-interface-spi-dma) | ESP32 SPI DMA display interface | DMA-accelerated display transfers |
| [`mipidsi`](https://crates.io/crates/mipidsi) | Generic MIPI DSI display driver | Works with many TFT/OLED panels |
| [`sh8601-rs`](https://crates.io/crates/sh8601-rs) | SH8601 Rust driver | Our exact display chip! |
| [`esp-hub75`](https://crates.io/crates/esp-hub75) | ESP32 HUB75 LED matrix driver | If using LED matrices |
| [`ws2812-esp32-rmt-driver`](https://crates.io/crates/ws2812-esp32-rmt-driver) | WS2812 LED strip via ESP32 RMT | Addressable LEDs |
| [`m5dial-bsp`](https://crates.io/crates/m5dial-bsp) | M5Stack Dial BSP | If using M5Dial hardware |
| [`m5cardputer`](https://crates.io/crates/m5cardputer) | M5Stack Cardputer BSP | If using Cardputer |
| [`espforge_devices`](https://crates.io/crates/espforge_devices) | ESP device abstractions | ESP device support |

* * *

🎮 Simulator / Testing
----------------------

| Crate | Description | Notes |
| --- | --- | --- |
| [`embedded-graphics-simulator`](https://crates.io/crates/embedded-graphics-simulator) | Desktop simulator for embedded-graphics | Test UI without hardware |
| [`embedded-graphics-web-simulator`](https://crates.io/crates/embedded-graphics-web-simulator) | Web-based simulator | Test in browser |

* * *

🏆 Top Recommendations
----------------------

Given our setup (ESP32-S3 + SH8601 AMOLED + `embedded-graphics` + `buoyant` + `profont`), the highest-value additions:

| Priority | Crate | Why |
| --- | --- | --- |
| ⭐⭐⭐ | `u8g2-fonts` | Massive font library — way more choices than profont alone |
| ⭐⭐⭐ | `embedded-text` | Rich text with word wrapping — essential for any real UI |
| ⭐⭐⭐ | `tinybmp` / `tinytga` | Display images/icons on the AMOLED |
| ⭐⭐⭐ | `embedded-iconoir` | Beautiful icon set, ready to use |
| ⭐⭐ | `embedded-graphics-framebuf` | Double-buffering / off-screen compositing |
| ⭐⭐ | `embedded-layout` | Complements buoyant for positioning |
| ⭐⭐ | `embedded-graphics-simulator` | Iterate on UI without flashing hardware |
| ⭐ | `embedded-ttf` | Scalable fonts if bitmap fonts aren't enough |
| ⭐ | `kolibri-embedded-gui` | Full widget toolkit if buoyant isn't sufficient |
| ⭐ | `embedded-graphics-transform` | Rotation/scaling for animations |
