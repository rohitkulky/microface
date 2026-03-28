use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::geometry::Size;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// A simple in-memory framebuffer that implements `DrawTarget` for any color
/// type that converts to `Rgb888`. Exports to 24-bit BMP.
///
/// Works with `Rgb565`, `Rgb888`, `Gray4`, `Gray8`, `BinaryColor`, etc.
pub struct BitmapTarget<C: PixelColor + Into<Rgb888> = Rgb888> {
    width: u32,
    height: u32,
    /// Pixel data stored as packed RGB888 (3 bytes per pixel), row-major.
    pixels: Vec<[u8; 3]>,
    _color: core::marker::PhantomData<C>,
}

impl<C: PixelColor + Into<Rgb888>> BitmapTarget<C> {
    /// Create a new bitmap target with the given dimensions (black background).
    pub fn new(width: u32, height: u32) -> Self {
        BitmapTarget {
            width,
            height,
            pixels: vec![[0u8; 3]; (width * height) as usize],
            _color: core::marker::PhantomData,
        }
    }

    /// Export the framebuffer as a 24-bit BMP file.
    pub fn export_bmp(&self, path: &Path) -> std::io::Result<()> {
        let (w, h) = (self.width, self.height);
        let row_size = ((w * 3 + 3) / 4) * 4;
        let pixel_data_size = row_size * h;
        let file_size = 54 + pixel_data_size;

        let mut file = File::create(path)?;

        // BMP Header (14 bytes)
        file.write_all(b"BM")?;
        file.write_all(&(file_size as u32).to_le_bytes())?;
        file.write_all(&0u32.to_le_bytes())?; // reserved
        file.write_all(&54u32.to_le_bytes())?; // pixel data offset

        // DIB Header (40 bytes)
        file.write_all(&40u32.to_le_bytes())?;
        file.write_all(&(w as i32).to_le_bytes())?;
        file.write_all(&(h as i32).to_le_bytes())?;
        file.write_all(&1u16.to_le_bytes())?;
        file.write_all(&24u16.to_le_bytes())?;
        file.write_all(&0u32.to_le_bytes())?;
        file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
        file.write_all(&2835u32.to_le_bytes())?;
        file.write_all(&2835u32.to_le_bytes())?;
        file.write_all(&0u32.to_le_bytes())?;
        file.write_all(&0u32.to_le_bytes())?;

        // Pixel data (bottom-up, BGR order)
        let padding = vec![0u8; (row_size - w * 3) as usize];
        for y in (0..h).rev() {
            for x in 0..w {
                let [r, g, b] = self.pixels[(y * w + x) as usize];
                file.write_all(&[b, g, r])?;
            }
            file.write_all(&padding)?;
        }
        Ok(())
    }
}

impl<C> DrawTarget for BitmapTarget<C>
where
    C: PixelColor + Into<Rgb888>,
{
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            let (x, y) = (point.x, point.y);
            if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
                let rgb: Rgb888 = color.into();
                self.pixels[(y as u32 * self.width + x as u32) as usize] =
                    [rgb.r(), rgb.g(), rgb.b()];
            }
        }
        Ok(())
    }
}

impl<C: PixelColor + Into<Rgb888>> OriginDimensions for BitmapTarget<C> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
