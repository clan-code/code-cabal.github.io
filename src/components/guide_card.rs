use leptos::prelude::*;

use crate::data::guides::Guide;

#[component]
pub fn GuideMeta(guide: Guide) -> impl IntoView {
    view! {
        <div class="guide-meta" aria-label="Metadatos de la guía">
            <span>{guide.category.label()}</span>
            <span>{format!("{} min", guide.reading_minutes)}</span>
            <span class=guide.status.css_class()>{guide.status.label()}</span>
        </div>
    }
}

#[component]
pub fn GuideCard(guide: Guide) -> impl IntoView {
    let href = format!("/academia/{}", guide.slug);

    view! {
        <article class="guide-card">
            <a class="guide-card-media" href=href.clone() tabindex="-1" aria-hidden="true">
                <img loading="lazy" src=guide.image alt="" width="1280" height="720" />
            </a>
            <div class="guide-card-body">
                <GuideMeta guide=guide />
                <h3><a href=href>{guide.title}</a></h3>
                <p>{guide.summary}</p>
                <div class="guide-card-footer">
                    <span>{guide.difficulty}</span>
                    <a class="text-link" href=format!("/academia/{}", guide.slug)>
                        "Leer guía →"
                    </a>
                </div>
            </div>
        </article>
    }
}
