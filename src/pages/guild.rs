use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::{PageBanner, RankCard, SectionHeading},
    data::site::{GUILD_MANAGER, GUILD_MASTER, MOTTO, RANKS},
};

#[component]
pub fn GuildPage() -> impl IntoView {
    view! {
        <Title text="Guild CODE | Filosofía y rangos" />
        <PageBanner
            eyebrow="Guild Lv.12 · CABAL Online NA"
            title="CODE es una comunidad para aprender y compartir"
            lead="No medimos el valor de una persona por su CP. Los rangos reflejan participación, conocimiento compartido y compromiso."
            image="images/generated/hero-code.webp"
            image_alt="Miembros de una hermandad reciben a un nuevo aventurero"
        />

        <section class="section philosophy-section">
            <div class="container two-column philosophy-grid">
                <div>
                    <p class="eyebrow">"Nuestra filosofía"</p>
                    <h2>{MOTTO}</h2>
                    <p>"CODE nace para que una duda no se convierta en frustración. Ayudar no significa jugar por otra persona: significa darle contexto para que pueda decidir y progresar."</p>
                </div>
                <ul class="principles-list">
                    <li><strong>"No exigimos CP mínimo."</strong><span>"Importan la actitud y la convivencia."</span></li>
                    <li><strong>"Preguntar no resta valor."</strong><span>"Toda persona experta comenzó sin saber."</span></li>
                    <li><strong>"Compartimos conocimiento."</strong><span>"Una respuesta útil puede convertirse en una guía."</span></li>
                    <li><strong>"Progresamos con autonomía."</strong><span>"Orientamos para que cada miembro pueda avanzar."</span></li>
                </ul>
            </div>
        </section>

        <section class="section section-muted ranks-section">
            <div class="container">
                <SectionHeading
                    eyebrow="Estructura del guild"
                    title="Rangos por rol, no por poder"
                    description="El orden visual es Core → Senior → Junior → Alts. Guild Master y Guild Manager conservan las funciones administrativas del juego."
                />
                <div class="rank-grid">
                    <For
                        each=move || RANKS.iter().copied()
                        key=|rank| rank.name
                        children=move |rank| view! { <RankCard rank=rank /> }
                    />
                </div>
            </div>
        </section>

        <section class="section leadership-section">
            <div class="container">
                <SectionHeading
                    eyebrow="Administración"
                    title="Contactos principales"
                    description="Los rangos administrativos se asignan por confianza y responsabilidad, no como premio por CP."
                />
                <div class="leader-grid">
                    <article class="leader-card">
                        <img src="images/generated/emblem-code.webp" alt="" width="640" height="640" />
                        <div><span>"Guild Master"</span><h3>{GUILD_MASTER}</h3><p>"Dirección general, filosofía y crecimiento de CODE."</p></div>
                    </article>
                    <article class="leader-card">
                        <img src="images/generated/rank-alts.webp" alt="" width="512" height="512" />
                        <div><span>"Guild Manager"</span><h3>{GUILD_MANAGER}</h3><p>"Organización, acompañamiento y soporte a nuevos miembros."</p></div>
                    </article>
                </div>
            </div>
        </section>
    }
}
