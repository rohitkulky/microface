//! Layout helpers — thin wrappers around [`embedded_layout`].
//!
//! Provides ergonomic constructors for horizontal and vertical stacks,
//! spacing presets, and re-exported alignment types so UI code doesn't
//! need to reach into `embedded_layout` directly.
//!
//! # Example
//!
//! ```rust,ignore
//! use microface::basis::foundation::layout::{hz_stack, vt_stack, gap, even, tight};
//! use microface::basis::foundation::layout::align::{horizontal, vertical};
//! use microface::Canvas;
//! use embedded_layout::prelude::{Chain, Views, Align};
//!
//! // Horizontal row with 8 px gaps, vertically centered
//! let row = hz_stack(Chain::new(icon).append(label).append(value))
//!     .with_spacing(gap(8))
//!     .with_alignment(vertical::Center)
//!     .arrange();
//!
//! // Vertical stack distributed across full display height
//! let page = vt_stack(Chain::new(header).append(content).append(footer))
//!     .with_spacing(even(display.hp(100)))
//!     .with_alignment(horizontal::Center)
//!     .arrange()
//!     .align_to(&display.canvas_bounds(), horizontal::Center, vertical::Top);
//!
//! // Homogeneous list from a slice
//! let mut items = [rect1, rect2, rect3];
//! let list = vt_stack(Views::new(&mut items))
//!     .with_spacing(gap(4))
//!     .arrange();
//! ```

use embedded_layout::layout::linear::LinearLayout;
use embedded_layout::layout::linear::spacing::{DistributeFill, FixedMargin, Tight};
use embedded_layout::layout::linear::{Horizontal, Vertical};
use embedded_layout::view_group::ViewGroup;

// ── Re-exports for clean imports ────────────────────────────────────

/// Alignment types — use `align::horizontal::*` and `align::vertical::*`.
pub mod align {
    pub use embedded_layout::align::horizontal;
    pub use embedded_layout::align::vertical;
    pub use embedded_layout::align::Align;
}

/// View grouping — `Chain` for heterogeneous, `Views` for homogeneous.
pub mod group {
    pub use embedded_layout::object_chain::Chain;
    pub use embedded_layout::view_group::Views;
}

// ── Stack constructors ──────────────────────────────────────────────

/// Create a horizontal (left → right) linear layout.
pub fn hz_stack<VG>(views: VG) -> LinearLayout<Horizontal<embedded_layout::align::vertical::Bottom, Tight>, VG>
where
    VG: ViewGroup,
{
    LinearLayout::horizontal(views)
}

/// Create a vertical (top → bottom) linear layout.
pub fn vt_stack<VG>(views: VG) -> LinearLayout<Vertical<embedded_layout::align::horizontal::Left, Tight>, VG>
where
    VG: ViewGroup,
{
    LinearLayout::vertical(views)
}

// ── Spacing presets ─────────────────────────────────────────────────

/// No space between children.
pub const fn tight() -> Tight {
    Tight
}

/// Fixed pixel gap between children.
pub const fn gap(px: i32) -> FixedMargin {
    FixedMargin(px)
}

/// Distribute children evenly across `total_px` pixels.
///
/// First child at the start, last child at the end, equal gaps between.
pub const fn even(total_px: u32) -> DistributeFill {
    DistributeFill(total_px)
}
