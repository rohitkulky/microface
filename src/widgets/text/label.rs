use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use crate::color::{GraphicsColorMode, FG};

/// A text label widget.
///
/// Usage:
/// ```ignore
/// Label::new("Hello", &FONT_6X10)
///     .color(ACCENT_PR)
///     .paint(bounds, &mut display)?;
/// ```
pub struct Label<'a> {
    text: &'a str,
    font: &'a MonoFont<'a>,
    color: GraphicsColorMode,
}

impl<'a> Label<'a> {
    /// Create a new label with text and font. Color defaults to FG.
    pub fn new(text: &'a str, font: &'a MonoFont<'a>) -> Self {
        Label { text, font, color: FG }
    }

    /// Set the text color.
    pub fn color(mut self, c: GraphicsColorMode) -> Self {
        self.color = c;
        self
    }

    /// Draw the label into the given bounds.
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        // Step 1: Create a MonoTextStyle from self.font and self.color
        // Hint: MonoTextStyle::new(font, color)
        let style = MonoTextStyle::new(self.font, self.color);

        // Step 2: Create a Text at the top-left of bounds and draw it
        // Hint: Text::new(self.text, position, style).draw(target)
        // Position comes from bounds — what field gives you the top-left point?
        Text::new(self.text, bounds.top_left, style).draw(target)?;
        Ok(())
    }
}
