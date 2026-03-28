//! # Microface
//!
//! A simplified, hardware-agnostic interface for embedded graphics in Rust.
//!
//! Built on top of [`embedded-graphics`] and [`embedded_layout`], microface
//! provides a friendlier API for drawing and laying out UI elements on
//! any display.
//!
//! ## Design principles
//!
//! 1. **No hardcoded pixels** — use `Screen` for percentage-based sizing
//! 2. **Hardware-agnostic** — works with any `DrawTarget` implementation
//! 3. **Configurable** — swap display driver + Screen config to port

pub mod basis;
pub mod color;
pub mod element;
pub mod fonts;
pub mod render;
pub mod widgets;

pub use basis::foundation::screen::Screen;

/// Re-export the `include_font!` proc macro for compile-time font rasterization.
///
/// # Example
///
/// ```ignore
/// use microface::{include_font, fonts::MicroFont};
///
/// const MY_FONT: MicroFont = include_font!("fonts/myfont.ttf", size = 24, bpp = 4);
/// ```
pub use microface_macros::include_font;
