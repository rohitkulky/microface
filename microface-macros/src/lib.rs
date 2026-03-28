//! Compile-time font rasterization for microface.
//!
//! Provides the `include_font!` proc macro that reads a TTF/OTF font file,
//! rasterizes ASCII glyphs at the specified size and bit depth, and emits
//! a `MicroFont` const — all at compile time.
//!
//! Uses [`fontdue`] for font parsing and glyph rasterization.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{LitInt, LitStr, Token};

// ── Macro argument parsing ─────────────────────────────────────────────────

struct IncludeFontArgs {
    path: String,
    size: u32,
    bpp: u8,
}

impl Parse for IncludeFontArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut size = None;
        let mut bpp = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: LitInt = input.parse()?;

            match key.to_string().as_str() {
                "size" => size = Some(val.base10_parse()?),
                "bpp" => {
                    let v: u8 = val.base10_parse()?;
                    if !matches!(v, 1 | 2 | 4 | 8) {
                        return Err(syn::Error::new(val.span(), "bpp must be 1, 2, 4, or 8"));
                    }
                    bpp = Some(v);
                }
                _ => return Err(syn::Error::new(key.span(), "expected `size` or `bpp`")),
            }

            let _ = input.parse::<Token![,]>(); // optional trailing comma
        }

        Ok(IncludeFontArgs {
            path: path.value(),
            size: size.ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "missing `size`"))?,
            bpp: bpp.ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "missing `bpp`"))?,
        })
    }
}

// ── Per-glyph tight bounding box ───────────────────────────────────────────

/// Tight bounding box for a single glyph (offsets from cell origin).
struct GlyphBBox {
    x_off: u8,   // pixels from left edge of cell to first inked column
    y_off: u8,   // pixels from top edge of cell to first inked row
    tight_w: u8, // width of tight bounding box
    tight_h: u8, // height of tight bounding box
}

/// Compute tight bounding box for a glyph within its cell in the atlas bitmap.
fn compute_tight_bbox(bitmap: &[u8], cell_x: u32, cell_y: u32, cell_w: u32, cell_h: u32, strip_w: u32) -> GlyphBBox {
    let mut min_x = cell_w;
    let mut max_x = 0u32;
    let mut min_y = cell_h;
    let mut max_y = 0u32;

    for row in 0..cell_h {
        for col in 0..cell_w {
            let px = bitmap[((cell_y + row) * strip_w + cell_x + col) as usize];
            if px > 0 {
                if col < min_x { min_x = col; }
                if col > max_x { max_x = col; }
                if row < min_y { min_y = row; }
                if row > max_y { max_y = row; }
            }
        }
    }

    if min_x > max_x || min_y > max_y {
        // Empty glyph (e.g. space)
        GlyphBBox { x_off: 0, y_off: 0, tight_w: 0, tight_h: 0 }
    } else {
        GlyphBBox {
            x_off: min_x as u8,
            y_off: min_y as u8,
            tight_w: (max_x - min_x + 1) as u8,
            tight_h: (max_y - min_y + 1) as u8,
        }
    }
}

// ── Font rasterization (uses fontdue) ──────────────────────────────────────

struct RasterizedFont {
    packed: Vec<u8>,
    cell_width: u32,
    cell_height: u32,
    cols_per_row: u32,
    strip_width: u32,
    glyph_widths: Vec<u8>,
    glyph_bboxes: Vec<GlyphBBox>,
    kerning_pairs: Vec<(u8, u8, i8)>, // (left_idx, right_idx, adjustment)
    is_proportional: bool,
}

