---
name: release-pages
description: Validar y publicar la web estática de CODE en GitHub Pages de forma reproducible.
---

# Release en GitHub Pages

## Antes de fusionar

```bash
python3 scripts/validate_content.py
cargo fmt --all -- --check
cargo clippy --lib --all-targets -- -D warnings
cargo test --lib
trunk build --release
cp dist/index.html dist/404.html
```

Comprueba además:

- Las rutas directas cargan después de refrescar.
- Imágenes y favicon no devuelven 404.
- Búsqueda y filtros funcionan.
- Copiar mensaje funciona bajo HTTPS o localhost.
- No existen secretos ni datos personales.

## Configuración de Pages

- Repositorio público.
- Settings → Pages → Source: GitHub Actions.
- Workflow: `.github/workflows/deploy-pages.yml`.
- Permisos requeridos: `contents: read`, `pages: write`, `id-token: write`.

## Cambio de URL

Actualizar juntos:

1. `Trunk.toml` (`public_url`).
2. `public/robots.txt`.
3. `public/sitemap.xml`.
4. Metadatos Open Graph/canonical.
5. URL dentro del Guild Homepage.
