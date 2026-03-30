//! Stack layout demo: flex weights, justify, gap, padding, and nesting.
//!
//! Run: `cargo run --example test_stacks --features std`

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use microface::Canvas;
use microface::color::{ACCENT_PR, ACCENT_SC, ACCENT_TR, FG};
use microface::element::Element;
use microface::widgets::{HStack, Justify, Rect, VStack};

use std::time::Duration;

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));

    // Full-screen VStack with 3 rows
    VStack::within(display.region(0, 0, 100, 100))
        .gap(2)
        .padding(4)
        // Row 1: HStack with flex weights + gap (nested)
        .child(Element::HStack(
            HStack::new()
                .gap(2)
                .child(Element::Rect(Rect::new().color(ACCENT_PR)))
                .child_flex(Element::Rect(Rect::new().color(FG)), 2)
                .child_flex(Element::Rect(Rect::new().color(ACCENT_TR)), 3)
                .child(Element::Rect(Rect::new().color(ACCENT_SC))),
        ))
        // Row 2: HStack with padding + gap (nested)
        .child(Element::HStack(
            HStack::new()
                .padding(8)
                .gap(4)
                .child(Element::Rect(Rect::new().color(ACCENT_TR)))
                .child(Element::Rect(Rect::new().color(ACCENT_PR)))
                .child(Element::Rect(Rect::new().color(ACCENT_SC))),
        ))
        // Row 3: HStack with Justify::Center (nested)
        .child(Element::HStack(
            HStack::new()
                .justify(Justify::Center)
                .gap(4)
                .child(Element::Rect(Rect::new().color(ACCENT_PR)))
                .child(Element::Rect(Rect::new().color(FG)))
                .child(Element::Rect(Rect::new().color(ACCENT_SC))),
        ))
        // Row 4: plain rect
        .child(Element::Rect(Rect::new().color(ACCENT_TR)))
        .paint(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("microface — nested stacks", &output_settings);
    window.update(&mut display);

    loop {
        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