fn rasterize(font_data: &[u8], pixel_size: f32, bpp: u8) -> RasterizedFont {
    let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("include_font!: failed to parse font");

    let chars: Vec<char> = (32u8..=126).map(|c| c as char).collect();
    let cols_per_row = 16u32;

    // Font-level metrics for baseline alignment
    let lm = font.horizontal_line_metrics(pixel_size).expect("no line metrics");
    let ascent = lm.ascent.ceil() as i32;
    let descent = (-lm.descent).ceil() as i32;

    // Rasterize all glyphs
    let glyphs: Vec<_> = chars.iter().map(|&ch| font.rasterize(ch, pixel_size)).collect();

    // Cell dimensions
    let cell_width = glyphs.iter()
        .map(|(m, _)| (m.advance_width.ceil() as u32).max(m.width as u32))
        .max().unwrap_or(1).max(1);
    let cell_height = (ascent + descent).max(1) as u32;

    let num_rows = (chars.len() as u32 + cols_per_row - 1) / cols_per_row;
    let strip_width = cell_width * cols_per_row;
    let strip_height = cell_height * num_rows;

    // Compose glyphs into a grid bitmap
    let mut bitmap = vec![0u8; (strip_width * strip_height) as usize];
    let mut glyph_widths = Vec::with_capacity(chars.len());
    let (mut min_adv, mut max_adv) = (u32::MAX, 0u32);

    for (idx, (metrics, glyph_bitmap)) in glyphs.iter().enumerate() {
        let cell_x = (idx as u32 % cols_per_row) * cell_width;
        let cell_y = (idx as u32 / cols_per_row) * cell_height;
        let ox = metrics.xmin.max(0) as u32;
        let oy = (ascent - (metrics.ymin + metrics.height as i32)).max(0) as u32;

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let dx = cell_x + ox + col as u32;
                let dy = cell_y + oy + row as u32;
                if dx < strip_width && dy < strip_height {
                    bitmap[(dy * strip_width + dx) as usize] = glyph_bitmap[row * metrics.width + col];
                }
            }
        }

        let adv = (metrics.advance_width.ceil() as u32).max(1).min(255);
        glyph_widths.push(adv as u8);
        min_adv = min_adv.min(adv);
        max_adv = max_adv.max(adv);
    }

    // Compute tight bounding boxes for each glyph
    let mut glyph_bboxes = Vec::with_capacity(chars.len());
    for idx in 0..chars.len() {
        let cell_x = (idx as u32 % cols_per_row) * cell_width;
        let cell_y = (idx as u32 / cols_per_row) * cell_height;
        glyph_bboxes.push(compute_tight_bbox(&bitmap, cell_x, cell_y, cell_width, cell_height, strip_width));
    }

    // Extract kerning pairs (only non-zero, clamped to i8 range)
    let mut kerning_pairs = Vec::new();
    for (li, &lch) in chars.iter().enumerate() {
        for (ri, &rch) in chars.iter().enumerate() {
            if let Some(kern) = font.horizontal_kern(lch, rch, pixel_size) {
                let k = kern.round() as i32;
                if k != 0 {
                    let k_clamped = k.clamp(-128, 127) as i8;
                    kerning_pairs.push((li as u8, ri as u8, k_clamped));
                }
            }
        }
    }

    RasterizedFont {
        packed: pack_pixels(&bitmap, bpp),
        cell_width,
        cell_height,
        cols_per_row,
        strip_width,
        glyph_widths,
        glyph_bboxes,
        kerning_pairs,
        is_proportional: min_adv != max_adv,
    }
}

// ── Pixel packing ──────────────────────────────────────────────────────────

/// Pack 8-bit grayscale pixels into `bpp` bits per pixel, MSB-first.
fn pack_pixels(raw: &[u8], bpp: u8) -> Vec<u8> {
    if bpp == 8 { return raw.to_vec(); }

    let max_val = (1u16 << bpp) - 1;
    let ppb = 8 / bpp as usize; // pixels per byte
    let mut out = Vec::with_capacity(raw.len() / ppb + 1);
    let mut byte = 0u8;
    let mut pos = 0usize;

    for &val in raw {
        let q = ((val as u16 * max_val + 127) / 255) as u8;
        byte |= q << ((ppb - 1 - pos) as u8 * bpp);
        pos += 1;
        if pos == ppb {
            out.push(byte);
            byte = 0;
            pos = 0;
        }
    }
    if pos > 0 { out.push(byte); }
    out
}

