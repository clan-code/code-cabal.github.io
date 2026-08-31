# Changelog

## 0.1.1 — 2026-08-31

### Corregido

- La lista estática de guías destacadas de `HomePage` ahora se construye antes de `view!`, evitando que el parser RSX interprete el cierre de `collect::<Vec<_>>()` como parte de una etiqueta.
- La sección de guías relacionadas ya no mueve su colección desde una clausura que Leptos necesita reutilizar como `Fn`.
- `Trunk.toml` usa `addresses` en lugar del campo obsoleto `address` y desactiva la búsqueda inversa de nombres para mostrar únicamente la dirección local esperada.

### Documentación

- Se documentó en `AGENTS.md` y `skills/leptos-web/SKILL.md` la convención para renderizar colecciones estáticas y evitar expresiones turbofish dentro de propiedades RSX.
