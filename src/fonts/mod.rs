//! Bitmap fonts for microface.
//!
//! Fonts are rasterized at compile time via the `include_font!` proc macro.
//! Works with any display: AMOLED, LCD, e-ink (color, grayscale, or B&W).
//!
//! Rendering uses a **16-entry LUT** built once per draw call,
//! `fill_contiguous` for single-call-per-glyph rendering,
//! tight per-glyph bounding boxes, and optional kerning.
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

const PALETTE_SIZE: usize = 16;

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
    /// Tight bounding boxes: flat [x_off, y_off, tight_w, tight_h] × glyph_count.
    pub bboxes: &'static [u8],
    /// Kerning pairs: flat [left_idx, right_idx, adjustment_as_u8] triples, or None.
    pub kerning: Option<&'static [u8]>,
}

impl fmt::Debug for MicroFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MicroFont")
            .field("char_width", &self.char_width)
            .field("char_height", &self.char_height)
            .field("bpp", &self.bpp)
            .field("data_len", &self.data.len())
            .field("has_kerning", &self.kerning.is_some())
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

    /// Tight bounding box for a glyph: (x_off, y_off, tight_w, tight_h).
    #[inline]
    pub fn glyph_bbox(&self, char_idx: u32) -> (u32, u32, u32, u32) {
        let base = (char_idx as usize) * 4;
        if base + 3 < self.bboxes.len() {
            (self.bboxes[base] as u32, self.bboxes[base + 1] as u32,
             self.bboxes[base + 2] as u32, self.bboxes[base + 3] as u32)
        } else {
            (0, 0, self.char_width, self.char_height)
        }
    }

    /// Kerning adjustment between two glyph indices (in pixels, signed).
    #[inline]
    pub fn kern(&self, left_idx: u32, right_idx: u32) -> i32 {
        let table = match self.kerning {
            Some(k) => k,
            None => return 0,
        };
        let (l, r) = (left_idx as u8, right_idx as u8);
        let mut i = 0;
        while i + 2 < table.len() {
            if table[i] == l && table[i + 1] == r {
                return table[i + 2] as i8 as i32;
            }
            i += 3;
        }
        0
    }

    /// Read full 0–255 alpha (legacy, kept for compatibility).
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

    /// Read raw alpha as a 4-bit palette index (0–15), normalized from any bpp.
    #[inline]
    pub fn read_alpha_index(&self, pixel_index: usize) -> u8 {
        match self.bpp {
            4 => { let b = self.data[pixel_index / 2]; if pixel_index % 2 == 0 { b >> 4 } else { b & 0x0F } }
            8 => self.data[pixel_index] >> 4,
            2 => ((self.data[pixel_index / 4] >> ((3 - pixel_index % 4) * 2)) & 0x03) * 5,
            1 => ((self.data[pixel_index / 8] >> (7 - pixel_index % 8)) & 0x01) * 15,
            _ => 0,
        }
    }
}

// ── Alpha blending ─────────────────────────────────────────────────────────

/// Fast alpha blend: replaces `/255` with `(x + 128) >> 8` (max error: ±1).
#[inline]
fn blend(src: Rgb888, dst: Rgb888, a: u8) -> Rgb888 {
    let (a, inv) = (a as u16, (255 - a) as u16);
    Rgb888::new(
        ((src.r() as u16 * a + dst.r() as u16 * inv + 128) >> 8) as u8,
        ((src.g() as u16 * a + dst.g() as u16 * inv + 128) >> 8) as u8,
        ((src.b() as u16 * a + dst.b() as u16 * inv + 128) >> 8) as u8,
    )
}

#[inline]
fn blend_over_black(src: Rgb888, a: u8) -> Rgb888 {
    let a = a as u16;
    Rgb888::new(
        ((src.r() as u16 * a + 128) >> 8) as u8,
        ((src.g() as u16 * a + 128) >> 8) as u8,
        ((src.b() as u16 * a + 128) >> 8) as u8,
    )
}

/// Build 16-entry palette once, replacing all per-pixel blend math with a LUT lookup.
fn build_palette<C: PixelColor + From<Rgb888>>(fg: Rgb888, bg: Option<Rgb888>) -> [C; PALETTE_SIZE] {
    let mut pal = [C::from(Rgb888::new(0, 0, 0)); PALETTE_SIZE];
    for i in 0..PALETTE_SIZE {
        let alpha = (i as u8) * 17;
        pal[i] = C::from(match bg {
            Some(bg) => blend(fg, bg, alpha),
            None => blend_over_black(fg, alpha),
        });
    }
    pal
}

// ── GlyphIterator (tight bbox aware, unified scale) ───────────────────────

/// Yields pre-blended colors for a glyph's tight bounding box, with scaling.
/// Used with `fill_contiguous` for single-call-per-glyph rendering.
struct GlyphIter<'a, C> {
    font: &'a MicroFont,
    pal: [C; PALETTE_SIZE],
    gx: u32, gy: u32,       // tight bbox origin in atlas (cell_x + x_off, cell_y + y_off)
    w: u32, h: u32,          // tight bbox dimensions
    scale: u32,
    cur: u32,
}

