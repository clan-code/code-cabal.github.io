---
name: leptos-web
description: Implementar y mantener la aplicación Leptos CSR de CODE sin ampliar innecesariamente la arquitectura.
---

# Leptos Web

## Úsala cuando

- Agregues una ruta, página o componente.
- Modifiques señales, formularios, filtros o navegación.
- Cambies dependencias Rust o configuración de Trunk.

## Flujo

1. Identifica la página y el componente mínimo afectado.
2. Reutiliza datos desde `src/data`; no dupliques textos globales.
3. Usa señales locales para estado efímero. No agregues state managers.
4. Usa enlaces reales para navegación; evita botones que simulan enlaces.
5. Implementa estado vacío cuando un filtro no tenga resultados.
6. Mantén la lógica de negocio fuera del markup cuando supere unas pocas líneas.
7. Añade una prueba unitaria a funciones puras.
8. Ejecuta fmt, clippy, tests y `trunk build --release`.

## Convenciones

- Componentes en `PascalCase`.
- Una página por archivo en `src/pages`.
- Props pequeñas y tipadas; preferir `&'static str` para datos estáticos.
- No usar `unwrap()` con datos externos o parámetros de ruta.
- HTML crudo solo puede provenir de `markdown::render_markdown`.
- Para colecciones estáticas, preferir `iterator.map(...).collect_view()`; reservar `<For>` para listas reactivas.
- Evitar expresiones con `::<...>` directamente dentro de una prop RSX; calcularlas antes del `view!` para no confundir al parser del macro.
- Para Web APIs, aislar código con `#[cfg(target_arch = "wasm32")]`.

## No hacer

- Añadir SSR, Axum o base de datos para contenido versionado.
- Añadir Tailwind, un kit de componentes o JavaScript manual solo por comodidad.
- Cargar contenido desde una API que no exista.
- Implementar login o almacenamiento de solicitudes de reclutamiento.
