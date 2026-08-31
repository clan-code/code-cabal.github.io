# Contribuir a CODE

Gracias por ayudar a construir la web y la Academia del guild.

## Flujo recomendado

1. Crea una rama pequeña y enfocada.
2. Lee `AGENTS.md` y la skill pertinente en `skills/`.
3. No inventes datos sobre CABAL NA; separa hechos oficiales de experiencia del guild.
4. Ejecuta las validaciones locales.
5. Describe claramente qué cambió y cómo se verificó.

```bash
python3 scripts/validate_content.py
cargo fmt --all -- --check
cargo clippy --lib --all-targets -- -D warnings
cargo test --lib
trunk build --release
```

## Principios

- Mantener el MVP estático y sin backend.
- Evitar dependencias para tareas que resuelve la plataforma web.
- Priorizar móvil, accesibilidad y claridad para jugadores nuevos.
- No clasificar miembros por CP.
- No publicar información personal ni credenciales.
