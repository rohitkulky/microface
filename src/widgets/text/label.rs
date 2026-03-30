use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::Drawable;

use crate::color::{GraphicsColorMode, FG};
use crate::fonts::{MicroFont, MicroFontStyle};

/// Horizontal text alignment within the label bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    /// Align text to the left edge (default).
    Left,
    /// Center text horizontally within bounds.
    Center,
    /// Align text to the right edge.
    Right,
}

/// A single-line text label rendered with a [`MicroFont`].
pub struct Label<'a> {
    text: &'a str,
    font: &'a MicroFont,
    color: GraphicsColorMode,
    align: TextAlign,
}

impl<'a> Label<'a> {
    /// Create a label with the given text and font. Color defaults to [`FG`].
    pub fn new(text: &'a str, font: &'a MicroFont) -> Self {
        Label { text, font, color: FG, align: TextAlign::Left }
    }

    /// Set the text color.
    pub fn color(mut self, c: GraphicsColorMode) -> Self {
        self.color = c;
        self
    }

    /// Set horizontal text alignment within the label bounds.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Return the intrinsic pixel size of this label's text.
    ///
    /// Width comes from glyph metrics; height is the font's line height.
    pub fn measure(&self) -> Size {
        let style = MicroFontStyle::new(self.font, self.color);
        let metrics = style.measure_string(
            self.text,
            Point::zero(),
            Baseline::Top,
        );
        metrics.bounding_box.size
    }

    /// Draw the label within `bounds`, respecting alignment.
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        let style = MicroFontStyle::new(self.font, self.color);

        let origin = match self.align {
            TextAlign::Left => bounds.top_left,
            TextAlign::Center | TextAlign::Right => {
                let metrics = style.measure_string(
                    self.text,
                    bounds.top_left,
                    Baseline::Top,
                );
                let text_width = metrics.bounding_box.size.width;
                let offset = match self.align {
                    TextAlign::Center => {
                        bounds.size.width.saturating_sub(text_width) as i32 / 2
                    }
                    TextAlign::Right => {
                        bounds.size.width.saturating_sub(text_width) as i32
                    }
                    _ => unreachable!(),
                };
                Point::new(bounds.top_left.x + offset, bounds.top_left.y)
            }
        };

        Text::with_baseline(self.text, origin, style, Baseline::Top)
            .draw(target)?;
        Ok(())
    }
}
