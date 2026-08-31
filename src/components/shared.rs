use leptos::prelude::*;

use crate::data::site::GuildRank;

#[component]
pub fn PageBanner(
    eyebrow: &'static str,
    title: &'static str,
    lead: &'static str,
    image: &'static str,
    image_alt: &'static str,
) -> impl IntoView {
    view! {
        <section class="page-banner">
            <div class="container page-banner-grid">
                <div class="page-banner-copy">
                    <p class="eyebrow">{eyebrow}</p>
                    <h1>{title}</h1>
                    <p class="page-lead">{lead}</p>
                </div>
                <div class="page-banner-media" aria-hidden=image_alt.is_empty()>
                    <img src=image alt=image_alt width="1280" height="720" />
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn SectionHeading(
    eyebrow: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="section-heading">
            <p class="eyebrow">{eyebrow}</p>
            <h2>{title}</h2>
            <p>{description}</p>
        </div>
    }
}

#[component]
pub fn RankCard(rank: GuildRank) -> impl IntoView {
    view! {
        <article class="rank-card">
            <img src=rank.image alt=format!("Emblema de {}", rank.name) width="512" height="512" />
            <div>
                <h3>{rank.name}</h3>
                <p>{rank.description}</p>
                <small>{rank.criteria}</small>
            </div>
        </article>
    }
}
