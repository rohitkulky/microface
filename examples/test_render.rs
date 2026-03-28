//! Test rendering — font.at_size() API for easy size experimentation.
//! Fonts are loaded via include_font! — just change the path/size/bpp.

use microface::Screen;
use microface::color::GraphicsColorMode;
use microface::element::Element;
use microface::widgets::primitives::Rect;
use microface::widgets::layout::HStack;
use microface::render::BitmapTarget;
use microface::include_font;
use microface::fonts::GrayFont;

use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use std::path::Path;

// Fonts are rasterized at compile time — just point at a file
const GOOGLE_SANS_CODE_32: GrayFont = include_font!("fonts/GoudyBookletter1911.ttf", size = 32, bpp = 2);

fn main() {
    let screen = Screen::new(320, 240);
    let mut display = BitmapTarget::new(screen.width, screen.height);

    let ui = HStack::new()
        .child(Element::Rect(Rect::new().color(GraphicsColorMode::BLACK)))
        .child(Element::Rect(Rect::new().color(GraphicsColorMode::BLACK)));
    ui.paint(screen.bounds(), &mut display).unwrap();

    let black = Rgb565::new(0, 0, 0);

    // Just pick a size — at_size() figures out scale/divisor automatically
    let f_medium = GOOGLE_SANS_CODE_32.at_size(32);

    f_medium.draw("Any IP address", Point::new(10, 10), Rgb565::GREEN, black, &mut display).unwrap();
    f_medium.draw("can be either a ", Point::new(10, 40), Rgb565::GREEN, black, &mut display).unwrap();
    f_medium.draw("version four or a", Point::new(10, 70), Rgb565::GREEN, black, &mut display).unwrap();
    f_medium.draw("version six address,", Point::new(10, 100), Rgb565::GREEN, black, &mut display).unwrap();
    f_medium.draw("but not both", Point::new(10, 130), Rgb565::GREEN, black, &mut display).unwrap();
    f_medium.draw("at the same time.", Point::new(10, 160), Rgb565::GREEN, black, &mut display).unwrap();

    display.export_bmp(Path::new("/tmp/test_render.bmp")).unwrap();
    println!("Exported /tmp/test_render.bmp");
}
