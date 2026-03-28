#!/usr/bin/env python3
"""Convert any font (TTF, OTF, BDF) to a packed grayscale bitmap + Rust GrayFont definition.

Auto-detects input format by file extension. Uses Pillow for rendering.
Supports configurable bits-per-pixel (1, 2, 4, or 8) for size/quality tradeoff.

Single size:
    python3 tools/font2bin.py fonts/abc.ttf 24 abc --bpp 4

Multiple sizes (packed into one .bin):
    python3 tools/font2bin.py fonts/abc.ttf 16,24,32 abc --bpp 4

Outputs (single size):
    src/fonts/data/abc.bin         — packed grayscale bitmap
    src/fonts/abc.rs               — Rust GrayFont definition
    fonts/abc_preview.png          — visual preview

Outputs (multi-size):
    src/fonts/data/abc.bin         — all sizes concatenated
    src/fonts/abc.rs               — Rust module with all size consts
    fonts/abc_16_preview.png       — preview per size
    fonts/abc_24_preview.png
    fonts/abc_32_preview.png
"""

import argparse
import tempfile
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, BdfFontFile

# Directory layout (relative to project root)
SRC_FONTS_DIR = Path("src/fonts")
DATA_DIR = SRC_FONTS_DIR / "data"
PREVIEW_DIR = Path("fonts")


def pack_pixels(raw_bytes: bytes, bpp: int) -> bytes:
    """Pack 8-bit grayscale pixels into the target bit depth, MSB-first."""
    if bpp == 8:
        return raw_bytes

    max_val = (1 << bpp) - 1
    pixels_per_byte = 8 // bpp

    packed = bytearray()
    current_byte = 0
    pixel_in_byte = 0

    for val in raw_bytes:
        quantized = (val * max_val + 127) // 255
        shift = (pixels_per_byte - 1 - pixel_in_byte) * bpp
        current_byte |= (quantized << shift)
        pixel_in_byte += 1

        if pixel_in_byte == pixels_per_byte:
            packed.append(current_byte)
            current_byte = 0
            pixel_in_byte = 0

    if pixel_in_byte > 0:
        packed.append(current_byte)

    return bytes(packed)


def load_font(font_path: str, pixel_size: int):
    """Load a font from any supported format. Returns (PIL ImageFont, actual_pixel_size)."""
    ext = Path(font_path).suffix.lower()

    if ext == ".bdf":
        tmp_dir = tempfile.mkdtemp(prefix="font2bin_")
        with open(font_path, "rb") as bdf_file:
            bdf = BdfFontFile.BdfFontFile(bdf_file)
            pil_path = str(Path(tmp_dir) / "font.pil")
            bdf.save(pil_path)
        font = ImageFont.load(pil_path)
        bbox = font.getbbox("Ag")
        actual_size = bbox[3] - bbox[1]
        print(f"BDF font loaded (native size {actual_size}px, pixel_size arg ignored)")
        return font, actual_size
    elif ext in (".ttf", ".otf"):
        font = ImageFont.truetype(font_path, pixel_size)
        return font, pixel_size
    else:
        try:
            font = ImageFont.truetype(font_path, pixel_size)
            return font, pixel_size
        except Exception:
            font = ImageFont.load(font_path)
            bbox = font.getbbox("Ag")
            return font, bbox[3] - bbox[1]


