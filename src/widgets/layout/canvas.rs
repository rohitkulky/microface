//! Canvas — percentage-based layout helpers for any display.
//!
//! Extends any [`OriginDimensions`] implementor (including all
//! `embedded-graphics` display types) with percentage-based sizing
//! and sub-region methods via a blanket implementation.
//!
//! # Example
//!
//! ```rust,ignore
//! use microface::widgets::layout::Canvas;
//!
//! let top_half = display.region(0, 0, 100, 50);
//! let icon_w   = display.wp(25);
//! ```

use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::primitives::Rectangle;

/// Percentage-based layout helpers for any display.
///
/// Blanket-implemented for every [`OriginDimensions`] type.
pub trait Canvas: OriginDimensions {
    /// The full display as a [`Rectangle`] at origin (0, 0).
    fn canvas_bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), self.size())
    }

    /// Resolve a percentage of **width** to pixels.
    fn wp(&self, percent: u32) -> u32 {
        pct(self.size().width, percent)
    }

    /// Resolve a percentage of **height** to pixels.
    fn hp(&self, percent: u32) -> u32 {
        pct(self.size().height, percent)
    }

    /// Resolve a percentage of **width** to pixels, clamped to `[min..=max]`.
    fn w(&self, percent: u32, min: u32, max: u32) -> u32 {
        clamp(pct(self.size().width, percent), min, max)
    }

    /// Resolve a percentage of **height** to pixels, clamped to `[min..=max]`.
    fn h(&self, percent: u32, min: u32, max: u32) -> u32 {
        clamp(pct(self.size().height, percent), min, max)
    }

    /// Carve out a sub-region, all values in percentages.
    ///
    /// `display.region(5, 10, 90, 75)` → a rectangle starting at (5%, 10%)
    /// with size (90%, 75%) of the display.
    fn region(&self, x_pct: u32, y_pct: u32, w_pct: u32, h_pct: u32) -> Rectangle {
        let s = self.size();
        Rectangle::new(
            Point::new(pct(s.width, x_pct) as i32, pct(s.height, y_pct) as i32),
            Size::new(pct(s.width, w_pct), pct(s.height, h_pct)),
        )
    }

    /// A shortcut for `region(0, 0, 100, 100)`.
    fn full(&self) -> Rectangle {
        self.region( 0, 0, 100, 100)
    }

    /// A shortcut for full row from Y % to Y % 
    fn full_row(&self, y_from: u32, y_to: u32) -> Rectangle {
        self.region(0, y_from, 100, y_to - y_from)
    }

    /// A shortcut for full column from X % to X %
    fn full_col(&self, x_from: u32, x_to: u32) -> Rectangle {
        self.region(x_from, 0, x_to - x_from, 100)
    }

    /// Carve out a sub-region with clamped dimensions.
    ///
    /// Position is percentage-based (no clamp), size is clamped.
    fn region_clamped(
        &self,
        x_pct: u32,
        y_pct: u32,
        w_pct: u32,
        h_pct: u32,
        min_w: u32,
        max_w: u32,
        min_h: u32,
        max_h: u32,
    ) -> Rectangle {
        let s = self.size();
        Rectangle::new(
            Point::new(pct(s.width, x_pct) as i32, pct(s.height, y_pct) as i32),
            Size::new(
                clamp(pct(s.width, w_pct), min_w, max_w),
                clamp(pct(s.height, h_pct), min_h, max_h),
            ),
        )
    }
}

impl<T: OriginDimensions> Canvas for T {}

// ── Helpers ─────────────────────────────────────────────────────────

/// Integer percentage: `pct(300, 50)` → 150.
const fn pct(total: u32, percent: u32) -> u32 {
    (total * percent) / 100
}

/// Const-compatible clamp.
const fn clamp(value: u32, min: u32, max: u32) -> u32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
