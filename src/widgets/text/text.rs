use core::cell::Cell;

use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;
use embedded_text::TextBox;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::style::{HeightMode, TextBoxStyleBuilder};
use embedded_graphics::text::renderer::TextRenderer;

use crate::color::{FG, GraphicsColorMode};
use crate::fonts::{MicroFont, MicroFontStyle};

/// A text element supporting single-line and multi-line (word-wrapped) rendering.
///
/// Uses `embedded_text::TextBox` for multi-line layout.
pub struct Text<'a> {
    text: &'a str,
    font: &'a MicroFont,
    color: GraphicsColorMode,
    h_align: HorizontalAlignment,
    max_width: Option<u32>,
    cached_size: Cell<Option<Size>>,
    /// Debug counter: how many times the expensive measurement path ran.
    /// Should be 1 after any number of `measure()` calls (cache working).
    compute_count: Cell<u32>,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str, font: &'a MicroFont) -> Self {
        Text {
            text,
            font,
            color: FG,
            h_align: HorizontalAlignment::Left,
            max_width: None,
            cached_size: Cell::new(None),
            compute_count: Cell::new(0),
        }
    }
    
    /// Set the text color.
    pub fn color(mut self, color: impl Into<GraphicsColorMode>) -> Self {
        self.color = color.into();
        self
    }
    
    /// Left-align text (default).
    pub fn left(mut self) -> Self {
        self.h_align = HorizontalAlignment::Left;
        self.cached_size = Cell::new(None);
        self
    }

    /// Center text horizontally.
    pub fn center(mut self) -> Self {
        self.h_align = HorizontalAlignment::Center;
        self.cached_size = Cell::new(None);
        self
    }

    /// Right-align text.
    pub fn right(mut self) -> Self {
        self.h_align = HorizontalAlignment::Right;
        self.cached_size = Cell::new(None);
        self
    }

    /// Justify text (distribute words to fill width).
    pub fn justified(mut self) -> Self {
        self.h_align = HorizontalAlignment::Justified;
        self.cached_size = Cell::new(None);
        self
    }

    /// Enable multi-line word wrapping at the given pixel width.
    /// Without this, Text behaves as single-line (like Label).
    pub fn max_width(mut self, w: u32) -> Self {
        self.max_width = Some(w);
        self.cached_size = Cell::new(None);
        self
    }

        /// Return the intrinsic pixel size of this text.
    ///
    /// Single-line: width from glyph metrics, height from font line height.
    /// Multi-line: width = max_width, height from word-wrapped text measurement.
    ///
    /// Result is cached — subsequent calls return instantly (pretext-inspired).
    pub fn measure(&self) -> Size {
        if let Some(size) = self.cached_size.get() {
            return size;
        }

        let style = MicroFontStyle::new(self.font, self.color);

        let size = match self.max_width {
            None => {
                // Single-line: measure string width directly
                let metrics = style.measure_string(
                    self.text,
                    Point::zero(),
                    embedded_graphics::text::Baseline::Top,
                );
                metrics.bounding_box.size
            }
            Some(max_w) => {
                // Multi-line: use embedded-text to measure wrapped height
                let tb_style = TextBoxStyleBuilder::new()
                    .alignment(self.h_align)
                    .height_mode(HeightMode::FitToText)
                    .build();
                let height = tb_style.measure_text_height(&style, self.text, max_w);
                Size::new(max_w, height)
            }
        };

        self.compute_count.set(self.compute_count.get() + 1);
        self.cached_size.set(Some(size));
        size
    }

    /// How many times the expensive measurement actually ran (debug only).
    /// If cache works, this should be 1 no matter how many times `measure()` is called.
    pub fn compute_count(&self) -> u32 {
        self.compute_count.get()
    }

    /// Draw the text within `bounds`, respecting alignment and wrapping mode.
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        let style = MicroFontStyle::new(self.font, self.color);

        match self.max_width {
            None => {
                // Single-line fast path — skip TextBox overhead
                let text_width = self.measure().width; // cached!

                let origin = match self.h_align {
                    HorizontalAlignment::Left | HorizontalAlignment::Justified => {
                        bounds.top_left
                    }
                    HorizontalAlignment::Center => {
                        let offset =
                            bounds.size.width.saturating_sub(text_width) as i32 / 2;
                        Point::new(bounds.top_left.x + offset, bounds.top_left.y)
                    }
                    HorizontalAlignment::Right => {
                        let offset =
                            bounds.size.width.saturating_sub(text_width) as i32;
                        Point::new(bounds.top_left.x + offset, bounds.top_left.y)
                    }
                };

                embedded_graphics::text::Text::with_baseline(
                    self.text,
                    origin,
                    style,
                    embedded_graphics::text::Baseline::Top,
                )
                .draw(target)?;

                Ok(())
            }
            Some(_) => {
                // Multi-line path — delegate to embedded-text's TextBox
                let tb_style = TextBoxStyleBuilder::new()
                    .alignment(self.h_align)
                    .height_mode(HeightMode::FitToText)
                    .build();

                let _ = TextBox::with_textbox_style(
                    self.text, bounds, style, tb_style,
                )
                .draw(target)?;

                Ok(())
            }
        }
    }
}
