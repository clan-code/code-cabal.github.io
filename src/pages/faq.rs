use leptos::prelude::*;
use leptos_meta::Title;

use crate::{components::PageBanner, data::faq::FAQ_ITEMS};

#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <Title text="Preguntas frecuentes | CODE" />
        <PageBanner
            eyebrow="Respuestas rápidas"
            title="Preguntas frecuentes"
            lead="Lo esencial sobre ingreso, rangos, convivencia y la filosofía de CODE."
            image="images/generated/academy.webp"
            image_alt="Una sala de aprendizaje de fantasía tecnológica"
        />

        <section class="section">
            <div class="container narrow">
                <div class="faq-list">
                    <For
                        each=move || FAQ_ITEMS.iter().copied()
                        key=|item| item.question
                        children=move |item| {
                            view! {
                                <details class="faq-item">
                                    <summary>{item.question}</summary>
                                    <p>{item.answer}</p>
                                </details>
                            }
                        }
                    />
                </div>
                <div class="faq-cta panel panel-accent">
                    <h2>"¿Tu pregunta no aparece?"</h2>
                    <p>"Esa es precisamente la cultura de CODE: pregunta sin miedo. Una buena duda puede convertirse en la próxima guía de la Academia."</p>
                    <a class="button button-secondary" href="/reclutamiento">"Contactar al guild"</a>
                </div>
            </div>
        </section>
    }
}
