//! Bitmap fonts for microface.
//!
//! Fonts are rasterized at compile time via the `include_font!` proc macro.
//! Works with any display: AMOLED, LCD, e-ink (color, grayscale, or B&W).
//!
//! ```ignore
//! use microface::{include_font, fonts::{MicroFont, MicroFontStyle}};
//! use embedded_text::TextBox;
//!
//! const DIN: MicroFont = include_font!("fonts/din.otf", size = 16, bpp = 4);
//!
//! let style = MicroFontStyle::new(&DIN, Rgb565::WHITE).scaled(2);
//! TextBox::new("Wrapped text!", bounds, style).draw(&mut display)?;
//! ```

use core::fmt;

use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::{CharacterStyle, TextMetrics, TextRenderer};
use embedded_graphics::text::{Baseline, DecorationColor};

// ── MicroFont ──────────────────────────────────────────────────────────────

/// An anti-aliased bitmap font with configurable bit depth (1/2/4/8 bpp).
pub struct MicroFont {
    pub data: &'static [u8],
    pub char_width: u32,
    pub char_height: u32,
    pub cols_per_row: u32,
    pub strip_width: u32,
    pub first_char: u8,
    pub last_char: u8,
    pub bpp: u8,
    pub widths: Option<&'static [u8]>,
}

impl fmt::Debug for MicroFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MicroFont")
            .field("char_width", &self.char_width)
            .field("char_height", &self.char_height)
            .field("bpp", &self.bpp)
            .field("data_len", &self.data.len())
            .finish()
    }
}

impl MicroFont {
    #[inline]
    pub fn advance_width(&self, char_idx: u32) -> u32 {
        match self.widths {
            Some(w) => w[char_idx as usize] as u32,
            None => self.char_width,
        }
    }

    #[inline]
    pub fn read_alpha(&self, pixel_index: usize) -> u32 {
        match self.bpp {
            8 => self.data[pixel_index] as u32,
            4 => {
                let byte = self.data[pixel_index / 2];
                (if pixel_index % 2 == 0 { byte >> 4 } else { byte & 0x0F }) as u32 * 17
            }
            2 => ((self.data[pixel_index / 4] >> ((3 - pixel_index % 4) * 2)) & 0x03) as u32 * 85,
            1 => ((self.data[pixel_index / 8] >> (7 - pixel_index % 8)) & 0x01) as u32 * 255,
            _ => 0,
        }
    }
}

// ── Alpha blending in Rgb888 space ─────────────────────────────────────────

#[inline]
fn blend(source: Rgb888, dest: Rgb888, alpha: u8) -> Rgb888 {
    let a = alpha as u16;
    let inv_a = 255 - a;
    Rgb888::new(
        ((source.r() as u16 * a + dest.r() as u16 * inv_a) / 255) as u8,
        ((source.g() as u16 * a + dest.g() as u16 * inv_a) / 255) as u8,
        ((source.b() as u16 * a + dest.b() as u16 * inv_a) / 255) as u8,
    )
}

#[inline]
fn blend_over_black(source: Rgb888, alpha: u8) -> Rgb888 {
    let a = alpha as u16;
    Rgb888::new(
        (source.r() as u16 * a / 255) as u8,
        (source.g() as u16 * a / 255) as u8,
        (source.b() as u16 * a / 255) as u8,
    )
}

// ── MicroFontStyle ─────────────────────────────────────────────────────────

/// Styled font implementing `TextRenderer` + `CharacterStyle`.
///
/// Generic over any `PixelColor` that converts to/from `Rgb888`.
/// Works with `Rgb565`, `Rgb888`, `Gray8`, `Gray4`, `BinaryColor`, etc.
#[derive(Clone, Debug)]
pub struct MicroFontStyle<'a, C> {
    pub font: &'a MicroFont,
    pub text_color: Option<C>,
    pub background_color: Option<C>,
    pub scale: u32,
}

impl<'a, C: PixelColor> MicroFontStyle<'a, C> {
    pub fn new(font: &'a MicroFont, text_color: C) -> Self {
        Self { font, text_color: Some(text_color), background_color: None, scale: 1 }
    }

    pub fn with_background(font: &'a MicroFont, text_color: C, background_color: C) -> Self {
        Self { font, text_color: Some(text_color), background_color: Some(background_color), scale: 1 }
    }

    pub fn scaled(mut self, factor: u32) -> Self { self.scale = factor.max(1); self }

    fn effective_height(&self) -> u32 { self.font.char_height * self.scale }

