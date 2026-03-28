use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::geometry::Size;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// A simple in-memory framebuffer that implements DrawTarget.
/// Renders to Rgb565 internally and can export to BMP.
pub struct BitmapTarget {
    width: u32,
    height: u32,
    /// Pixel data stored as RGB565 (2 bytes per pixel), row-major.
    pixels: Vec<u16>,
}

impl BitmapTarget {
    /// Create a new bitmap target with the given dimensions.
    /// All pixels start as black (0x0000).
    pub fn new(width: u32, height: u32) -> Self {
        BitmapTarget {
            width,
            height,
            pixels: vec![0u16; (width * height) as usize],
        }
    }

    /// Export the framebuffer as a 24-bit BMP file.
    pub fn export_bmp(&self, path: &Path) -> std::io::Result<()> {
        let w = self.width;
        let h = self.height;

        // BMP rows must be padded to 4-byte boundaries
        let row_size = ((w * 3 + 3) / 4) * 4;
        let pixel_data_size = row_size * h;
        let file_size = 54 + pixel_data_size;

        let mut file = File::create(path)?;

        // BMP Header (14 bytes)
        file.write_all(b"BM")?;
        file.write_all(&(file_size as u32).to_le_bytes())?;
        file.write_all(&0u16.to_le_bytes())?; // reserved
        file.write_all(&0u16.to_le_bytes())?; // reserved
        file.write_all(&54u32.to_le_bytes())?; // pixel data offset

        // DIB Header (40 bytes) - BITMAPINFOHEADER
        file.write_all(&40u32.to_le_bytes())?; // header size
        file.write_all(&(w as i32).to_le_bytes())?;
        file.write_all(&(h as i32).to_le_bytes())?; // positive = bottom-up
        file.write_all(&1u16.to_le_bytes())?; // color planes
        file.write_all(&24u16.to_le_bytes())?; // bits per pixel
        file.write_all(&0u32.to_le_bytes())?; // no compression
        file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
        file.write_all(&2835u32.to_le_bytes())?; // h resolution (72 DPI)
        file.write_all(&2835u32.to_le_bytes())?; // v resolution
        file.write_all(&0u32.to_le_bytes())?; // colors in palette
        file.write_all(&0u32.to_le_bytes())?; // important colors

        // Pixel data (bottom-up, BGR order)
        let padding = vec![0u8; (row_size - w * 3) as usize];
        for y in (0..h).rev() {
            for x in 0..w {
                let rgb565 = self.pixels[(y * w + x) as usize];
                // Convert RGB565 to RGB888
                let r = ((rgb565 >> 11) & 0x1F) as u8;
                let g = ((rgb565 >> 5) & 0x3F) as u8;
                let b = (rgb565 & 0x1F) as u8;
                let r8 = (r << 3) | (r >> 2);
                let g8 = (g << 2) | (g >> 4);
                let b8 = (b << 3) | (b >> 2);
                // BMP uses BGR order
                file.write_all(&[b8, g8, r8])?;
            }
            file.write_all(&padding)?;
        }

        Ok(())
    }
}

impl DrawTarget for BitmapTarget {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            let x = point.x;
            let y = point.y;
            if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
                let raw = embedded_graphics::pixelcolor::raw::RawU16::from(color).into_inner();
                self.pixels[(y as u32 * self.width + x as u32) as usize] = raw;
            }
        }
        Ok(())
    }
}

impl OriginDimensions for BitmapTarget {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
