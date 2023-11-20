#!/bin/bash
for HEADER in /usr/include/libdisplay-info/*.h; do
    OUT="libdisplay-info-sys/src/$(basename "$HEADER" .h)".rs
    bindgen "$HEADER" --allowlist-type="di_.*" --allowlist-function="di_.*" -o "$OUT"
done