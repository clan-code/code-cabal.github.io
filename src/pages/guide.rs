use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::{
    components::{GuideCard, GuideMeta},
    data::guides::{Guide, find_guide, related_guides},
    markdown::render_markdown,
};

#[component]
pub fn GuidePage() -> impl IntoView {
    let params = use_params_map();
    let guide = Memo::new(move |_| {
        let slug = params.read().get("slug").unwrap_or_default();
        find_guide(&slug)
    });

    view! {
        {move || match guide.get() {
            Some(guide) => Either::Left(view! { <GuideContent guide=guide /> }),
            None => Either::Right(view! {
                <section class="section not-found-inline">
                    <div class="container narrow">
                        <p class="eyebrow">"Academia CODE"</p>
                        <h1>"La guía no existe"</h1>
                        <p>"Puede que la ruta haya cambiado o que el contenido todavía no esté publicado."</p>
                        <a class="button button-primary" href="/academia">"Volver a la Academia"</a>
                    </div>
                </section>
            }),
        }}
    }
}

#[component]
fn GuideContent(guide: Guide) -> impl IntoView {
    let html = render_markdown(guide.body);
    let related = related_guides(guide, 3);
    let related_section = if related.is_empty() {
        Either::Left(())
    } else {
        let cards = related
            .into_iter()
            .map(|guide| view! { <GuideCard guide=guide /> })
            .collect_view();

        Either::Right(view! {
            <section class="section related-guides">
                <div class="container">
                    <div class="section-heading">
                        <p class="eyebrow">"Continúa aprendiendo"</p>
                        <h2>"Guías relacionadas"</h2>
                    </div>
                    <div class="guide-grid">
                        {cards}
                    </div>
                </div>
            </section>
        })
    };

    view! {
        <Title text=format!("{} | Academia CODE", guide.title) />

        <article class="guide-page">
            <header class="guide-header">
                <div class="container guide-header-grid">
                    <div>
                        <a class="back-link" href="/academia">"← Volver a la Academia"</a>
                        <GuideMeta guide=guide />
                        <h1>{guide.title}</h1>
                        <p class="page-lead">{guide.summary}</p>
                    </div>
                    <img src=guide.image alt="" width="1280" height="720" />
                </div>
            </header>

            <div class="container guide-layout">
                <aside class="guide-facts" aria-label="Información de la guía">
                    <h2>"Ficha"</h2>
                    <dl>
                        <div><dt>"Región"</dt><dd>{guide.region}</dd></div>
                        <div><dt>"Revisión"</dt><dd>{guide.updated}</dd></div>
                        <div><dt>"Episodio"</dt><dd>{guide.episode}</dd></div>
                        <div><dt>"Dificultad"</dt><dd>{guide.difficulty}</dd></div>
                        <div><dt>"Tipo de fuente"</dt><dd>{guide.source_kind}</dd></div>
                    </dl>
                    <p class="guide-facts-note">"CABAL cambia con eventos y parches. Reporta en el guild cualquier dato que parezca desactualizado."</p>
                </aside>
                <div class="prose" inner_html=html></div>
            </div>
        </article>

        {related_section}
    }
}
