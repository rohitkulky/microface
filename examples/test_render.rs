//! Test rendering — MicroFontStyle + embedded-text TextBox with Gray4 display.

use microface::fonts::{MicroFont, MicroFontStyle};
use microface::include_font;
use microface::render::BitmapTarget;

use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_text::TextBox;
use std::path::Path;

const INTER_8: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 8);
const INTER_4: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 4);
const INTER_2: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 2);
const INTER_1: MicroFont = include_font!("fonts/Inter.ttf", size = 16, bpp = 1);

fn main() {
    let mut display = BitmapTarget::<Gray4>::new(320, 240);

    // Compare bpp levels at native 16px
    let mut y = 5;
    for (font, label) in [
        (&INTER_8, "8bpp anti-aliased text"),
        (&INTER_4, "4bpp anti-aliased text"),
        (&INTER_2, "2bpp anti-aliased text"),
        (&INTER_1, "1bpp binary text"),
    ] {
        let style = MicroFontStyle::new(font, Gray4::WHITE);
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
    let style_2x = MicroFontStyle::new(&DIN_4, Gray4::new(0x0A)).scaled(2);
    TextBox::new(
        "Scaled 2x with word wrap!",
        Rectangle::new(Point::new(10, y + 5), Size::new(300, 130)),
        style_2x,
    )
    .draw(&mut display)
    .unwrap();

    display.export_bmp(Path::new("/tmp/test_render.bmp")).unwrap();
    println!("Exported  /tmp/test_render.bmp");
}
