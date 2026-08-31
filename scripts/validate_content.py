#!/usr/bin/env python3
"""Validaciones rápidas del contenido y los activos, sin dependencias externas."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDE_REGISTRY = ROOT / "src/data/guides.rs"
GUIDE_DIR = ROOT / "content/guides"
PUBLIC_DIR = ROOT / "public"

REQUIRED_FILES = [
    ROOT / "AGENTS.md",
    ROOT / "README.md",
    ROOT / "Cargo.toml",
    ROOT / "Trunk.toml",
    ROOT / "index.html",
    ROOT / "design/asset-manifest.toml",
]


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def main() -> int:
    errors: list[str] = []

    for path in REQUIRED_FILES:
        if not path.is_file():
            fail(errors, f"Falta archivo requerido: {path.relative_to(ROOT)}")

    if not GUIDE_REGISTRY.is_file():
        fail(errors, "No existe src/data/guides.rs")
        return report(errors)

    registry = GUIDE_REGISTRY.read_text(encoding="utf-8")
    blocks = re.findall(r"Guide\s*\{(.*?)\n\s*\},", registry, flags=re.DOTALL)
    if not blocks:
        fail(errors, "No se encontraron entradas Guide en el registro")
        return report(errors)

    slugs: list[str] = []
    registered_markdown: set[Path] = set()

    for index, block in enumerate(blocks, start=1):
        slug_match = re.search(r'slug:\s*"([^"]+)"', block)
        body_match = re.search(r'include_str!\("([^"]+)"\)', block)
        image_match = re.search(r'image:\s*"([^"]+)"', block)
        required_fields = [
            "title:",
            "summary:",
            "category:",
            "difficulty:",
            "region:",
            "episode:",
            "updated:",
            "status:",
            "source_kind:",
        ]

        for field in required_fields:
            if field not in block:
                fail(errors, f"Guía #{index}: falta el campo {field}")

        if not slug_match:
            fail(errors, f"Guía #{index}: falta slug")
        else:
            slug = slug_match.group(1)
            slugs.append(slug)
            if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", slug):
                fail(errors, f"Slug inválido: {slug}")

        if not body_match:
            fail(errors, f"Guía #{index}: falta include_str! del Markdown")
        else:
            relative = body_match.group(1)
            markdown = (GUIDE_REGISTRY.parent / relative).resolve()
            registered_markdown.add(markdown)
            if not markdown.is_file():
                fail(errors, f"Markdown no encontrado: {relative}")
            else:
                text = markdown.read_text(encoding="utf-8").strip()
                if len(text) < 120:
                    fail(errors, f"Guía demasiado corta: {markdown.relative_to(ROOT)}")
                if re.search(r"<(script|iframe|object|embed|form)\b", text, flags=re.I):
                    fail(errors, f"HTML peligroso en: {markdown.relative_to(ROOT)}")
                if text.startswith("# "):
                    fail(
                        errors,
                        f"La guía no debe incluir H1; la página ya lo crea: {markdown.relative_to(ROOT)}",
                    )

        if not image_match:
            fail(errors, f"Guía #{index}: falta image")
        else:
            image = PUBLIC_DIR / image_match.group(1).removeprefix("/")
            if not image.is_file():
                fail(errors, f"Imagen no encontrada: {image_match.group(1)}")

    duplicates = sorted({slug for slug in slugs if slugs.count(slug) > 1})
    for slug in duplicates:
        fail(errors, f"Slug duplicado: {slug}")

    markdown_files = {path.resolve() for path in GUIDE_DIR.glob("*.md")}
    for path in sorted(markdown_files - registered_markdown):
        fail(errors, f"Markdown sin registrar: {path.relative_to(ROOT)}")
    for path in sorted(registered_markdown - markdown_files):
        if path.is_file():
            fail(errors, f"Guía registrada fuera de content/guides: {path.relative_to(ROOT)}")

    image_limits = {
        "hero-code.webp": 350 * 1024,
        "hero-code-mobile.webp": 250 * 1024,
        "academy.webp": 150 * 1024,
        "recruitment.webp": 150 * 1024,
        "security.webp": 150 * 1024,
    }
    generated = PUBLIC_DIR / "images/generated"
    for name, max_bytes in image_limits.items():
        path = generated / name
        if not path.is_file():
            fail(errors, f"Falta activo generado: {path.relative_to(ROOT)}")
        elif path.stat().st_size > max_bytes:
            fail(
                errors,
                f"Activo excede presupuesto ({path.stat().st_size // 1024} KB): {path.relative_to(ROOT)}",
            )

    return report(errors, guide_count=len(blocks), markdown_count=len(markdown_files))


def report(errors: list[str], guide_count: int = 0, markdown_count: int = 0) -> int:
    if errors:
        print("Validación fallida:\n")
        for error in errors:
            print(f"  - {error}")
        return 1

    print(
        f"Validación OK: {guide_count} guías registradas, "
        f"{markdown_count} archivos Markdown y activos principales presentes."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
