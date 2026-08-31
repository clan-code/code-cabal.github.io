# Arquitectura

## Objetivo

Mantener una web pública de costo cero que funcione como identidad, puerta de entrada y fuente de conocimiento del guild CODE.

## Decisiones

- **Leptos CSR + Trunk:** suficiente para el MVP y compatible con GitHub Pages.
- **Una sola crate:** reduce coordinación y evita sobreingeniería.
- **Contenido Markdown embebido:** no requiere red ni backend; los metadatos tipados viven en Rust.
- **CSS plano:** no añade toolchains de Node, Sass ni Tailwind.
- **Imágenes estáticas optimizadas:** WebP, sin servicios externos durante la ejecución.
- **Formularios locales:** generan texto y no transmiten datos.

## Capas

```text
src/data       Datos estáticos y registro de guías
content        Cuerpo Markdown de las guías
src/markdown   Conversión segura de Markdown a HTML
src/components Componentes reutilizables
src/pages      Páginas y estado local
styles         Tokens y CSS global
public         Activos que Trunk publica
```

## Cuándo crecer

Agregar un backend solo cuando exista un requerimiento real que no pueda resolverse con archivos versionados: por ejemplo, contribuciones de miembros no técnicos o calendario editable sin despliegue. Antes de eso, preferir pull requests y Markdown.
