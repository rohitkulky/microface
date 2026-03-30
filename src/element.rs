//! Top-level element enum for polymorphic widget rendering.

use embedded_graphics::primitives::Rectangle;
use embedded_graphics_core::draw_target::DrawTarget;
use crate::color::GraphicsColorMode;
use crate::widgets::primitives::Rect;
use crate::widgets::text::Label;
use crate::widgets::layout::stack::{HStack, VStack};

/// A drawable UI element.
///
/// Wraps the concrete widget types so they can be stored and painted
/// uniformly by layout containers such as [`Stack`](crate::widgets::layout::Stack).
pub enum Element<'a> {
    /// No-op placeholder that draws nothing.
    Empty,
    /// A filled or stroked rectangle.
    Rect(Rect),
    /// A text label.
    Label(Label<'a>),
    /// A nested horizontal stack.
    HStack(HStack<'a>),
    /// A nested vertical stack.
    VStack(VStack<'a>),
}

impl<'a> Element<'a> {
    /// Draw this element into `bounds` on the given draw target.
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        match self {
            Element::Empty => Ok(()),
            Element::Rect(r) => r.paint(bounds, target),
            Element::Label(l) => l.paint(bounds, target),
            Element::HStack(s) => s.paint_into(bounds, target),
            Element::VStack(s) => s.paint_into(bounds, target),
        }
    }
}
