//! Test rendering — MicroFontStyle + embedded-text TextBox with Gray4 display.

use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use microface::fonts::{MicroFont, MicroFontStyle};
use microface::include_font;

use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_text::TextBox;
use std::time::Duration;

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

const INTER_8: MicroFont = include_font!("fonts/Inter.ttf", size = 12, bpp = 2);
const INTER_4: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 4);
const INTER_2: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 2);
const INTER_1: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 1);

fn main() {
    let mut display = SimulatorDisplay::<Gray4>::new(Size::new(600, 400));

    // Compare bpp levels at native 16px
    let mut y = 5;
    for (font, label) in [
        (&INTER_8, "8bpp anti-aliased text"),
        (&INTER_4, "4bpp anti-aliased text"),
        (&INTER_2, "2bpp anti-aliased text"),
        (&INTER_1, "1bpp binary text"),
    ] {
        let style = MicroFontStyle::new(font, Gray4::BLACK);
        TextBox::new(
            label,
            Rectangle::new(Point::new(10, y), Size::new(300, 30)),
            style,
        )
        .draw(&mut display)
        .unwrap();
        y += 25;
    }

    // Scaled 2×
    let style_2x = MicroFontStyle::new(&INTER_4, Gray4::new(0x0A)).scaled(2);
    TextBox::new(
        "Scaled 2x with word wrap!",
        Rectangle::new(Point::new(10, y + 5), Size::new(300, 130)),
        style_2x,
    )
    .draw(&mut display)
    .unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("microface", &output_settings);

    loop {
        display.clear(Gray4::BLACK).unwrap();
        draw_clock(&mut display).unwrap();

        window.update(&mut display);

        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

macro_rules! mstyle {
    ($size:expr, $bpp:expr, $color:expr, $scale:expr) => {
        MicroFontStyle::new(
            &include_font!("fonts/Inter.ttf", size = $size, bpp = $bpp),
            $color,
        )
        .scaled($scale)
    };
}

fn draw_clock<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Gray4>,
{
    let style_6 = mstyle!(6, 4, Gray4::WHITE, 1);
    let style_6_2 = mstyle!(6, 4, Gray4::WHITE, 2);
    let style_8 = mstyle!(8, 4, Gray4::WHITE, 1);
    let style_8_2 = mstyle!(8, 4, Gray4::WHITE, 2);
    let style_10 = mstyle!(10, 4, Gray4::WHITE, 1);
    let style_10_2 = mstyle!(10, 4, Gray4::WHITE, 2);
    let style_12 = mstyle!(12, 4, Gray4::WHITE, 1);
    let style_12_2 = mstyle!(12, 4, Gray4::WHITE, 2);
    let style_16 = mstyle!(16, 4, Gray4::WHITE, 1);
    let style_32_8 = mstyle!(32, 8, Gray4::WHITE, 1);
    let style_32_2 = mstyle!(32, 2, Gray4::WHITE, 1);

    // Create a centered alignment for text
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();

    let time = chrono::Local::now().format("%H:%M:%S").to_string();

    macro_rules! drawtext {
        ($ypos:expr, $stl:expr) => {{
            let text = format!("{} --- {:?}", time, $ypos);
            Text::with_text_style(&text, Point { x: 10, y: $ypos }, $stl, text_style).draw(display)?;
        }};
    }

    drawtext!(10, style_6);
    drawtext!(40, style_6_2);
    drawtext!(60, style_8);
    drawtext!(80, style_8_2);
    drawtext!(100, style_10);
    drawtext!(120, style_10_2);
    drawtext!(140, style_12);
    drawtext!(160, style_12_2);
    drawtext!(180, style_16);
    drawtext!(200, style_32_2);
    drawtext!(240, style_32_8);

    Ok(())
}
