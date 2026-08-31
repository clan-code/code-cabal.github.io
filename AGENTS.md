# AGENTS.md — CODE Guild Website

Este archivo es la guía operativa para agentes de IA y desarrolladores humanos. Aplica a todo el repositorio.

## 1. Misión

Construir y mantener la identidad web del guild **CODE**, Guild Lv.12 de **CABAL Online NA**, con prioridad en jugadores nuevos.

Filosofía inmutable del producto:

> Pregunta sin miedo. Aprende y comparte. Aquí crecemos juntos.

Mensaje principal:

> Aquí no importa cómo empiezas, sino hasta dónde quieres llegar.

## 2. Restricciones no negociables

- Mantener el MVP **estático**, sin backend, base de datos ni autenticación.
- No solicitar credenciales, correos de cuenta, códigos 2FA, Alz, eCoins ni acceso remoto.
- No clasificar miembros por CP. Los rangos representan participación, experiencia compartida y compromiso.
- No inventar información sobre CABAL NA. Etiquetar claramente experiencia del guild frente a fuente oficial.
- No copiar logotipos, personajes ni interfaces oficiales en imágenes promocionales.
- No agregar una dependencia cuando Rust, Leptos, CSS o la plataforma web ya resuelvan el caso de forma sencilla.
- La interfaz visible debe permanecer en español; conservar términos oficiales en inglés cuando ayuden a buscar información.
- Proteger la experiencia móvil y la accesibilidad.

## 3. Arquitectura

```text
src/app.rs                Router y layout global
src/components/           Piezas visuales reutilizables
src/pages/                Una página por módulo
src/data/                 Configuración, rangos, FAQ y registro tipado de guías
src/markdown.rs           Markdown confiable → HTML, bloqueando HTML crudo
content/guides/           Cuerpo de cada guía
styles/                   Tokens, base, componentes, páginas y responsive
public/images/            Activos publicados
scripts/                  Validaciones sin dependencias externas
design/                   Dirección visual, prompts y manifiesto
skills/                   Procedimientos especializados
```

No crear un workspace, una API o un sistema de plugins sin un requisito aprobado.

## 4. Comandos de calidad

Ejecutar antes de cerrar una tarea:

```bash
python3 scripts/validate_content.py
cargo fmt --all -- --check
cargo clippy --lib --all-targets -- -D warnings
cargo test --lib
trunk build --release
```

Cuando no exista toolchain Rust en el entorno, ejecutar al menos la validación Python, revisar rutas/activos y declarar explícitamente qué no se pudo compilar.

## 5. Cambios de contenido

Antes de editar una guía, leer `skills/content-authoring/SKILL.md`.

- El Markdown contiene solo el cuerpo.
- Los metadatos viven en `src/data/guides.rs`.
- El `slug` es ASCII, minúsculo y usa guiones.
- Toda guía necesita `region`, `updated`, `episode`, `status` y `source_kind`.
- Enlaces externos deben apuntar preferentemente a PlayThisGame/CABAL NA u otra fuente primaria.
- Contenido potencialmente cambiante requiere fecha y lenguaje prudente.

## 6. Cambios de UI

Antes de editar componentes o CSS, leer `skills/leptos-web/SKILL.md` y `skills/visual-identity/SKILL.md`.

- Reutilizar tokens en `styles/tokens.css`.
- No introducir estilos inline salvo valores realmente dinámicos.
- No depender solo del color para comunicar estados.
- Conservar foco visible, labels, landmarks semánticos y targets táctiles adecuados.
- Probar 360 px, 768 px, 1366 px y 1920 px.
- Evitar animaciones continuas; respetar `prefers-reduced-motion`.
- Para listas estáticas en Leptos, construir la vista antes de `view!` con `collect_view()`; usar `<For>` solo cuando la colección sea reactiva.

## 7. Imágenes

- Los activos aprobados están en `public/images/`.
- La procedencia está en `design/asset-manifest.toml`.
- Los originales generados no se usan directamente si son pesados; publicar versiones WebP optimizadas.
- Actualizar `alt` al cambiar el significado visual.
- No publicar chats privados ni datos de cuenta en screenshots.

## 8. Despliegue

Leer `skills/release-pages/SKILL.md`.

- `main` es publicable.
- GitHub Pages usa `dist/` producido por Trunk.
- Copiar `dist/index.html` a `dist/404.html` para soportar rutas SPA.
- Si cambia el dominio o nombre de organización, actualizar `Trunk.toml`, sitemap, robots y metadata.

## 9. Definición de terminado

Una tarea está terminada cuando:

1. Cumple el objetivo sin ampliar alcance innecesariamente.
2. No rompe rutas, búsqueda, filtros o navegación móvil.
3. Incluye estados vacíos y de error cuando corresponda.
4. Pasa los comandos disponibles.
5. Actualiza documentación si cambia una decisión.
6. Explica riesgos o validaciones pendientes sin fingir certeza.
