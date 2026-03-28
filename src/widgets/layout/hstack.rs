use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;

use crate::color::GraphicsColorMode;
use crate::element::Element;

/// A child in an HStack with a flex weight.
struct FlexChild<'a> {
    element: Element<'a>,
    flex: u32,
}

/// Horizontal stack — lays out children left to right.
///
/// Each child gets a width proportional to its flex value.
/// Default flex is 1 (equal distribution).
///
/// Usage:
/// ```ignore
/// HStack::new()
///     .child(Rect::new().into())              // flex 1
///     .child_flex(Label::new("Hi", &f).into(), 2)  // flex 2 (twice as wide)
///     .child(Rect::new().into())              // flex 1
///     .paint(screen.bounds(), &mut display)?;
/// ```
pub struct HStack<'a> {
    children: Vec<FlexChild<'a>>,
}

impl<'a> HStack<'a> {
    /// Create an empty horizontal stack.
    pub fn new() -> Self {
        HStack { children: (vec![]) }
    }

    /// Add a child with default flex weight of 1.
    pub fn child(mut self, element: Element<'a>) -> Self {
        self.children.push(FlexChild { element, flex: 1 });
        self
    }

    /// Add a child with a specific flex weight.
    pub fn child_flex(mut self, element: Element<'a>, flex: u32) -> Self {
        self.children.push(FlexChild { element, flex });
        self
    }

    /// Draw all children left-to-right within the given bounds.
    ///
    /// The math:
    ///   total_flex = sum of all children's flex values
    ///   for each child:
    ///     child_width = bounds.width * child.flex / total_flex
    ///     child_bounds = Rectangle at (x_offset, bounds.y) with (child_width, bounds.height)
    ///     x_offset += child_width
    pub fn paint<D>(&self, bounds: Rectangle, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = GraphicsColorMode>,
    {
        // Step 1: Calculate total flex
        // Hint: self.children.iter().map(|c| c.flex).sum::<u32>()
        let total_flex: u32 = self.children.iter().map(|c| c.flex).sum::<u32>();

        // Guard: if no children or zero flex, nothing to draw
        if total_flex == 0 {
            return Ok(());
        }

        // Step 2: Walk through children, computing bounds for each
        let mut x_offset = bounds.top_left.x;

        for child in &self.children {
            // Step 3: Calculate this child's width
            // Hint: bounds.size.width * child.flex / total_flex
            let child_width = bounds.size.width * child.flex / total_flex;

            // Step 4: Create the child's bounding rectangle
            // Hint: Rectangle::new(Point::new(x_offset, bounds.top_left.y), Size::new(...))
            let child_bounds = Rectangle::new(
                Point::new(x_offset, bounds.top_left.y),
                Size::new(child_width, bounds.size.height),
            );

            // Step 5: Paint the child into its bounds
            child.element.paint(child_bounds, target)?;

            // Step 6: Advance x_offset for the next child
            x_offset += child_width as i32;
        }

        Ok(())
    }
}
