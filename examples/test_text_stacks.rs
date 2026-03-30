//! Text positioning demo: labels at 5 screen positions using stacks.
//!
//! Shows how to combine `display.region()`, `VStack`, and `TextAlign`
//! to place text at top-center, left-center, center, right-center,
//! and bottom-center of the screen.
//!
//! Run: `cargo run --example test_text_stacks --features std`

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use microface::element::Element;
use microface::fonts::MicroFont;
use microface::widgets::{Label, TextAlign, VStack};
use microface::{Canvas, include_font};

use std::time::Duration;

const FONT: MicroFont = include_font!("fonts/Inter.ttf", size = 12, bpp = 2);

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));

    // ── Top-Center ─────────────────────────────────────────────
    VStack::within(display.full())
        .child(Label::new("Top-Center", &FONT).align(TextAlign::Center))
        .spacer(9)
        .paint(&mut display)
        .unwrap();

    // ── Bottom-Center ──────────────────────────────────────────
    VStack::within(display.full())
        .spacer(9)
        .child(Label::new("Bottom-Center", &FONT).align(TextAlign::Center))
        .paint(&mut display)
        .unwrap();

    // ── Left-Center ────────────────────────────────────────────
    VStack::within(display.full())
        .spacer(1)
        .child(Label::new("Left-Center", &FONT))
        .spacer(1)
        .paint(&mut display)
        .unwrap();

    // ── Right-Center ───────────────────────────────────────────
    VStack::within(display.full())
        .spacer(1)
        .child(Label::new("Right-Center", &FONT).align(TextAlign::Right))
        .spacer(1)
        .paint(&mut display)
        .unwrap();

    // ── Dead Center ────────────────────────────────────────────
    VStack::within(display.full())
        .spacer(1)
        .child(Label::new("Center", &FONT).align(TextAlign::Center))
        .spacer(1)
        .paint(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("microface — text positions", &output_settings);
    window.update(&mut display);

    loop {
        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