// ── The proc macro ─────────────────────────────────────────────────────────

/// Compile-time font rasterization.
///
/// Reads a TTF/OTF font, rasterizes ASCII 32–126, and emits a `MicroFont` const.
///
/// ```ignore
/// use microface::{include_font, fonts::MicroFont};
///
/// const MONO: MicroFont = include_font!("fonts/mono.ttf", size = 24, bpp = 4);
/// const DIN: MicroFont = include_font!("fonts/din.otf", size = 32, bpp = 2);
/// ```
#[proc_macro]
pub fn include_font(input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input as IncludeFontArgs);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let font_path = std::path::Path::new(&manifest_dir).join(&args.path);
    let font_data = std::fs::read(&font_path).unwrap_or_else(|e| {
        panic!("include_font!: cannot read `{}`: {e}", font_path.display())
    });

    let r = rasterize(&font_data, args.size as f32, args.bpp);
    let bpp = args.bpp;
    let packed_len = r.packed.len();
    let (cw, ch, cpr, sw) = (r.cell_width, r.cell_height, r.cols_per_row, r.strip_width);

    // Report font size at compile time
    let widths_bytes = if r.is_proportional { r.glyph_widths.len() } else { 0 };
    let bbox_bytes = r.glyph_bboxes.len() * 4; // 4 bytes per glyph (x_off, y_off, tight_w, tight_h)
    let kern_bytes = r.kerning_pairs.len() * 3; // 3 bytes per pair (left, right, adj)
    let total_bytes = packed_len + widths_bytes + bbox_bytes + kern_bytes;
    let font_name = font_path.file_name().unwrap_or_default().to_string_lossy();
    let prop_label = if r.is_proportional { " (proportional)" } else { "" };
    let kern_label = if r.kerning_pairs.is_empty() { "" } else { " +kern" };
    eprintln!(
        "include_font!: {font_name} {size}px {bpp}bpp → {cw}×{ch} cell, {total_bytes} bytes{prop_label}{kern_label} ({kern_pairs} kern pairs)",
        size = args.size,
        kern_pairs = r.kerning_pairs.len(),
    );

    let data_tokens = r.packed.iter().map(|b| quote! { #b });

    let widths_tokens = if r.is_proportional {
        let ws = r.glyph_widths.iter().map(|w| quote! { #w });
        quote! { Some(&[#(#ws),*]) }
    } else {
        quote! { None }
    };

    // Emit tight bounding boxes as a flat &[u8] array: [x_off, y_off, tight_w, tight_h, ...]
    let bbox_flat: Vec<u8> = r.glyph_bboxes.iter()
        .flat_map(|bb| [bb.x_off, bb.y_off, bb.tight_w, bb.tight_h])
        .collect();
    let bbox_len = bbox_flat.len();
    let bbox_tokens = bbox_flat.iter().map(|b| quote! { #b });

    // Emit kerning pairs as a flat &[u8] array: [left, right, adj_as_u8, ...]
    let kern_tokens = if r.kerning_pairs.is_empty() {
        quote! { None }
    } else {
        let kern_flat: Vec<u8> = r.kerning_pairs.iter()
            .flat_map(|&(l, r, adj)| [l, r, adj as u8])
            .collect();
        let kern_len = kern_flat.len();
        let kt = kern_flat.iter().map(|b| quote! { #b });
        quote! { Some(&{ const _K: [u8; #kern_len] = [#(#kt),*]; _K }) }
    };

    quote! {{
        const _DATA: &[u8; #packed_len] = &[#(#data_tokens),*];
        const _BBOX: &[u8; #bbox_len] = &[#(#bbox_tokens),*];
        MicroFont {
            data: _DATA,
            char_width: #cw,
            char_height: #ch,
            cols_per_row: #cpr,
            strip_width: #sw,
            first_char: 32,
            last_char: 126,
            bpp: #bpp,
            widths: #widths_tokens,
            bboxes: _BBOX,
            kerning: #kern_tokens,
        }
    }}.into()
}
