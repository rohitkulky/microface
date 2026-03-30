#!/bin/bash
# Zip source code only (no builds, binaries, or generated files).
# Uses password from ./zippwd file. Output goes to ../../archives/

set -e
cd "$(dirname "$0")"

PWD_FILE="zippwd"
if [ ! -f "$PWD_FILE" ]; then
    echo "Error: $PWD_FILE not found" >&2
    exit 1
fi

PASSWORD=$(head -1 "$PWD_FILE" | tr -d '\n\r')
TIMESTAMP=$(date +%s)
ARCHIVE_DIR="../../archives"
mkdir -p "$ARCHIVE_DIR"
OUTPUT="$ARCHIVE_DIR/microface_${TIMESTAMP}.zip"

zip -r -P "$PASSWORD" "$OUTPUT" * \
    -x "target/*" "*/target/*" \
    "*/.DS_Store" "*.zip" "*.bin" "*.bdf" "*.bmp" \
    "GUIDE_TEXT_ELEMENT.md" "ALIGN_ISSUE.md" \
    "screenshots/*" "tools/*"

echo "Created $OUTPUT ($(du -h "$OUTPUT" | cut -f1))"
