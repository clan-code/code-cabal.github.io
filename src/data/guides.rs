#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideCategory {
    PrimerosPasos,
    Progresion,
    Equipo,
    Dungeons,
    Economia,
    Comunidad,
    Seguridad,
}

impl GuideCategory {
    pub const ALL: [Self; 7] = [
        Self::PrimerosPasos,
        Self::Progresion,
        Self::Equipo,
        Self::Dungeons,
        Self::Economia,
        Self::Comunidad,
        Self::Seguridad,
    ];

    pub const fn slug(self) -> &'static str {
        match self {
            Self::PrimerosPasos => "primeros-pasos",
            Self::Progresion => "progresion",
            Self::Equipo => "equipo",
            Self::Dungeons => "dungeons",
            Self::Economia => "economia",
            Self::Comunidad => "comunidad",
            Self::Seguridad => "seguridad",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::PrimerosPasos => "Primeros pasos",
            Self::Progresion => "Progresión",
            Self::Equipo => "Equipo",
            Self::Dungeons => "Dungeons",
            Self::Economia => "Economía",
            Self::Comunidad => "Comunidad",
            Self::Seguridad => "Seguridad",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideStatus {
    Verified,
    CodeExperience,
    InReview,
}

impl GuideStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Verified => "Verificada",
            Self::CodeExperience => "Experiencia CODE",
            Self::InReview => "En revisión",
        }
    }

    pub const fn css_class(self) -> &'static str {
        match self {
            Self::Verified => "status-verified",
            Self::CodeExperience => "status-code",
            Self::InReview => "status-review",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guide {
    pub title: &'static str,
    pub slug: &'static str,
    pub summary: &'static str,
    pub category: GuideCategory,
    pub difficulty: &'static str,
    pub region: &'static str,
    pub episode: &'static str,
    pub updated: &'static str,
    pub status: GuideStatus,
    pub source_kind: &'static str,
    pub reading_minutes: u8,
    pub featured: bool,
    pub image: &'static str,
    pub body: &'static str,
}

pub const GUIDES: &[Guide] = &[
    Guide {
        title: "Empieza sin intentar aprenderlo todo",
        slug: "primeros-pasos",
        summary: "Una ruta mental sencilla para no perderte durante tus primeros días en CABAL.",
        category: GuideCategory::PrimerosPasos,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Contenido general",
        updated: "2026-08-30",
        status: GuideStatus::CodeExperience,
        source_kind: "Experiencia CODE",
        reading_minutes: 4,
        featured: true,
        image: "images/generated/hero-code-mobile.webp",
        body: include_str!("../../content/guides/primeros-pasos.md"),
    },
    Guide {
        title: "¿Qué es el CP y para qué sirve?",
        slug: "que-es-cp",
        summary: "Cómo interpretar el Combat Power sin convertirlo en tu única meta.",
        category: GuideCategory::PrimerosPasos,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Contenido general",
        updated: "2026-08-30",
        status: GuideStatus::CodeExperience,
        source_kind: "Explicación comunitaria",
        reading_minutes: 4,
        featured: true,
        image: "images/generated/academy.webp",
        body: include_str!("../../content/guides/que-es-cp.md"),
    },
    Guide {
        title: "Cómo elegir tu primera clase",
        slug: "elegir-clase",
        summary: "Una decisión basada en estilo de juego, no en perseguir la clase de moda.",
        category: GuideCategory::PrimerosPasos,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "9 Battle Styles",
        updated: "2026-08-30",
        status: GuideStatus::Verified,
        source_kind: "Fuente oficial + criterio CODE",
        reading_minutes: 6,
        featured: true,
        image: "images/generated/recruitment.webp",
        body: include_str!("../../content/guides/elegir-clase.md"),
    },
    Guide {
        title: "Ruta de progreso: decide tu siguiente paso",
        slug: "ruta-de-progreso",
        summary: "Un método para avanzar con objetivos pequeños y evitar mejorar todo al mismo tiempo.",
        category: GuideCategory::Progresion,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Contenido general",
        updated: "2026-08-30",
        status: GuideStatus::CodeExperience,
        source_kind: "Experiencia CODE",
        reading_minutes: 5,
        featured: true,
        image: "images/generated/hero-code.webp",
        body: include_str!("../../content/guides/ruta-de-progreso.md"),
    },
    Guide {
        title: "Qué mejorar primero en tu equipo",
        slug: "mejorar-equipo",
        summary: "Prioridades y preguntas útiles antes de gastar materiales o Alz.",
        category: GuideCategory::Equipo,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Revisar según parche",
        updated: "2026-08-30",
        status: GuideStatus::InReview,
        source_kind: "Marco de decisión, no build final",
        reading_minutes: 6,
        featured: false,
        image: "images/generated/academy.webp",
        body: include_str!("../../content/guides/mejorar-equipo.md"),
    },
    Guide {
        title: "Cómo empezar a aprender dungeons",
        slug: "dungeons-principiantes",
        summary: "Una forma segura de practicar rutas, mecánicas y consumo de recursos.",
        category: GuideCategory::Dungeons,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Revisar según parche",
        updated: "2026-08-30",
        status: GuideStatus::InReview,
        source_kind: "Experiencia CODE",
        reading_minutes: 5,
        featured: false,
        image: "images/generated/recruitment.webp",
        body: include_str!("../../content/guides/dungeons-principiantes.md"),
    },
    Guide {
        title: "Cuida tus Alz desde el primer día",
        slug: "cuidar-tus-alz",
        summary: "Hábitos sencillos para reducir compras impulsivas y errores costosos.",
        category: GuideCategory::Economia,
        difficulty: "Principiante",
        region: "CABAL Online NA",
        episode: "Economía variable",
        updated: "2026-08-30",
        status: GuideStatus::CodeExperience,
        source_kind: "Buenas prácticas comunitarias",
        reading_minutes: 5,
        featured: false,
        image: "images/generated/academy.webp",
        body: include_str!("../../content/guides/cuidar-tus-alz.md"),
    },
    Guide {
        title: "Cómo pedir ayuda para recibir una buena respuesta",
        slug: "pedir-ayuda",
        summary: "Qué información compartir para que el guild pueda orientarte con rapidez.",
        category: GuideCategory::Comunidad,
        difficulty: "Todos",
        region: "CABAL Online NA",
        episode: "No aplica",
        updated: "2026-08-30",
        status: GuideStatus::CodeExperience,
        source_kind: "Cultura CODE",
        reading_minutes: 3,
        featured: false,
        image: "images/generated/hero-code.webp",
        body: include_str!("../../content/guides/pedir-ayuda.md"),
    },
    Guide {
        title: "Protege tu cuenta y reconoce enlaces sospechosos",
        slug: "seguridad-de-cuenta",
        summary: "Reglas básicas para no entregar credenciales ni caer en páginas falsas.",
        category: GuideCategory::Seguridad,
        difficulty: "Todos",
        region: "CABAL Online NA",
        episode: "Vigente",
        updated: "2026-08-30",
        status: GuideStatus::Verified,
        source_kind: "Recomendaciones oficiales de seguridad",
        reading_minutes: 4,
        featured: true,
        image: "images/generated/security.webp",
        body: include_str!("../../content/guides/seguridad-de-cuenta.md"),
    },
];

pub fn find_guide(slug: &str) -> Option<Guide> {
    GUIDES.iter().copied().find(|guide| guide.slug == slug)
}

pub fn related_guides(current: Guide, limit: usize) -> Vec<Guide> {
    GUIDES
        .iter()
        .copied()
        .filter(|guide| guide.slug != current.slug && guide.category == current.category)
        .take(limit)
        .collect()
}

pub fn guide_matches(guide: Guide, query: &str, category: &str) -> bool {
    let category_matches = category == "all" || guide.category.slug() == category;
    if !category_matches {
        return false;
    }

    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }

    guide.title.to_lowercase().contains(&query)
        || guide.summary.to_lowercase().contains(&query)
        || guide.category.label().to_lowercase().contains(&query)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn guide_slugs_are_unique() {
        let mut seen = HashSet::new();
        for guide in GUIDES {
            assert!(seen.insert(guide.slug), "slug duplicado: {}", guide.slug);
        }
    }

    #[test]
    fn search_is_case_insensitive() {
        let guide = GUIDES[1];
        assert!(guide_matches(guide, "COMBAT", "all"));
        assert!(!guide_matches(guide, "dungeons", "all"));
    }

    #[test]
    fn category_filter_works() {
        let guide = GUIDES[0];
        assert!(guide_matches(guide, "", "primeros-pasos"));
        assert!(!guide_matches(guide, "", "seguridad"));
    }
}
