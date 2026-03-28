use embedded_graphics::prelude::{DrawTarget, Primitive};
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::Drawable;

use crate::color::{GraphicsColorMode, FG};

pub struct Rect {
    color: GraphicsColorMode,
    filled: bool,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            color: FG,
            filled: true,
        }
    }

    pub fn color(mut self, c: GraphicsColorMode) -> Self {
        self.color = c;
        self
    }

    pub fn stroke(mut self) -> Self {
        self.filled = false;
        self
    }

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
