#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FaqItem {
    pub question: &'static str,
    pub answer: &'static str,
}

pub const FAQ_ITEMS: &[FaqItem] = &[
    FaqItem {
        question: "¿Necesito un CP mínimo para entrar?",
        answer: "No. En CODE el CP no define tu lugar. Nos importan el respeto, la participación y la disposición para aprender.",
    },
    FaqItem {
        question: "¿Aceptan jugadores nuevos?",
        answer: "Sí. El guild está pensado especialmente para personas que recién comienzan o que todavía están entendiendo la progresión de CABAL.",
    },
    FaqItem {
        question: "¿Aceptan jugadores que regresan después de mucho tiempo?",
        answer: "Sí. Volver a CABAL puede sentirse como empezar de nuevo; te ayudaremos a ordenar prioridades y recuperar contexto.",
    },
    FaqItem {
        question: "¿Debo escoger una clase específica?",
        answer: "No. CABAL Online NA mantiene nueve Battle Styles. Elige el que disfrutes y luego aprende a aprovechar sus fortalezas.",
    },
    FaqItem {
        question: "¿Cómo funcionan Junior, Senior y Core?",
        answer: "Junior es la entrada y etapa de aprendizaje; Senior reconoce experiencia compartida; Core representa compromiso sostenido con la comunidad. Ninguno depende del CP.",
    },
    FaqItem {
        question: "¿CODE entrega equipo o Alz?",
        answer: "No se promete equipo ni moneda. Podemos compartir orientación, explicar opciones y organizar actividades; la meta es que aprendas a progresar.",
    },
    FaqItem {
        question: "¿Debo conectarme todos los días?",
        answer: "No. La vida real va primero. Solo pedimos comunicación, respeto y participación razonable cuando estés disponible.",
    },
    FaqItem {
        question: "¿Puedo preguntar algo muy básico?",
        answer: "Sí. Pregunta sin miedo. Una respuesta clara hoy puede convertirse en la guía que ayude al siguiente jugador mañana.",
    },
    FaqItem {
        question: "¿Cómo solicito entrar?",
        answer: "Usa el generador de la sección Reclutamiento y envía el mensaje dentro del juego a zRust o zPython.",
    },
];
