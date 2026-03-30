use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::primitives::Rectangle;

/// Trait abstracting the axis along which a [`Stack`](super::Stack) lays out children.
pub trait StackDirection {
    /// Main-axis length of the given size.
    fn main_axis_size(size: Size) -> u32;

    /// Cross-axis length of the given size.
    fn cross_axis_size(size: Size) -> u32;

    /// Starting offset along the main axis from the parent bounds.
    fn initial_offset(bounds: &Rectangle) -> i32;

    /// Compute a child's bounding rectangle from the parent bounds,
    /// the current main-axis offset, the child's main-axis extent,
    /// and the child's cross-axis offset and extent.
    fn child_bounds(
        parent: &Rectangle,
        offset: i32,
        main_size: u32,
        cross_offset: i32,
        cross_size: u32,
    ) -> Rectangle;
}

/// Left-to-right direction marker.
pub struct Horizontal;

/// Top-to-bottom direction marker.
pub struct Vertical;

impl StackDirection for Horizontal {
    fn main_axis_size(size: Size) -> u32 {
        size.width
    }

    fn cross_axis_size(size: Size) -> u32 {
        size.height
    }

    fn initial_offset(bounds: &Rectangle) -> i32 {
        bounds.top_left.x
    }

    fn child_bounds(
        parent: &Rectangle,
        offset: i32,
        main_size: u32,
        cross_offset: i32,
        cross_size: u32,
    ) -> Rectangle {
        Rectangle::new(
            Point::new(offset, parent.top_left.y + cross_offset),
            Size::new(main_size, cross_size),
        )
    }
}

impl StackDirection for Vertical {
    fn main_axis_size(size: Size) -> u32 {
        size.height
    }

    fn cross_axis_size(size: Size) -> u32 {
        size.width
    }

    fn initial_offset(bounds: &Rectangle) -> i32 {
        bounds.top_left.y
    }

    fn child_bounds(
        parent: &Rectangle,
        offset: i32,
        main_size: u32,
        cross_offset: i32,
        cross_size: u32,
    ) -> Rectangle {
        Rectangle::new(
            Point::new(parent.top_left.x + cross_offset, offset),
            Size::new(cross_size, main_size),
        )
    }
}
