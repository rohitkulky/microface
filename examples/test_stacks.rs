//! Stack layout demo: justify, align, flex weights, gap, padding, and nesting.
//!
//! Run: `cargo run --example test_stacks --features std`

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use microface::Canvas;
use microface::color::{ACCENT_PR, ACCENT_SC, ACCENT_TR, BG, FG};
use microface::widgets::{Align, HStack, Justify, Label, Rect, VStack};
use microface::{fonts::MicroFont, include_font};

use std::time::Duration;

const FONT: MicroFont = include_font!("fonts/Inter.ttf", size = 12, bpp = 2);

/// Dim gray for reference rects — shows container extent.
const DIM: Rgb565 = Rgb565::new(4, 8, 4);

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(720, 480));

    // ── Background ─────────────────────────────────────────────
    VStack::within(display.full())
        .child(Rect::new().color(BG))
        .paint(&mut display)
        .unwrap();

    // ── Main layout: two columns ───────────────────────────────
    HStack::within(display.full())
        .gap(16)
        // .padding(10)
        // ════════════════════════════════════════════════════════
        // LEFT COLUMN: Justify (main-axis positioning)
        // Labels have intrinsic width → leftover space is visible.
        // ════════════════════════════════════════════════════════
        .child_flex(
            VStack::new()
                // .gap(4)
                .child(Label::new("JUSTIFY (main axis)", &FONT).color(ACCENT_SC))
                // ── Justify::Start ─────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Justify::Start — packed left", &FONT).color(ACCENT_TR))
                        .child_flex(
                            HStack::new()
                                .justify(Justify::Start)
                                // .gap(8)
                                .child(Label::new("AAA", &FONT).color(ACCENT_PR))
                                .child(Label::new("BBB", &FONT).color(FG))
                                .child(Label::new("CCC", &FONT).color(ACCENT_SC)),
                            2,
                        ),
                )
                // ── Justify::Center ────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Justify::Center — centered", &FONT).color(ACCENT_TR))
                        .child_flex(
                            HStack::new()
                                .justify(Justify::Center)
                                // .gap(8)
                                .child(Label::new("AAA", &FONT).color(ACCENT_PR))
                                .child(Label::new("BBB", &FONT).color(FG))
                                .child(Label::new("CCC", &FONT).color(ACCENT_SC)),
                            2,
                        ),
                )
                // ── Justify::End ───────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Justify::End — packed right", &FONT).color(ACCENT_TR))
                        .child_flex(
                            HStack::new()
                                .justify(Justify::End)
                                // .gap(8)
                                .child(Label::new("AAA", &FONT).color(ACCENT_PR))
                                .child(Label::new("BBB", &FONT).color(FG))
                                .child(Label::new("CCC", &FONT).color(ACCENT_SC)),
                            2,
                        ),
                )
                // ── Flex weights ───────────────────────────────
                .child(
                    VStack::new()
                        .child(
                            Label::new("Flex 1 : 2 : 3 (Rects fill all space)", &FONT)
                                .color(ACCENT_TR),
                        )
                        .child(
                            HStack::new()
                                // .gap(3)
                                .child(Rect::new().color(ACCENT_PR))
                                .child_flex(Rect::new().color(FG), 2)
                                .child_flex(Rect::new().color(ACCENT_TR), 3),
                        ),
                )
                .space(),
            5,
        )
        .space()
        // ════════════════════════════════════════════════════════
        // RIGHT COLUMN: Align (cross-axis positioning)
        // Each row has Labels + a Rect. The Rect stretches full
        // height (no intrinsic size), while Labels are positioned
        // by Align. The Rect makes the row height visible.
        // ════════════════════════════════════════════════════════
        .child_flex(
            VStack::new()
                // .gap(4)
                .child(Label::new("ALIGN (cross axis)", &FONT).color(ACCENT_SC))
                // ── Align::Start ───────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Align::Start — labels at top", &FONT).color(ACCENT_TR))
                        .child_flex(
                            HStack::new()
                                .align(Align::Start)
                                // .gap(8)
                                .child(Label::new("top", &FONT).color(ACCENT_PR))
                                .child(Label::new("top", &FONT).color(FG))
                                .child(Label::new("top", &FONT).color(ACCENT_SC))
                                .child(Rect::new().color(DIM)),
                            3,
                        ),
                )
                // ── Align::Center ──────────────────────────────
                .child(
                    VStack::new()
                        .child(
                            Label::new("Align::Center — labels centered", &FONT).color(ACCENT_TR),
                        )
                        .child_flex(
                            HStack::new()
                                .align(Align::Center)
                                // .gap(8)
                                .child(Label::new("mid", &FONT).color(ACCENT_PR))
                                .child(Label::new("mid", &FONT).color(FG))
                                .child(Label::new("mid", &FONT).color(ACCENT_SC))
                                .child(Rect::new().color(DIM)),
                            3,
                        ),
                )
                // ── Align::End ─────────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Align::End — labels at bottom", &FONT).color(ACCENT_TR))
                        .child_flex(
                            HStack::new()
                                .align(Align::End)
                                // .gap(8)
                                .child(Label::new("bot", &FONT).color(ACCENT_PR))
                                .child(Label::new("bot", &FONT).color(FG))
                                .child(Label::new("bot", &FONT).color(ACCENT_SC))
                                .child(Rect::new().color(DIM)),
                            3,
                        ),
                )
                // ── Padding + Gap ──────────────────────────────
                .child(
                    VStack::new()
                        .child(Label::new("Padding + Gap", &FONT).color(ACCENT_TR))
                        .child(
                            HStack::new()
                                // .padding(8)
                                // .gap(6)
                                .child(Rect::new().color(ACCENT_PR))
                                .child(Rect::new().color(FG))
                                .child(Rect::new().color(ACCENT_SC)),
                        ),
                )
                .space(),
            5,
        )
        .paint(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("microface — justify & align demo", &output_settings);
    window.update(&mut display);

    loop {
        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
