//! UI foundation — layout, sizing, and drawing primitives.
//!
//! This module provides the building blocks for all UI screens:
//!
//! - [`screen::Screen`] — display-agnostic percentage-based sizing
//! - [`layout`] — horizontal/vertical stacks, spacing, alignment
//! - [`primitives`] — low-level drawing helpers
//!
//! # Design principles
//!
//! 1. **No hardcoded pixels** — use `Screen` to resolve percentages
//! 2. **Nest stacks** — `hz_stack` inside `vt_stack` for any layout
//! 3. **Portable** — same UI code works on any display resolution

pub mod layout;
pub mod screen;
