use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::{GuideCard, PageBanner},
    data::guides::{GUIDES, GuideCategory, guide_matches},
};

#[component]
pub fn AcademyPage() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (category, set_category) = signal(String::from("all"));

    let filtered = Memo::new(move |_| {
        let query = query.get();
        let category = category.get();
        GUIDES
            .iter()
            .copied()
            .filter(|guide| guide_matches(*guide, &query, &category))
            .collect::<Vec<_>>()
    });

    view! {
        <Title text="Academia CODE | Guías de CABAL Online NA" />
        <PageBanner
            eyebrow="Pregunta · Aprende · Comparte"
            title="Academia CODE"
            lead="Guías breves y organizadas para ayudarte a tomar mejores decisiones en CABAL Online NA."
            image="images/generated/academy.webp"
            image_alt="Una sala de conocimiento de fantasía tecnológica"
        />

        <section class="section academy-section">
            <div class="container">
                <div class="academy-toolbar" role="search">
                    <label class="search-field">
                        <span>"Buscar una guía"</span>
                        <input
                            type="search"
                            placeholder="Ejemplo: CP, equipo, Alz..."
                            prop:value=move || query.get()
                            on:input=move |event| set_query.set(event_target_value(&event))
                        />
                    </label>
                    <label class="select-field">
                        <span>"Categoría"</span>
                        <select on:change=move |event| set_category.set(event_target_value(&event))>
                            <option value="all">"Todas las categorías"</option>
                            <For
                                each=move || GuideCategory::ALL
                                key=|category| category.slug()
                                children=move |category| {
                                    view! { <option value=category.slug()>{category.label()}</option> }
                                }
                            />
                        </select>
                    </label>
                    <p class="result-count" aria-live="polite">
                        {move || format!("{} guía(s)", filtered.get().len())}
                    </p>
                </div>

                <Show
                    when=move || !filtered.get().is_empty()
                    fallback=move || {
                        view! {
                            <div class="empty-state">
                                <span aria-hidden="true">"⌕"</span>
                                <h2>"No encontramos una guía con esos filtros"</h2>
                                <p>"Prueba una palabra más general o selecciona todas las categorías."</p>
                            </div>
                        }
                    }
                >
                    <div class="guide-grid">
                        <For
                            each=move || filtered.get()
                            key=|guide| guide.slug
                            children=move |guide| view! { <GuideCard guide=guide /> }
                        />
                    </div>
                </Show>
            </div>
        </section>
    }
}
