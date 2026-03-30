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

/// Controls how children are positioned along the **main axis**.
///
/// The main axis is horizontal for [`HStack`] and vertical for [`VStack`].
///
/// # How it works
///
/// The layout engine first sizes each child:
/// - Children with **intrinsic sizes** (e.g. [`Label`](crate::widgets::Label))
///   get their measured main-axis extent.
/// - Children **without** intrinsic sizes (e.g. [`Rect`](crate::widgets::Rect))
///   divide the remaining space proportionally by flex weight.
///
/// If the children don't consume all available main-axis space,
/// `Justify` controls where the block of children is placed within
/// the leftover space:
///
/// ```text
/// HStack width = 200px, children consume 60px total
///
/// Justify::Start:   |AAA BBB CCC                        |
/// Justify::Center:  |           AAA BBB CCC             |
/// Justify::End:     |                        AAA BBB CCC|
/// ```
///
/// **Note:** If all children are flex (no intrinsic size), they consume
/// 100% of the space and `Justify` has no visible effect. Use children
/// with intrinsic sizes (like [`Label`](crate::widgets::Label)) or
/// [`spacer()`](Stack::spacer) to create leftover space.
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

/// Controls how each child is positioned on the **cross axis**.
///
/// The cross axis is vertical for [`HStack`] and horizontal for [`VStack`].
///
/// # How it works
///
/// `Align` determines where each child sits within the cross-axis extent:
///
/// - Children with **intrinsic sizes** (e.g. [`Label`](crate::widgets::Label))
///   are positioned according to the `Align` mode — their bounds are
///   shrunk to their measured cross-axis size and offset accordingly.
/// - Children **without** intrinsic sizes (e.g. [`Rect`](crate::widgets::Rect))
///   always stretch to fill the full cross-axis extent, regardless of
///   the `Align` setting.
///
/// ```text
/// HStack height = 60px, Label height = 15px
///
/// Align::Stretch:  |Label fills 60px|  (default)
/// Align::Start:    |Label|           |  (at top)
/// Align::Center:   |     |Label|     |  (centered)
/// Align::End:      |           |Label|  (at bottom)
/// ```
///
/// **Tip:** Place a [`Rect`](crate::widgets::Rect) alongside labels in
/// the same stack to visualize the full cross-axis extent — the Rect
/// stretches while labels are positioned by `Align`.
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
/// - [`Horizontal`] — children flow left to right  (`HStack`)
/// - [`Vertical`]   — children flow top to bottom (`VStack`)
///
/// # Sizing model
///
/// Children are sized in two passes:
/// 1. **Intrinsic children** (those whose [`Element::measure()`] returns
///    `Some(size)`, e.g. [`Label`](crate::widgets::Label)) get their
///    measured main-axis extent — they take only the space they need.
/// 2. **Flex children** (those without intrinsic size, e.g.
///    [`Rect`](crate::widgets::Rect)) divide the **remaining** space
///    proportionally by their flex weight (default 1).
///
/// # Layout controls
///
/// | Method                          | Axis  | Effect                                    |
/// |---------------------------------|-------|-------------------------------------------|
/// | [`justify`](Stack::justify)     | Main  | Position the block of children (Start/Center/End) |
/// | [`align`](Stack::align)         | Cross | Position each intrinsic child (Start/Center/End/Stretch) |
/// | [`padding`](Stack::padding)     | Both  | Inset from bounds on all sides            |
/// | [`gap`](Stack::gap)             | Main  | Fixed pixel spacing between children      |
/// | [`child_flex`](Stack::child_flex) | Main | Set a child's flex weight                |
/// | [`spacer`](Stack::spacer)       | Main  | Invisible flex child (creates empty space)|
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
    pub fn child(mut self, element: impl Into<Element<'a>>) -> Self {
        self.children.push(FlexChild {
            element: element.into(),
            flex: 1,
        });
        self
    }

    /// Append a child with the given flex weight.
    pub fn child_flex(mut self, element: impl Into<Element<'a>>, flex: u32) -> Self {
        self.children.push(FlexChild {
            element: element.into(),
            flex,
        });
        self
    }

    /// Append an invisible flex child with the given flex weight.
    pub fn spacer(self, flex: u32) -> Self {
        self.child_flex(Element::Empty, flex)
    }

    /// Append a single invisible flex child with the given flex weight.
    pub fn space(self) -> Self {
        self.spacer(1)
    }

    /// Paint using the bounds set by [`within`](Stack::within).
    ///
    /// Panics if no bounds were set. For parent-supplied bounds, use
    /// [`paint_into`](Stack::paint_into).
    pub fn paint<T>(&self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = GraphicsColorMode>,
    {
        let bounds = self
            .bounds
            .expect("Stack::paint() requires bounds set via within()");
        self.paint_into(bounds, target)
    }

    /// Paint all children into the given `bounds`.
    ///
    /// Use this when a parent layout supplies the bounds at paint time
    /// (e.g. a Stack nested inside another Stack).
    ///
    /// Layout algorithm:
    /// 1. Children with intrinsic sizes (via `measure()`) get their
    ///    measured main-axis extent.
    /// 2. Remaining space is divided among flex children proportionally.
    /// 3. The resulting block is positioned by [`Justify`].
    /// 4. Each child is positioned on the cross axis by [`Align`].
    pub fn paint_into<T>(&self, bounds: Rectangle, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = GraphicsColorMode>,
    {
        let n = self.children.len();
        if n == 0 {
            return Ok(());
        }

        let inner = inset(&bounds, self.padding);
        let main_total = D::main_axis_size(inner.size);
        let cross_total = D::cross_axis_size(inner.size);

        let total_gap = self.gap * (n as u32).saturating_sub(1);
        let main_for_children = main_total.saturating_sub(total_gap);

        // Pass 1: measure intrinsic children, sum flex weights for the rest.
        let mut intrinsic_used = 0u32;
        let mut total_flex = 0u32;
        let mut measured: Vec<Option<u32>> = Vec::with_capacity(n);
        for child in &self.children {
            if let Some(size) = child.element.measure() {
                let m = D::main_axis_size(size).min(main_for_children);
                measured.push(Some(m));
                intrinsic_used += m;
            } else {
                measured.push(None);
                total_flex += child.flex;
            }
        }

        // Pass 2: compute final main-axis size for each child.
        let flex_pool = main_for_children.saturating_sub(intrinsic_used);
        let mut sizes: Vec<u32> = Vec::with_capacity(n);
        let mut flex_allocated = 0u32;
        let mut flex_count = 0u32;
        let flex_children_count = self
            .children
            .iter()
            .zip(measured.iter())
            .filter(|(_, m)| m.is_none())
            .count() as u32;

        for (i, child) in self.children.iter().enumerate() {
            let size = if let Some(m) = measured[i] {
                m
            } else if total_flex == 0 {
                0
            } else {
                flex_count += 1;
                if flex_count == flex_children_count {
                    // Last flex child gets remainder to avoid rounding gaps.
                    flex_pool.saturating_sub(flex_allocated)
                } else {
                    let s = flex_pool * child.flex / total_flex;
                    flex_allocated += s;
                    s
                }
            };
            sizes.push(size);
        }

        let consumed: u32 = sizes.iter().sum::<u32>() + total_gap;

        let base_offset = D::initial_offset(&inner);
        let start_offset = match self.justify {
            Justify::Start => base_offset,
            Justify::End => base_offset + (main_total.saturating_sub(consumed)) as i32,
            Justify::Center => base_offset + (main_total.saturating_sub(consumed)) as i32 / 2,
        };

        // Pass 3: paint each child.
        let mut offset = start_offset;
        for (i, child) in self.children.iter().enumerate() {
            let child_main = sizes[i];

            // Cross-axis positioning based on Align mode and intrinsic size.
            let (cross_offset, cross_size) = match self.align {
                Align::Stretch => (0i32, cross_total),
                other => {
                    if let Some(intrinsic) = child.element.measure() {
                        let child_cross = D::cross_axis_size(intrinsic).min(cross_total);
                        match other {
                            Align::Start => (0i32, child_cross),
                            Align::Center => (
                                (cross_total.saturating_sub(child_cross)) as i32 / 2,
                                child_cross,
                            ),
                            Align::End => (
                                (cross_total.saturating_sub(child_cross)) as i32,
                                child_cross,
                            ),
                            Align::Stretch => unreachable!(),
                        }
                    } else {
                        // No intrinsic size → stretch to fill.
                        (0i32, cross_total)
                    }
                }
            };

            let child_bounds =
                D::child_bounds(&inner, offset, child_main, cross_offset, cross_size);
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
