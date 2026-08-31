#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueCard {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JourneyStep {
    pub number: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub href: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuildRank {
    pub name: &'static str,
    pub image: &'static str,
    pub description: &'static str,
    pub criteria: &'static str,
}

pub const SITE_NAME: &str = "CODE";
pub const SITE_SUBTITLE: &str = "Guild Lv.12 · CABAL Online NA";
pub const MOTTO: &str = "Pregunta sin miedo. Aprende y comparte. Aquí crecemos juntos.";
pub const RECRUITMENT_LINE: &str =
    "Aquí no importa cómo empiezas, sino hasta dónde quieres llegar.";
pub const GUILD_MASTER: &str = "zRust";
pub const GUILD_MANAGER: &str = "zPython";

pub const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        label: "Inicio",
        href: "/",
    },
    NavItem {
        label: "Empieza aquí",
        href: "/empieza-aqui",
    },
    NavItem {
        label: "Academia",
        href: "/academia",
    },
    NavItem {
        label: "Guild",
        href: "/guild",
    },
    NavItem {
        label: "Reclutamiento",
        href: "/reclutamiento",
    },
    NavItem {
        label: "FAQ",
        href: "/faq",
    },
    NavItem {
        label: "Seguridad",
        href: "/seguridad",
    },
    NavItem {
        label: "Galería",
        href: "/galeria",
    },
];

pub const VALUES: &[ValueCard] = &[
    ValueCard {
        icon: "◇",
        title: "Sin CP mínimo",
        description: "Tu equipo no define tu lugar. Importan las ganas de aprender y participar.",
    },
    ValueCard {
        icon: "◎",
        title: "Comunidad",
        description: "Jugamos en equipo, respondemos dudas y celebramos el progreso de todos.",
    },
    ValueCard {
        icon: "▤",
        title: "Conocimiento",
        description: "Lo que una persona aprende se convierte en una ayuda para la siguiente.",
    },
    ValueCard {
        icon: "✦",
        title: "Progreso",
        description: "Cada paso cuenta. Buscamos decisiones útiles, no atajos que no enseñan.",
    },
    ValueCard {
        icon: "⬡",
        title: "Respeto",
        description: "Preguntar está bien. Ayudamos sin ego y mantenemos un ambiente sano.",
    },
];

pub const JOURNEY: &[JourneyStep] = &[
    JourneyStep {
        number: "01",
        title: "Entiende tu clase",
        description: "Conoce su estilo, fortalezas y función antes de invertir recursos.",
        href: "/academia/elegir-clase",
    },
    JourneyStep {
        number: "02",
        title: "Sube de nivel",
        description: "Avanza con objetivos pequeños y aprende qué desbloquea cada etapa.",
        href: "/academia/ruta-de-progreso",
    },
    JourneyStep {
        number: "03",
        title: "Mejora tu equipo",
        description: "Prioriza mejoras que realmente aporten a tu personaje.",
        href: "/academia/mejorar-equipo",
    },
    JourneyStep {
        number: "04",
        title: "Aprende dungeons",
        description: "Empieza con mecánicas simples y aumenta la dificultad con criterio.",
        href: "/academia/dungeons-principiantes",
    },
    JourneyStep {
        number: "05",
        title: "Cuida tus Alz",
        description: "Evita compras impulsivas y aprende a reconocer una mejora rentable.",
        href: "/academia/cuidar-tus-alz",
    },
    JourneyStep {
        number: "06",
        title: "Pregunta y participa",
        description: "Tu comunidad está para orientarte, no para juzgarte.",
        href: "/guild",
    },
];

pub const RANKS: &[GuildRank] = &[
    GuildRank {
        name: "CODE Core",
        image: "images/generated/rank-core.webp",
        description: "Miembros comprometidos con construir y cuidar la comunidad.",
        criteria: "Confianza, constancia, iniciativa y apoyo sostenido al guild.",
    },
    GuildRank {
        name: "CODE Senior",
        image: "images/generated/rank-senior.webp",
        description: "Jugadores con experiencia que comparten lo que saben.",
        criteria: "Conocimiento útil, buena convivencia y disposición para orientar.",
    },
    GuildRank {
        name: "CODE Junior",
        image: "images/generated/rank-junior.webp",
        description: "Jugadores nuevos o miembros que todavía están aprendiendo.",
        criteria: "Es la puerta de entrada; no es una etiqueta de poder ni de CP.",
    },
    GuildRank {
        name: "Alts",
        image: "images/generated/rank-alts.webp",
        description: "Personajes secundarios vinculados a miembros del guild.",
        criteria: "Mantiene el roster claro y evita confundir alts con personas distintas.",
    },
];
