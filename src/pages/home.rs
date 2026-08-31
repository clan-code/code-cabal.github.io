use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::{GuideCard, SectionHeading},
    data::{
        guides::GUIDES,
        site::{
            GUILD_MANAGER, GUILD_MASTER, JOURNEY, MOTTO, RECRUITMENT_LINE, SITE_SUBTITLE, VALUES,
        },
    },
};

#[component]
pub fn HomePage() -> impl IntoView {
    let featured_guides = GUIDES
        .iter()
        .copied()
        .filter(|guide| guide.featured)
        .take(3)
        .map(|guide| view! { <GuideCard guide=guide /> })
        .collect_view();

    view! {
        <Title text="CODE | Guild Lv.12 de CABAL Online NA" />

        <section class="hero">
            <picture class="hero-media">
                <source media="(max-width: 680px)" srcset="images/generated/hero-code-mobile.webp" />
                <img
                    src="images/generated/hero-code.webp"
                    alt="Un aventurero nuevo se acerca a los miembros del guild CODE"
                    width="1920"
                    height="1080"
                />
            </picture>
            <div class="hero-overlay"></div>
            <div class="container hero-inner">
                <div class="hero-copy">
                    <p class="eyebrow">{SITE_SUBTITLE}</p>
                    <h1>{RECRUITMENT_LINE}</h1>
                    <p class="hero-motto">{MOTTO}</p>
                    <div class="hero-actions">
                        <a class="button button-primary" href="/empieza-aqui">"Soy nuevo en CABAL"</a>
                        <a class="button button-secondary" href="/academia">"Ver guías"</a>
                        <a class="button button-ghost" href="/reclutamiento">"Quiero unirme"</a>
                    </div>
                    <div class="hero-contacts" aria-label="Contactos del guild">
                        <span><small>"Guild Master"</small>{GUILD_MASTER}</span>
                        <span><small>"Guild Manager"</small>{GUILD_MANAGER}</span>
                    </div>
                </div>
            </div>
        </section>

        <section class="section values-section" aria-labelledby="values-title">
            <div class="container">
                <div class="section-heading section-heading-centered">
                    <p class="eyebrow">"Nuestra filosofía"</p>
                    <h2 id="values-title">"Un guild donde ser nuevo está bien"</h2>
                    <p>"No buscamos impresionar con requisitos. Buscamos construir una comunidad que enseñe, aprenda y avance junta."</p>
                </div>
                <div class="values-grid">
                    <For
                        each=move || VALUES.iter().copied()
                        key=|value| value.title
                        children=move |value| {
                            view! {
                                <article class="value-card">
                                    <span class="value-icon" aria-hidden="true">{value.icon}</span>
                                    <h3>{value.title}</h3>
                                    <p>{value.description}</p>
                                </article>
                            }
                        }
                    />
                </div>
            </div>
        </section>

        <section class="section journey-section">
            <div class="container">
                <SectionHeading
                    eyebrow="Tu primera ruta"
                    title="No necesitas entender todo hoy"
                    description="Empieza con una pregunta, resuelve el siguiente paso y vuelve cuando aparezca la próxima duda."
                />
                <ol class="journey-grid">
                    <For
                        each=move || JOURNEY.iter().copied()
                        key=|step| step.number
                        children=move |step| {
                            view! {
                                <li class="journey-card">
                                    <span>{step.number}</span>
                                    <h3>{step.title}</h3>
                                    <p>{step.description}</p>
                                    <a class="text-link" href=step.href>"Explorar →"</a>
                                </li>
                            }
                        }
                    />
                </ol>
            </div>
        </section>

        <section class="section academy-preview">
            <div class="container">
                <div class="split-heading">
                    <SectionHeading
                        eyebrow="Academia CODE"
                        title="Conocimiento útil para tu siguiente decisión"
                        description="Las guías indican región, fecha y tipo de fuente para que sepas qué estás leyendo."
                    />
                    <a class="button button-secondary" href="/academia">"Ver todas las guías"</a>
                </div>
                <div class="guide-grid guide-grid-featured">
                    {featured_guides}
                </div>
            </div>
        </section>

        <section class="section guild-callout">
            <div class="container guild-callout-grid">
                <img
                    src="images/generated/emblem-code.webp"
                    alt="Emblema original de CODE"
                    loading="lazy"
                    width="640"
                    height="640"
                />
                <div>
                    <p class="eyebrow">"CODE · Guild Lv.12"</p>
                    <h2>"Una comunidad enfocada en nuevos jugadores"</h2>
                    <p>"Los rangos no dependen del CP. Junior aprende, Senior comparte experiencia y Core ayuda a construir la comunidad."</p>
                    <div class="inline-actions">
                        <a class="button button-primary" href="/guild">"Conocer el guild"</a>
                        <a class="button button-ghost" href="/reclutamiento">"Solicitar ingreso"</a>
                    </div>
                </div>
            </div>
        </section>
    }
}
