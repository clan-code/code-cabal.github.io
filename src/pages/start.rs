use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::{PageBanner, SectionHeading},
    data::site::JOURNEY,
};

#[component]
pub fn StartPage() -> impl IntoView {
    view! {
        <Title text="Empieza aquí | CODE" />
        <PageBanner
            eyebrow="Para nuevos y quienes regresan"
            title="Tu camino no necesita empezar perfecto"
            lead="Ordena tus dudas, toma una decisión a la vez y evita gastar recursos solo por seguir una moda."
            image="images/generated/recruitment.webp"
            image_alt="Un camino entre montañas representa el inicio de una nueva aventura"
        />

        <section class="section">
            <div class="container">
                <SectionHeading
                    eyebrow="Ruta recomendada"
                    title="Seis pasos para comenzar con dirección"
                    description="No son requisitos ni una carrera. Son puntos de referencia para saber qué pregunta hacer después."
                />
                <ol class="start-steps">
                    <For
                        each=move || JOURNEY.iter().copied()
                        key=|step| step.number
                        children=move |step| {
                            view! {
                                <li>
                                    <span class="step-number">{step.number}</span>
                                    <div>
                                        <h2>{step.title}</h2>
                                        <p>{step.description}</p>
                                        <a class="text-link" href=step.href>"Abrir recurso →"</a>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ol>
            </div>
        </section>

        <section class="section section-muted">
            <div class="container two-column">
                <article class="panel">
                    <p class="eyebrow">"Antes de gastar"</p>
                    <h2>"Hazte estas cuatro preguntas"</h2>
                    <ul class="check-list">
                        <li>"¿Esta mejora sirve para mi objetivo actual?"</li>
                        <li>"¿Entiendo qué atributo estoy comprando?"</li>
                        <li>"¿Existe una alternativa más barata para aprender?"</li>
                        <li>"¿Puedo preguntar en el guild antes de decidir?"</li>
                    </ul>
                </article>
                <article class="panel panel-accent">
                    <p class="eyebrow">"Recuerda"</p>
                    <h2>"Tu CP es una referencia, no tu identidad"</h2>
                    <p>"Subir un número sin entender tu personaje puede dejarte con más poder y las mismas dudas. En CODE buscamos que cada mejora tenga sentido."</p>
                    <a class="button button-secondary" href="/academia/que-es-cp">"Entender el CP"</a>
                </article>
            </div>
        </section>
    }
}
