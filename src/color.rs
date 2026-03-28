/// Decide what color system to use

use embedded_graphics::pixelcolor;

#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
use embedded_graphics::prelude::RgbColor;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub type GraphicsColorMode = pixelcolor::Rgb565;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub const FG: GraphicsColorMode = GraphicsColorMode::WHITE;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub const BG: GraphicsColorMode = GraphicsColorMode::BLACK;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub const ACCENT_PR: GraphicsColorMode = GraphicsColorMode::MAGENTA;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub const ACCENT_SC: GraphicsColorMode = GraphicsColorMode::YELLOW;
#[cfg(not(any(feature = "bw", feature = "grayscale", feature = "color")))]
pub const ACCENT_TR: GraphicsColorMode = GraphicsColorMode::GREEN;

#[cfg(feature = "bw")]
pub type GraphicsColorMode = pixelcolor::BinaryColor;
#[cfg(feature = "bw")]
pub const FG: GraphicsColorMode = GraphicsColorMode::On;
#[cfg(feature = "bw")]
pub const BG: GraphicsColorMode = GraphicsColorMode::Off;
#[cfg(feature = "bw")]
pub const ACCENT_PR: GraphicsColorMode = GraphicsColorMode::On;
#[cfg(feature = "bw")]
pub const ACCENT_SC: GraphicsColorMode = GraphicsColorMode::On;
#[cfg(feature = "bw")]
pub const ACCENT_TR: GraphicsColorMode = GraphicsColorMode::On;

#[cfg(feature = "grayscale")]
pub type GraphicsColorMode = pixelcolor::Gray8;
#[cfg(feature = "grayscale")]
pub const FG: GraphicsColorMode = GraphicsColorMode::new(255);
#[cfg(feature = "grayscale")]
pub const BG: GraphicsColorMode = GraphicsColorMode::new(0);
#[cfg(feature = "grayscale")]
pub const ACCENT_PR: GraphicsColorMode = GraphicsColorMode::new(200);
#[cfg(feature = "grayscale")]
pub const ACCENT_SC: GraphicsColorMode = GraphicsColorMode::new(150);
#[cfg(feature = "grayscale")]
pub const ACCENT_TR: GraphicsColorMode = GraphicsColorMode::new(100);

#[cfg(feature = "color")]
use embedded_graphics::prelude::RgbColor;
#[cfg(feature = "color")]
pub type GraphicsColorMode = pixelcolor::Rgb565;
#[cfg(feature = "color")]
pub const FG: GraphicsColorMode = GraphicsColorMode::WHITE;
#[cfg(feature = "color")]
pub const BG: GraphicsColorMode = GraphicsColorMode::BLACK;
#[cfg(feature = "color")]
pub const ACCENT_PR: GraphicsColorMode = GraphicsColorMode::MAGENTA;
#[cfg(feature = "color")]
pub const ACCENT_SC: GraphicsColorMode = GraphicsColorMode::YELLOW;
#[cfg(feature = "color")]
pub const ACCENT_TR: GraphicsColorMode = GraphicsColorMode::GREEN;
