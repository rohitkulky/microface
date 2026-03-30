//! Top-level element enum for polymorphic widget rendering.

use embedded_graphics::geometry::Size;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_core::draw_target::DrawTarget;
use crate::color::GraphicsColorMode;
use crate::widgets::text::Text;
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
    /// A text label.
    Text(Text<'a>),
}

impl<'a> Element<'a> {
    /// Return the intrinsic pixel size of this element, if known.
    ///
    /// Elements that always stretch to fill their bounds (e.g. [`Rect`])
    /// return `None`. Elements with a natural size (e.g. [`Label`])
    /// return `Some(size)`.
    pub fn measure(&self) -> Option<Size> {
        match self {
            Element::Empty => None,
            Element::Rect(_) => None,
            Element::Label(l) => Some(l.measure()),
            Element::HStack(_) => None,
            Element::VStack(_) => None,
            Element::Text(t) => Some(t.measure()),
        }
    }

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
            Element::Text(t) => t.paint(bounds, target),
        }
    }
}

// ── From impls ──────────────────────────────────────────────────────

impl From<Rect> for Element<'_> {
    fn from(r: Rect) -> Self { Element::Rect(r) }
}

impl<'a> From<Label<'a>> for Element<'a> {
    fn from(l: Label<'a>) -> Self { Element::Label(l) }
}

impl<'a> From<Text<'a>> for Element<'a> {
    fn from(t: Text<'a>) -> Self { Element::Text(t) }
}

impl<'a> From<HStack<'a>> for Element<'a> {
    fn from(s: HStack<'a>) -> Self { Element::HStack(s) }
}

impl<'a> From<VStack<'a>> for Element<'a> {
    fn from(s: VStack<'a>) -> Self { Element::VStack(s) }
}
