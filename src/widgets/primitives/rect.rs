use embedded_graphics::prelude::{DrawTarget, Primitive};
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::Drawable;

use crate::color::{GraphicsColorMode, FG};

/// A rectangle widget, either filled or stroked.
pub struct Rect {
    color: GraphicsColorMode,
    filled: bool,
}

impl Rect {
    /// Create a filled rectangle using the default foreground color ([`FG`]).
    pub fn new() -> Self {
        Rect {
            color: FG,
            filled: true,
        }
    }

    /// Set the fill or stroke color.
    pub fn color(mut self, c: GraphicsColorMode) -> Self {
        self.color = c;
        self
    }

    /// Switch from filled to 1 px stroke outline.
    pub fn stroke(mut self) -> Self {
        self.filled = false;
        self
    }

    /// Draw the rectangle into `bounds` on the given draw target.
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        if self.filled {
            let style = PrimitiveStyle::with_fill(self.color);
            bounds.into_styled(style).draw(target)
        } else {
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(self.color)
                .stroke_width(1)
                .build();
            bounds.into_styled(style).draw(target)
        }
    }
}
