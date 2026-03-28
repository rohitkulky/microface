/// A place for all exposed elements

use embedded_graphics::primitives::Rectangle;
use embedded_graphics_core::draw_target::DrawTarget;
use crate::color::GraphicsColorMode;
use crate::widgets::primitives::Rect;
use crate::widgets::text::Label;

pub enum Element<'a> {
    Empty,
    Rect(Rect),
    Label(Label<'a>),
}

impl<'a> Element<'a> {
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        match self {
            Element::Empty => Ok(()),
            Element::Rect(r) => r.paint(bounds, target),
            Element::Label(l) => l.paint(bounds, target),
        }
    }
}
