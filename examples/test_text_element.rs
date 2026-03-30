//! Text element demo: single-line and multi-line text with cache verification.
//!
//! Run: `LIBRARY_PATH=/opt/homebrew/opt/sdl2/lib cargo run --example test_text_element --features std`

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use microface::fonts::MicroFont;
use microface::widgets::{Text, VStack};
use microface::{Canvas, include_font};

use std::time::{Duration, Instant};

const FONT: MicroFont = include_font!("fonts/Inter.ttf", size = 12, bpp = 2);

const LARGE_TEXT: &str = "\
Microface is a simplified, hardware-agnostic interface for embedded graphics in Rust. \
Built on top of embedded-graphics and embedded-layout, microface provides a friendlier \
API for drawing and laying out UI elements on any display. Design principles include \
no hardcoded pixels using the Canvas trait for percentage-based sizing, hardware-agnostic \
support for any DrawTarget implementation, and full configurability to swap display \
drivers and port to any hardware. The Text widget supports single-line and multi-line \
word-wrapped rendering using embedded-text's TextBox for line breaking and wrapping. \
It implements a pretext-inspired cached measurement optimization where the size is \
computed once and reused everywhere. This is the same pattern that pretext uses: \
prepare does the expensive work once, layout is pure arithmetic on cached data. \
The Cell Option Size cache means the first call to measure does the work, and every \
subsequent call from Stack paint_into, from paint alignment, etc is a free get. \
Fonts are rasterized at compile time via the include_font proc macro. Works with any \
display: AMOLED, LCD, e-ink in color, grayscale, or black and white. Rendering uses \
a 16-entry LUT built once per draw call, fill_contiguous for single-call-per-glyph \
rendering, tight per-glyph bounding boxes, and optional kerning.";

fn main() {
    // ── Cache verification: single-line ────────────────────────
    println!("┌─ Single-Line Cache ──────────────────────────────┐");
    let t1 = Text::new("Short single-line text", &FONT).center();
    let t0 = Instant::now();
    let s1 = t1.measure();
    let first = t0.elapsed().as_micros();
    let t0 = Instant::now();
    let _ = t1.measure();
    let second = t0.elapsed().as_micros();
    let t0 = Instant::now();
    for _ in 0..10_000 { let _ = t1.measure(); }
    let batch = t0.elapsed().as_micros();
    println!("│ size: {}×{}px", s1.width, s1.height);
    println!("│ 1st call (compute):    {:>6} µs", first);
    println!("│ 2nd call (cached):     {:>6} µs", second);
    println!("│ 10k calls (cached):    {:>6} µs ({:.3} µs/call)", batch, batch as f64 / 10_000.0);
    println!("│ compute_count: {}", t1.compute_count());
    println!("└──────────────────────────────────────────────────┘");
    assert_eq!(t1.compute_count(), 1);

    // ── Cache verification: multi-line large text ──────────────
    println!();
    println!("┌─ Multi-Line Cache ({}B text, 600px wrap) ───────┐", LARGE_TEXT.len());
    let t2 = Text::new(LARGE_TEXT, &FONT).max_width(600).left();
    let t0 = Instant::now();
    let s2 = t2.measure();
    let first = t0.elapsed().as_micros();
    let t0 = Instant::now();
    let _ = t2.measure();
    let second = t0.elapsed().as_micros();
    let t0 = Instant::now();
    for _ in 0..10_000 { let _ = t2.measure(); }
    let batch = t0.elapsed().as_micros();
    println!("│ size: {}×{}px ({} lines)", s2.width, s2.height, s2.height / 15);
    println!("│ 1st call (compute):    {:>6} µs", first);
    println!("│ 2nd call (cached):     {:>6} µs", second);
    println!("│ 10k calls (cached):    {:>6} µs ({:.3} µs/call)", batch, batch as f64 / 10_000.0);
    println!("│ compute_count: {}", t2.compute_count());
    println!("└──────────────────────────────────────────────────┘");
    assert_eq!(t2.compute_count(), 1);

    // ── Compare: Label (no cache) vs Text (cached) ─────────────
    println!();
    println!("┌─ Label vs Text: 1000× measure() ─────────────────┐");
    use microface::widgets::Label;
    let label = Label::new("Same text for comparison", &FONT);
    let text = Text::new("Same text for comparison", &FONT);
    let t0 = Instant::now();
    for _ in 0..1000 { let _ = label.measure(); }
    let label_us = t0.elapsed().as_micros();
    let t0 = Instant::now();
    for _ in 0..1000 { let _ = text.measure(); }
    let text_us = t0.elapsed().as_micros();
    println!("│ Label (no cache): {:>6} µs", label_us);
    println!("│ Text  (cached):   {:>6} µs", text_us);
    if label_us > 0 {
        println!("│ Speedup:          {:>5.1}×", label_us as f64 / text_us.max(1) as f64);
    }
    println!("└──────────────────────────────────────────────────┘");

    println!("\n✅ All cache checks passed!\n");

    // ── Visual demo on large display ───────────────────────────
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(640, 480));

    // Title
    VStack::within(display.full_row(0, 20))
        .child(Text::new("microface — Text Element Demo", &FONT).center())
        .space()
        .paint(&mut display)
        .unwrap();

    // Large multi-line text block
    VStack::within(display.full_row(30, 450))
        .padding(20)
        .child(
            Text::new(LARGE_TEXT, &FONT)
                .max_width(600)
                .left(),
        )
        .space()
        .paint(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("microface — Text element (640×480)", &output_settings);
    window.update(&mut display);

    loop {
        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