    fn baseline_offset(&self, baseline: Baseline) -> i32 {
        let h = self.effective_height();
        match baseline {
            Baseline::Top => 0,
            Baseline::Bottom => h.saturating_sub(1) as i32,
            Baseline::Middle => (h.saturating_sub(1) / 2) as i32,
            Baseline::Alphabetic => (h * 4 / 5) as i32,
        }
    }

    fn string_width(&self, text: &str) -> u32 {
        text.chars().map(|ch| {
            let code = ch as u8;
            if code < self.font.first_char || code > self.font.last_char {
                self.font.char_width
            } else {
                self.font.advance_width((code - self.font.first_char) as u32)
            }
        }).sum::<u32>() * self.scale
    }
}

// ── Generic TextRenderer for any PixelColor ↔ Rgb888 ──────────────────────

impl<'a, C> TextRenderer for MicroFontStyle<'a, C>
where
    C: PixelColor + From<Rgb888>,
    Rgb888: From<C>,
{
    type Color = C;

    fn draw_string<D: DrawTarget<Color = C>>(
        &self, text: &str, position: Point, baseline: Baseline, target: &mut D,
    ) -> Result<Point, D::Error> {
        let s = self.scale;
        let top = position - Point::new(0, self.baseline_offset(baseline));
        let mut cx = top.x;

        let fg888 = self.text_color.map(Rgb888::from);
        let bg888 = self.background_color.map(Rgb888::from);
        let (has_fg, has_bg) = (fg888.is_some(), bg888.is_some());

        for ch in text.chars() {
            let code = ch as u8;
            if code < self.font.first_char || code > self.font.last_char {
                let w = self.font.char_width * s;
                if has_bg {
                    target.fill_solid(&Rectangle::new(Point::new(cx, top.y), Size::new(w, self.effective_height())), self.background_color.unwrap())?;
                }
                cx += w as i32;
                continue;
            }
            let idx = (code - self.font.first_char) as u32;
            let gx = (idx % self.font.cols_per_row) * self.font.char_width;
            let gy = (idx / self.font.cols_per_row) * self.font.char_height;

            for row in 0..self.font.char_height {
                for col in 0..self.font.char_width {
                    let alpha = self.font.read_alpha(((gy + row) * self.font.strip_width + gx + col) as usize);

                    let color: Option<C> = if alpha == 0 {
                        if has_bg { Some(self.background_color.unwrap()) } else { None }
                    } else if alpha == 255 && has_fg {
                        Some(self.text_color.unwrap())
                    } else if has_fg {
                        let blended = if has_bg {
                            blend(fg888.unwrap(), bg888.unwrap(), alpha as u8)
                        } else {
                            blend_over_black(fg888.unwrap(), alpha as u8)
                        };
                        Some(C::from(blended))
                    } else { None };

                    if let Some(c) = color {
                        let (bx, by) = (cx + (col * s) as i32, top.y + (row * s) as i32);
                        if s == 1 { Pixel(Point::new(bx, by), c).draw(target)?; }
                        else { target.fill_solid(&Rectangle::new(Point::new(bx, by), Size::new(s, s)), c)?; }
                    }
                }
            }
            cx += (self.font.advance_width(idx) * s) as i32;
        }
        Ok(Point::new(cx, position.y))
    }

    fn draw_whitespace<D: DrawTarget<Color = C>>(
        &self, width: u32, position: Point, baseline: Baseline, target: &mut D,
    ) -> Result<Point, D::Error> {
        let top = position - Point::new(0, self.baseline_offset(baseline));
        if width > 0 { if let Some(bg) = self.background_color {
            target.fill_solid(&Rectangle::new(top, Size::new(width, self.effective_height())), bg)?;
        }}
        Ok(Point::new(position.x + width as i32, position.y))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let top = position - Point::new(0, self.baseline_offset(baseline));
        let w = self.string_width(text);
        TextMetrics {
            bounding_box: Rectangle::new(top, Size::new(w, self.effective_height())),
            next_position: Point::new(position.x + w as i32, position.y),
        }
    }

    fn line_height(&self) -> u32 { self.effective_height() }
}

impl<'a, C> CharacterStyle for MicroFontStyle<'a, C>
where
    C: PixelColor + From<Rgb888>,
    Rgb888: From<C>,
{
    type Color = C;
    fn set_text_color(&mut self, c: Option<Self::Color>) { self.text_color = c; }
    fn set_background_color(&mut self, c: Option<Self::Color>) { self.background_color = c; }
    fn set_underline_color(&mut self, _: DecorationColor<Self::Color>) {}
    fn set_strikethrough_color(&mut self, _: DecorationColor<Self::Color>) {}
}
