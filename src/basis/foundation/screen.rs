//! Display-agnostic screen context for percentage-based sizing.
//!
//! All UI code should use [`Screen`] to resolve sizes instead of
//! hardcoding pixel values. This keeps layouts portable across
//! different display resolutions.
//!
//! # Example
//!
//! ```rust,ignore
//! let screen = Screen::new(368, 448);
//!
//! // 50% of width, clamped to [40..200] px
//! let icon_w = screen.w(50, 40, 200);
//!
//! // 10% of height, no clamp
//! let header_h = screen.hp(10);
//!
//! // Sub-region: x=5%, y=10%, w=90%, h=75%
//! let content = screen.region(5, 10, 90, 75);
//! ```

use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::geometry::Point;

/// Display-agnostic screen dimensions.
///
/// Created once at init from the actual display size, then passed
/// to all UI code. Never store raw pixel constants — always resolve
/// through `Screen`.
#[derive(Debug, Clone, Copy)]
pub struct Screen {
    /// Display width in pixels.
    pub width: u32,
    /// Display height in pixels.
    pub height: u32,
}

impl Screen {
    /// Create a screen context from pixel dimensions.
    ///
    /// Typically called once in `board::init()` or after display setup:
    /// ```rust,ignore
    /// let size = display.size();
    /// let screen = Screen::new(size.width, size.height);
    /// ```
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// The full screen as a [`Rectangle`] at origin (0, 0).
    pub const fn bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(self.width, self.height))
    }

    // ── Percentage → pixel resolution ───────────────────────────────

    /// Resolve a percentage of **width** to pixels, clamped to `[min..=max]`.
    ///
    /// `screen.w(50, 20, 200)` on a 368px-wide display → `clamp(184, 20, 200)` = 184.
    pub const fn w(&self, percent: u32, min: u32, max: u32) -> u32 {
        clamp(pct(self.width, percent), min, max)
    }

    /// Resolve a percentage of **height** to pixels, clamped to `[min..=max]`.
    pub const fn h(&self, percent: u32, min: u32, max: u32) -> u32 {
        clamp(pct(self.height, percent), min, max)
    }

    /// Percentage of width, no clamp.
    pub const fn wp(&self, percent: u32) -> u32 {
        pct(self.width, percent)
    }

    /// Percentage of height, no clamp.
    pub const fn hp(&self, percent: u32) -> u32 {
        pct(self.height, percent)
    }

    // ── Sub-regions ─────────────────────────────────────────────────

    /// Carve out a sub-region of the screen, all values in percentages.
    ///
    /// `screen.region(5, 10, 90, 75)` → a rectangle starting at (5%, 10%)
    /// with size (90%, 75%) of the screen.
    pub const fn region(&self, x_pct: u32, y_pct: u32, w_pct: u32, h_pct: u32) -> Rectangle {
        Rectangle::new(
            Point::new(pct(self.width, x_pct) as i32, pct(self.height, y_pct) as i32),
            Size::new(pct(self.width, w_pct), pct(self.height, h_pct)),
        )
    }

    /// Carve out a sub-region with clamped dimensions.
    ///
    /// Position is percentage-based (no clamp), size is clamped.
    pub const fn region_clamped(
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
        Rectangle::new(
            Point::new(pct(self.width, x_pct) as i32, pct(self.height, y_pct) as i32),
            Size::new(
                clamp(pct(self.width, w_pct), min_w, max_w),
                clamp(pct(self.height, h_pct), min_h, max_h),
            ),
        )
    }
}

// ── Helpers (const-compatible) ──────────────────────────────────────

/// Integer percentage: `pct(368, 50)` → 184.
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
