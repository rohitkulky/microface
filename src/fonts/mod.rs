//! Bitmap fonts for microface.
//!
//! Fonts are rasterized at compile time via the `include_font!` proc macro.
//! Just point at a TTF/OTF file and specify size + bpp — no external tools needed.
//!
//! # Usage
//!
//! ```ignore
//! use microface::{include_font, fonts::GrayFont};
//!
//! // Drop any TTF/OTF in your project and reference it — that's it
//! const MY_FONT: GrayFont = include_font!("fonts/myfont.ttf", size = 24, bpp = 4);
//!
//! // Use it
//! MY_FONT.draw_string("Hello", pos, fg, bg, &mut display)?;
//!
//! // Or scale it
//! MY_FONT.at_size(48).draw("Big text", pos, fg, bg, &mut display)?;
//! ```

use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

// ── GrayFont ───────────────────────────────────────────────────────────────

/// An anti-aliased bitmap font with configurable bit depth.
///
/// Glyphs are stored in a grid (16 chars per row), packed at `bpp` bits per pixel
/// (MSB-first). Supported bpp values: 1, 2, 4, 8.
///
/// - `bpp=8`: 1 byte per pixel, 256 alpha levels (no packing)
/// - `bpp=4`: 2 pixels per byte, 16 alpha levels
/// - `bpp=2`: 4 pixels per byte, 4 alpha levels
/// - `bpp=1`: 8 pixels per byte, binary (on/off)
pub struct GrayFont {
    pub data: &'static [u8],
    pub char_width: u32,
    pub char_height: u32,
    pub cols_per_row: u32,
    /// Width of the bitmap strip in pixels (not bytes).
    pub strip_width: u32,
    pub first_char: u8,
    pub last_char: u8,
    /// Bits per pixel: 1, 2, 4, or 8.
    pub bpp: u8,
    /// Per-glyph advance widths for proportional fonts.
    /// If `None`, uses `char_width` for all glyphs (monospace).
    /// Array of (last_char - first_char + 1) bytes, one per glyph.
    pub widths: Option<&'static [u8]>,
}

/// A font at a specific target pixel size, created from a base `GrayFont`.
///
/// Automatically determines whether to upscale or downscale from the base.
/// ```ignore
/// let small = DINROUNDPRO_32.at_size(16);  // downscale ÷2
/// let big   = DINROUNDPRO_32.at_size(64);  // upscale ×2
/// let native = DINROUNDPRO_32.at_size(32); // no scaling
/// small.draw("Hello", pos, white, black, &mut display)?;
/// ```
pub struct FontAt<'a> {
    font: &'a GrayFont,
    target_height: u32,
}

impl<'a> FontAt<'a> {
    /// Draw text at the configured target size.
    pub fn draw<D>(
        &self,
        text: &str,
        pos: Point,
        fg: Rgb565,
        bg: Rgb565,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let base_h = self.font.char_height;
        let target_h = self.target_height;

        if target_h == base_h {
            // Native size
            self.font.draw_string(text, pos, fg, bg, target)
        } else if target_h > base_h {
            // Upscale
            let scale = target_h / base_h;
            self.font.draw_string_scaled(text, pos, fg, bg, scale.max(1), target)
        } else {
            // Downscale
            let divisor = base_h / target_h;
            self.font.draw_string_downscaled(text, pos, fg, bg, divisor.max(1), target)
        }
    }
}