impl<'a, C: PixelColor + Copy> Iterator for GlyphIter<'a, C> {
    type Item = C;
    #[inline]
    fn next(&mut self) -> Option<C> {
        let ow = self.w * self.scale;
        let total = ow * self.h * self.scale;
        if self.cur >= total { return None; }
        let (col, row) = (self.cur % ow / self.scale, self.cur / ow / self.scale);
        self.cur += 1;
        let idx = ((self.gy + row) * self.font.strip_width + self.gx + col) as usize;
        Some(self.pal[self.font.read_alpha_index(idx) as usize])
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let r = (self.w * self.scale * self.h * self.scale - self.cur) as usize;
        (r, Some(r))
    }
}

// ── MicroFontStyle ─────────────────────────────────────────────────────────

/// Styled font implementing `TextRenderer` + `CharacterStyle`.
///
/// Generic over any `PixelColor` that converts to/from `Rgb888`.
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
        let mut width = 0u32;
        let mut prev_idx: Option<u32> = None;
        for ch in text.chars() {
            let code = ch as u8;
            if code < self.font.first_char || code > self.font.last_char {
                width += self.font.char_width;
                prev_idx = None;
            } else {
                let idx = (code - self.font.first_char) as u32;
                if let Some(pi) = prev_idx {
                    let k = self.font.kern(pi, idx);
                    width = (width as i32 + k).max(0) as u32;
                }
                width += self.font.advance_width(idx);
                prev_idx = Some(idx);
            }
        }
        width * self.scale
    }
}

// ── TextRenderer ───────────────────────────────────────────────────────────

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
        let has_bg = bg888.is_some();

        // Build palette ONCE for the entire string (16 blends, not 256/glyph)
        let palette: Option<[C; PALETTE_SIZE]> = fg888.map(|fg| build_palette(fg, bg888));

        let mut prev_idx: Option<u32> = None;

        for ch in text.chars() {
            let code = ch as u8;
            if code < self.font.first_char || code > self.font.last_char {
                let w = self.font.char_width * s;
                if has_bg {
                    target.fill_solid(&Rectangle::new(Point::new(cx, top.y), Size::new(w, self.effective_height())), self.background_color.unwrap())?;
                }
                cx += w as i32;
                prev_idx = None;
                continue;
            }

            let idx = (code - self.font.first_char) as u32;

            // Apply kerning
            if let Some(pi) = prev_idx {
                cx += (self.font.kern(pi, idx) * s as i32) as i32;
            }
            prev_idx = Some(idx);

            let cell_gx = (idx % self.font.cols_per_row) * self.font.char_width;
            let cell_gy = (idx / self.font.cols_per_row) * self.font.char_height;
            let advance = self.font.advance_width(idx);
            let (bbox_xo, bbox_yo, bbox_w, bbox_h) = self.font.glyph_bbox(idx);

            if let Some(ref pal) = palette {
                if has_bg {
                    // Fill full advance rectangle with background first
                    let full_area = Rectangle::new(Point::new(cx, top.y), Size::new(advance * s, self.font.char_height * s));
                    target.fill_solid(&full_area, self.background_color.unwrap())?;

                    // Then draw only the tight bbox with fill_contiguous
                    if bbox_w > 0 && bbox_h > 0 {
                        let tight_area = Rectangle::new(
                            Point::new(cx + (bbox_xo * s) as i32, top.y + (bbox_yo * s) as i32),
                            Size::new(bbox_w * s, bbox_h * s),
                        );
                        target.fill_contiguous(&tight_area, GlyphIter {
                            font: self.font, pal: *pal,
                            gx: cell_gx + bbox_xo, gy: cell_gy + bbox_yo,
                            w: bbox_w, h: bbox_h, scale: s, cur: 0,
                        })?;
                    }
                } else {
                    // No background: skip transparent pixels, still use LUT
                    if bbox_w > 0 && bbox_h > 0 {
                        for row in 0..bbox_h {
                            for col in 0..bbox_w {
                                let pi = ((cell_gy + bbox_yo + row) * self.font.strip_width + cell_gx + bbox_xo + col) as usize;
                                let ai = self.font.read_alpha_index(pi) as usize;
                                if ai == 0 { continue; }
                                let c = pal[ai];
                                let px = cx + ((bbox_xo + col) * s) as i32;
                                let py = top.y + ((bbox_yo + row) * s) as i32;
                                if s == 1 {
                                    Pixel(Point::new(px, py), c).draw(target)?;
                                } else {
                                    target.fill_solid(&Rectangle::new(Point::new(px, py), Size::new(s, s)), c)?;
                                }
                            }
                        }
                    }
                }
            } else if has_bg {
                target.fill_solid(&Rectangle::new(Point::new(cx, top.y), Size::new(advance * s, self.effective_height())), self.background_color.unwrap())?;
            }

            cx += (advance * s) as i32;
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
