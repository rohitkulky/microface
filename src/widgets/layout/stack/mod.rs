mod stack_direction;

use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use embedded_graphics::geometry::Size;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;

use crate::color::GraphicsColorMode;
use crate::element::Element;

pub use stack_direction::{Horizontal, StackDirection, Vertical};

// ── Justify (main axis) ────────────────────────────────────────────

/// Controls how children are distributed along the main axis.
///
/// Works with flex weights: the total available space (after padding
/// and gaps) is divided among children proportionally to their flex
/// values, then the resulting block is positioned according to the
/// justify mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Justify {
    /// Pack children at the start of the main axis (default).
    Start,
    /// Pack children at the end of the main axis.
    End,
    /// Center children along the main axis.
    Center,
}

// ── Align (cross axis) ─────────────────────────────────────────────

/// Controls how each child is positioned on the cross axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    /// Fill the entire cross-axis extent (default).
    Stretch,
    /// Align to the start of the cross axis.
    Start,
    /// Align to the end of the cross axis.
    End,
    /// Center on the cross axis.
    Center,
}

// ── Stack ───────────────────────────────────────────────────────────

struct FlexChild<'a> {
    element: Element<'a>,
    flex: u32,
}

/// A flex-weighted stack layout along a main axis.
///
/// The type parameter `D` selects the direction:
/// - [`Horizontal`] — children flow left to right
/// - [`Vertical`]   — children flow top to bottom
///
/// Each child receives a share of the main-axis length proportional to
/// its flex weight (default 1). Use [`justify`](Stack::justify),
/// [`align`](Stack::align), [`padding`](Stack::padding), and
/// [`gap`](Stack::gap) to control distribution and spacing.
pub struct Stack<'a, D: StackDirection> {
    children: Vec<FlexChild<'a>>,
    _direction: PhantomData<D>,
    bounds: Option<Rectangle>,
    justify: Justify,
    align: Align,
    padding: u32,
    gap: u32,
}

impl<'a, D: StackDirection> Stack<'a, D> {
    /// Create an empty stack with no predefined bounds.
    ///
    /// Use [`paint_into`](Stack::paint_into) to supply bounds at paint
    /// time, or call [`within`](Stack::within) to set bounds up front.
    pub fn new() -> Self {
        Stack {
            children: vec![],
            _direction: PhantomData,
            bounds: None,
            justify: Justify::Start,
            align: Align::Stretch,
            padding: 0,
            gap: 0,
        }
    }

    /// Create an empty stack within the given bounding rectangle.
    pub fn within(bounds: Rectangle) -> Self {
        Stack {
            children: vec![],
            _direction: PhantomData,
            bounds: Some(bounds),
            justify: Justify::Start,
            align: Align::Stretch,
            padding: 0,
            gap: 0,
        }
    }

    /// Set main-axis distribution mode.
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// Set cross-axis alignment mode.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Set uniform padding (inset from bounds on all sides).
    pub fn padding(mut self, px: u32) -> Self {
        self.padding = px;
        self
    }

    /// Set fixed pixel gap between children.
    pub fn gap(mut self, px: u32) -> Self {
        self.gap = px;
        self
    }

    /// Append a child with a flex weight of 1.
    pub fn child(mut self, element: Element<'a>) -> Self {
        self.children.push(FlexChild { element, flex: 1 });
        self
    }

    /// Append a child with the given flex weight.
    pub fn child_flex(mut self, element: Element<'a>, flex: u32) -> Self {
        self.children.push(FlexChild { element, flex });
        self
    }

    /// Paint using the bounds set by [`within`](Stack::within).
    ///
    /// Panics if no bounds were set. For parent-supplied bounds, use
    /// [`paint_into`](Stack::paint_into).
    pub fn paint<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = GraphicsColorMode>,
    {
        let bounds = self.bounds.expect("Stack::paint() requires bounds set via within()");
        self.paint_into(bounds, target)
    }

    /// Paint all children into the given `bounds`.
    ///
    /// Use this when a parent layout supplies the bounds at paint time
    /// (e.g. a Stack nested inside another Stack).
    pub fn paint_into<T>(&self, bounds: Rectangle, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = GraphicsColorMode>,
    {
        let n = self.children.len();
        if n == 0 {
            return Ok(());
        }

        let total_flex: u32 = self.children.iter().map(|c| c.flex).sum();
        if total_flex == 0 {
            return Ok(());
        }

        let inner = inset(&bounds, self.padding);
        let main_total = D::main_axis_size(inner.size);
        let cross_total = D::cross_axis_size(inner.size);

        let total_gap = self.gap * (n as u32).saturating_sub(1);
        let main_for_children = main_total.saturating_sub(total_gap);

        let mut sizes: Vec<u32> = Vec::with_capacity(n);
        let mut allocated = 0u32;
        for (i, child) in self.children.iter().enumerate() {
            let size = if i == n - 1 {
                main_for_children.saturating_sub(allocated)
            } else {
                main_for_children * child.flex / total_flex
            };
            sizes.push(size);
            allocated += size;
        }

        let consumed = allocated + total_gap;

        let base_offset = D::initial_offset(&inner);
        let start_offset = match self.justify {
            Justify::Start => base_offset,
            Justify::End => base_offset + (main_total.saturating_sub(consumed)) as i32,
            Justify::Center => base_offset + (main_total.saturating_sub(consumed)) as i32 / 2,
        };

        // Cross-axis: all modes give full extent until Component trait
        // provides measure(). Enum variants defined for forward compat.
        let (cross_offset, cross_size) = (0i32, cross_total);

        let mut offset = start_offset;
        for (i, child) in self.children.iter().enumerate() {
            let child_main = sizes[i];
            let child_bounds = D::child_bounds(&inner, offset, child_main, cross_offset, cross_size);
            child.element.paint(child_bounds, target)?;
            offset += child_main as i32 + self.gap as i32;
        }

        Ok(())
    }
}

/// Horizontal stack — lays out children left to right.
pub type HStack<'a> = Stack<'a, Horizontal>;

/// Vertical stack — lays out children top to bottom.
pub type VStack<'a> = Stack<'a, Vertical>;

// ── Helpers ─────────────────────────────────────────────────────────

/// Shrink a rectangle by `px` on all sides.
fn inset(rect: &Rectangle, px: u32) -> Rectangle {
    let double = px * 2;
    Rectangle::new(
        embedded_graphics::geometry::Point::new(
            rect.top_left.x + px as i32,
            rect.top_left.y + px as i32,
        ),
        Size::new(
            rect.size.width.saturating_sub(double),
            rect.size.height.saturating_sub(double),
        ),
    )
}