impl GrayFont {
    /// Create a font at a specific target pixel height.
    ///
    /// Automatically picks upscale or downscale based on the base size.
    /// For best results, use target sizes that are integer multiples or divisors
    /// of the base height.
    pub fn at_size(&self, target_height: u32) -> FontAt<'_> {
        FontAt {
            font: self,
            target_height,
        }
    }

    /// Get the advance width for a character (proportional or fixed).
    #[inline]
    fn advance_width(&self, char_idx: u32) -> u32 {
        match self.widths {
            Some(w) => w[char_idx as usize] as u32,
            None => self.char_width,
        }
    }

    /// Read a single pixel's alpha value (0-255) from the packed bitmap.
    #[inline]
    fn read_alpha(&self, pixel_index: usize) -> u32 {
        match self.bpp {
            8 => self.data[pixel_index] as u32,
            4 => {
                let byte_idx = pixel_index / 2;
                let nibble = if pixel_index % 2 == 0 {
                    (self.data[byte_idx] >> 4) & 0x0F
                } else {
                    self.data[byte_idx] & 0x0F
                };
                // Scale 0-15 → 0-255: val * 17
                nibble as u32 * 17
            }
            2 => {
                let byte_idx = pixel_index / 4;
                let slot = pixel_index % 4;
                let shift = (3 - slot) * 2;
                let val = (self.data[byte_idx] >> shift) & 0x03;
                // Scale 0-3 → 0-255: val * 85
                val as u32 * 85
            }
            1 => {
                let byte_idx = pixel_index / 8;
                let bit = pixel_index % 8;
                let shift = 7 - bit;
                let val = (self.data[byte_idx] >> shift) & 0x01;
                // Scale 0-1 → 0-255
                val as u32 * 255
            }
            _ => 0,
        }
    }

    /// Draw a string at the given position with the given color (native size).
    /// Alpha-blends each glyph pixel against a background color.
    pub fn draw_string<D>(
        &self,
        text: &str,
        pos: Point,
        fg: Rgb565,
        bg: Rgb565,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.draw_string_scaled(text, pos, fg, bg, 1, target)
    }

    /// Draw a string with integer scaling (upscale or downscale).
    ///
    /// `scale=1`: native size (e.g. 24px)
    /// `scale=2`: upscale ×2 (24px → 48px, nearest-neighbor)
    /// `scale=3`: upscale ×3 (24px → 72px)
    ///
    /// For downscaling, use `draw_string_downscaled` instead.
    pub fn draw_string_scaled<D>(
        &self,
        text: &str,
        pos: Point,
        fg: Rgb565,
        bg: Rgb565,
        scale: u32,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let scale = if scale == 0 { 1 } else { scale };
        let mut cursor_x = pos.x;

        for ch in text.chars() {
            let code = ch as u8;
            if code < self.first_char || code > self.last_char {
                cursor_x += (self.char_width * scale) as i32;
                continue;
            }
            let idx = (code - self.first_char) as u32;
            let grid_col = idx % self.cols_per_row;
            let grid_row = idx / self.cols_per_row;
            let glyph_x = grid_col * self.char_width;
            let glyph_y = grid_row * self.char_height;

            let fg_r = fg.r() as u32;
            let fg_g = fg.g() as u32;
            let fg_b = fg.b() as u32;
            let bg_r = bg.r() as u32;
            let bg_g = bg.g() as u32;
            let bg_b = bg.b() as u32;

            for row in 0..self.char_height {
                for col in 0..self.char_width {
                    let src_x = glyph_x + col;
                    let src_y = glyph_y + row;
                    let pixel_index = (src_y * self.strip_width + src_x) as usize;
                    let alpha = self.read_alpha(pixel_index);

                    if alpha > 0 {
                        let r = (fg_r * alpha + bg_r * (255 - alpha)) / 255;
                        let g = (fg_g * alpha + bg_g * (255 - alpha)) / 255;
                        let b = (fg_b * alpha + bg_b * (255 - alpha)) / 255;
                        let blended = Rgb565::new(r as u8, g as u8, b as u8);

                        let base_x = cursor_x + (col * scale) as i32;
                        let base_y = pos.y + (row * scale) as i32;
                        for sy in 0..scale {
                            for sx in 0..scale {
                                Pixel(Point::new(base_x + sx as i32, base_y + sy as i32), blended)
                                    .draw(target)?;
                            }
                        }
                    }
                }
            }
            cursor_x += (self.advance_width(idx) * scale) as i32;
        }

        Ok(())
    }

    /// Draw a string downscaled by an integer divisor (box filter averaging).
    ///
    /// `divisor=1`: native size (24px)
    /// `divisor=2`: half size (24px → 12px, smooth box-filtered)
    /// `divisor=3`: third size (24px → 8px)
    ///
    /// Each output pixel is the average of a divisor×divisor block of source pixels.
    /// This produces smooth, high-quality downscaled text.
    pub fn draw_string_downscaled<D>(
        &self,
        text: &str,
        pos: Point,
        fg: Rgb565,
        bg: Rgb565,
        divisor: u32,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let divisor = if divisor == 0 { 1 } else { divisor };
        if divisor == 1 {
            return self.draw_string(text, pos, fg, bg, target);
        }

        let out_h = self.char_height / divisor;
        let mut cursor_x = pos.x;

        for ch in text.chars() {
            let code = ch as u8;
            if code < self.first_char || code > self.last_char {
                cursor_x += (self.char_width / divisor) as i32;
                continue;
            }
            let idx = (code - self.first_char) as u32;
            let grid_col = idx % self.cols_per_row;
            let grid_row = idx / self.cols_per_row;
            let glyph_x = grid_col * self.char_width;
            let glyph_y = grid_row * self.char_height;
            let out_w = self.advance_width(idx) / divisor;

            let fg_r = fg.r() as u32;
            let fg_g = fg.g() as u32;
            let fg_b = fg.b() as u32;
            let bg_r = bg.r() as u32;
            let bg_g = bg.g() as u32;
            let bg_b = bg.b() as u32;
            let area = divisor * divisor;

            for out_row in 0..out_h {
                for out_col in 0..(self.char_width / divisor) {
                    // Average a divisor×divisor block of source pixels
                    let mut alpha_sum = 0u32;
                    for dy in 0..divisor {
                        for dx in 0..divisor {
                            let src_x = glyph_x + out_col * divisor + dx;
                            let src_y = glyph_y + out_row * divisor + dy;
                            let pixel_index = (src_y * self.strip_width + src_x) as usize;
                            alpha_sum += self.read_alpha(pixel_index);
                        }
                    }
                    let alpha = alpha_sum / area;

                    if alpha > 0 {
                        let r = (fg_r * alpha + bg_r * (255 - alpha)) / 255;
                        let g = (fg_g * alpha + bg_g * (255 - alpha)) / 255;
                        let b = (fg_b * alpha + bg_b * (255 - alpha)) / 255;
                        let blended = Rgb565::new(r as u8, g as u8, b as u8);

                        Pixel(
                            Point::new(cursor_x + out_col as i32, pos.y + out_row as i32),
                            blended,
                        ).draw(target)?;
                    }
                }
            }
            cursor_x += out_w as i32;
        }

        Ok(())
    }
}
