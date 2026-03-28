//! Layout helpers — thin wrappers around [`embedded_layout`].
//!
//! Provides ergonomic constructors for horizontal and vertical stacks,
//! spacing presets, and re-exported alignment types so UI code doesn't
//! need to reach into `embedded_layout` directly.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use crate::ui::foundation::layout::{hz_stack, vt_stack, gap, even, tight};
//! use crate::ui::foundation::layout::align::{horizontal, vertical};
//! use crate::ui::foundation::screen::Screen;
//! use embedded_layout::prelude::{Chain, Views, Align};
//!
//! let screen = Screen::new(368, 448);
//!
//! // Horizontal row with 8px gaps, vertically centered
//! let row = hz_stack(
//!     Chain::new(icon).append(label).append(value),
//! )
//! .with_spacing(gap(8))
//! .with_alignment(vertical::Center)
//! .arrange();
//!
//! // Vertical stack with even distribution across full height
//! let page = vt_stack(
//!     Chain::new(header).append(content).append(footer),
//! )
//! .with_spacing(even(screen.hp(100)))
//! .with_alignment(horizontal::Center)
//! .arrange()
//! .align_to(&screen.bounds(), horizontal::Center, vertical::Top);
//!
//! // Homogeneous list from a slice
//! let mut items = [rect1, rect2, rect3];
//! let list = vt_stack(Views::new(&mut items))
//!     .with_spacing(gap(4))
//!     .arrange();
//!
//! // Nesting: rows inside a column
//! let nested = vt_stack(
//!     Chain::new(
//!         hz_stack(Chain::new(a).append(b)).arrange()
//!     ).append(
//!         hz_stack(Chain::new(c).append(d)).arrange()
//!     ),
//! )
//! .with_spacing(gap(12))
//! .arrange();
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

/// Create a horizontal stack (left → right).
///
/// Children are placed side by side. Use `.with_spacing()` to control
/// gaps and `.with_alignment()` to set vertical alignment of children.
///
/// ```rust,ignore
/// let row = hz_stack(Chain::new(a).append(b).append(c))
///     .with_spacing(gap(8))
///     .with_alignment(vertical::Center)
///     .arrange();
/// ```
pub fn hz_stack<VG>(views: VG) -> LinearLayout<Horizontal<embedded_layout::align::vertical::Bottom, Tight>, VG>
where
    VG: ViewGroup,
{
    LinearLayout::horizontal(views)
}

/// Create a vertical stack (top → bottom).
///
/// Children are stacked vertically. Use `.with_spacing()` to control
/// gaps and `.with_alignment()` to set horizontal alignment of children.
///
/// ```rust,ignore
/// let col = vt_stack(Chain::new(a).append(b).append(c))
///     .with_spacing(gap(4))
///     .with_alignment(horizontal::Center)
///     .arrange();
/// ```
pub fn vt_stack<VG>(views: VG) -> LinearLayout<Vertical<embedded_layout::align::horizontal::Left, Tight>, VG>
where
    VG: ViewGroup,
{
    LinearLayout::vertical(views)
}

// ── Spacing presets ─────────────────────────────────────────────────

/// No space between children.
///
/// ```rust,ignore
/// hz_stack(views).with_spacing(tight()).arrange();
/// ```
pub const fn tight() -> Tight {
    Tight
}

/// Fixed pixel gap between children.
///
/// Use with [`Screen`](super::screen::Screen) for relative sizing:
/// ```rust,ignore
/// let spacing = gap(screen.hp(2) as i32);  // 2% of height
/// vt_stack(views).with_spacing(spacing).arrange();
/// ```
pub const fn gap(px: i32) -> FixedMargin {
    FixedMargin(px)
}

/// Distribute children evenly across `total_px` pixels.
///
/// This is equivalent to CSS `justify-content: space-between` —
/// first child at the start, last child at the end, equal gaps
/// between.
///
/// Use with [`Screen`](super::screen::Screen) for relative sizing:
/// ```rust,ignore
/// // Distribute across full screen width
/// hz_stack(views)
///     .with_spacing(even(screen.wp(100)))
///     .arrange();
///
/// // Distribute across 80% of width
/// hz_stack(views)
///     .with_spacing(even(screen.wp(80)))
///     .arrange();
/// ```
pub const fn even(total_px: u32) -> DistributeFill {
    DistributeFill(total_px)
}
