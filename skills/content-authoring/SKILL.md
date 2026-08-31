---
name: content-authoring
description: Crear o actualizar guías claras, verificables y seguras para jugadores nuevos de CABAL Online NA.
---

# Autoría de contenido

## Objetivo

Una persona nueva debe terminar cada guía sabiendo qué hacer después y por qué.

## Pasos para una guía nueva

1. Crear `content/guides/<slug>.md`.
2. Registrar `Guide` en `src/data/guides.rs`.
3. Elegir categoría y dificultad existentes.
4. Indicar región `CABAL NA`, fecha de revisión y episodio/parche.
5. Distinguir `Fuente oficial`, `Experiencia CODE` o `En revisión`.
6. Añadir enlaces primarios cuando se afirme algo cambiante.
7. Ejecutar `python3 scripts/validate_content.py`.
8. Verificar tarjeta, búsqueda, filtro y ruta individual.

## Estructura recomendada

```markdown
## Qué vas a aprender

## La idea principal

## Pasos recomendados

## Errores frecuentes

## Siguiente paso
```

## Estilo

- Frases cortas y directas.
- Explicar siglas la primera vez.
- No humillar a quien pregunta.
- No prometer resultados, equipo o Alz.
- Evitar valores exactos que cambian con eventos o parches salvo que estén fechados.
- Cuando haya varias estrategias válidas, presentar opciones y condiciones.