def render_font_size(font_path, pixel_size, bpp):
    """Render one font size. Returns dict with all metadata + packed bitmap."""
    font, actual_size = load_font(font_path, pixel_size)
    chars = [chr(c) for c in range(32, 127)]

    widths = []
    heights = []
    for ch in chars:
        bbox = font.getbbox(ch)
        widths.append(bbox[2] - bbox[0])
        heights.append(bbox[3] - bbox[1])

    if hasattr(font, 'getmetrics'):
        ascent, descent = font.getmetrics()
        cell_height = ascent + descent
    else:
        cell_height = max(heights) if heights else actual_size
        ascent = cell_height
        descent = 0

    cell_width = max(widths) if widths and max(widths) > 0 else actual_size // 2
    if cell_width == 0:
        cell_width = actual_size // 2

    cols_per_row = 16
    num_rows = (len(chars) + cols_per_row - 1) // cols_per_row
    strip_width = cell_width * cols_per_row
    strip_height = cell_height * num_rows

    # Render
    img = Image.new('L', (strip_width, strip_height), 0)
    draw = ImageDraw.Draw(img)
    for idx, ch in enumerate(chars):
        x = (idx % cols_per_row) * cell_width
        y = (idx // cols_per_row) * cell_height
        draw.text((x, y), ch, font=font, fill=255)

    raw_bitmap = img.tobytes()
    packed_bitmap = pack_pixels(raw_bitmap, bpp)

    # Build quantized preview
    if bpp < 8:
        max_val = (1 << bpp) - 1
        quantized_pixels = bytes(
            ((val * max_val + 127) // 255) * 255 // max_val if max_val > 0 else 0
            for val in raw_bitmap
        )
        preview_img = Image.frombytes('L', (strip_width, strip_height), quantized_pixels)
    else:
        preview_img = img

    # Detect proportional: are glyph widths variable?
    is_proportional = min(widths) != max(widths)

    return {
        'pixel_size': actual_size,
        'cell_width': cell_width,
        'cell_height': cell_height,
        'cols_per_row': cols_per_row,
        'strip_width': strip_width,
        'packed_bitmap': packed_bitmap,
        'preview_img': preview_img,
        'ascent': ascent,
        'descent': descent,
        'glyph_widths': widths,  # per-char advance widths
        'is_proportional': is_proportional,
    }


def main():
    parser = argparse.ArgumentParser(
        description="Convert any font (TTF/OTF/BDF) to packed grayscale bitmap + Rust GrayFont"
    )
    parser.add_argument("font_path", help="Path to font file (TTF, OTF, or BDF)")
    parser.add_argument("pixel_sizes", help="Font size(s) in pixels, comma-separated (e.g. '16,24,32')")
    parser.add_argument("name", help="Font family name (e.g. abc)")
    parser.add_argument(
        "--bpp", type=int, default=4, choices=[1, 2, 4, 8],
        help="Bits per pixel (default: 4). Lower = smaller file, fewer alpha levels."
    )
    args = parser.parse_args()

    font_path = args.font_path
    sizes = [int(s.strip()) for s in args.pixel_sizes.split(",")]
    name = args.name
    bpp = args.bpp

    DATA_DIR.mkdir(parents=True, exist_ok=True)
    PREVIEW_DIR.mkdir(parents=True, exist_ok=True)

    font_filename = Path(font_path).name
    font_ext = Path(font_path).suffix.upper().lstrip(".")

    # Render all sizes
    renders = []
    for size in sizes:
        print(f"\n--- Rendering {font_filename} at {size}px, {bpp}bpp ---")
        info = render_font_size(font_path, size, bpp)
        renders.append(info)
        print(f"  Cell: {info['cell_width']}x{info['cell_height']}, "
              f"packed: {len(info['packed_bitmap'])} bytes")

    # Concatenate all bitmaps into one .bin file
    combined = bytearray()
    offsets = []
    for info in renders:
        offsets.append(len(combined))
        combined.extend(info['packed_bitmap'])

    bin_path = DATA_DIR / f"{name}.bin"
    with open(bin_path, "wb") as f:
        f.write(combined)
    print(f"\nWrote {bin_path} ({len(combined)} bytes, {len(sizes)} size(s))")

    # Save previews
    for info, size in zip(renders, sizes):
        if len(sizes) == 1:
            preview_path = PREVIEW_DIR / f"{name}_preview.png"
        else:
            preview_path = PREVIEW_DIR / f"{name}_{size}_preview.png"
        info['preview_img'].save(preview_path)
        print(f"Wrote {preview_path}")

    # Write Rust source — one file with all size consts
    rs_path = SRC_FONTS_DIR / f"{name}.rs"
    bin_filename = f"data/{name}.bin"

    with open(rs_path, "w") as f:
        size_list = ", ".join(str(s) for s in sizes)
        f.write(f'//! Auto-generated from {font_filename} at [{size_list}]px ({bpp}bpp) by font2bin.py\n')
        f.write(f'//! {len(sizes)} size(s), {len(combined)} bytes total ({bpp}-bit grayscale)\n\n')
        f.write(f'use crate::fonts::GrayFont;\n\n')
        f.write(f'/// All sizes packed into one binary.\n')
        f.write(f'const DATA: &[u8] = include_bytes!("{bin_filename}");\n\n')

        # Helper to get a const subslice — works on stable Rust via split_at
        f.write(f'/// Const helper: extract a subslice from DATA at compile time.\n')
        f.write(f'const fn subslice(data: &[u8], offset: usize, len: usize) -> &[u8] {{\n')
        f.write(f'    let (_, rest) = data.split_at(offset);\n')
        f.write(f'    let (slice, _) = rest.split_at(len);\n')
        f.write(f'    slice\n')
        f.write(f'}}\n\n')

        for info, size, offset in zip(renders, sizes, offsets):
            packed_len = len(info['packed_bitmap'])
            const_name = f"{name.upper()}_{size}"
            prop = info['is_proportional']

            # Emit width table for proportional fonts
            if prop:
                widths_name = f"WIDTHS_{size}"
                widths_str = ", ".join(str(w) for w in info['glyph_widths'])
                f.write(f'/// Per-glyph advance widths at {size}px (proportional)\n')
                f.write(f'const {widths_name}: &[u8] = &[{widths_str}];\n\n')

            f.write(f'/// {font_filename} at {size}px — {info["cell_width"]}x{info["cell_height"]}, '
                    f'{packed_len} bytes at offset {offset}')
            if prop:
                f.write(f' (proportional)')
            f.write(f'\n')
            f.write(f'pub const {const_name}: GrayFont = GrayFont {{\n')
            f.write(f'    data: subslice(DATA, {offset}, {packed_len}),\n')
            f.write(f'    char_width: {info["cell_width"]},\n')
            f.write(f'    char_height: {info["cell_height"]},\n')
            f.write(f'    cols_per_row: {info["cols_per_row"]},\n')
            f.write(f'    strip_width: {info["strip_width"]},\n')
            f.write(f'    first_char: 32,\n')
            f.write(f'    last_char: 126,\n')
            f.write(f'    bpp: {bpp},\n')
            if prop:
                f.write(f'    widths: Some({widths_name}),\n')
            else:
                f.write(f'    widths: None,\n')
            f.write(f'}};\n\n')

    print(f"Wrote {rs_path}")


if __name__ == "__main__":
    main()
