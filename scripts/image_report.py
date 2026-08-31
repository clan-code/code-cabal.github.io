#!/usr/bin/env python3
"""Muestra el peso de imágenes publicadas; no modifica archivos."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
IMAGE_DIR = ROOT / "public/images"

for path in sorted(IMAGE_DIR.rglob("*")):
    if path.is_file():
        size_kb = path.stat().st_size / 1024
        print(f"{size_kb:8.1f} KB  {path.relative_to(ROOT)}")
