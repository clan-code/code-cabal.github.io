use leptos::prelude::*;

use crate::data::site::{
    GUILD_MANAGER, GUILD_MASTER, MOTTO, NAV_ITEMS, SITE_NAME, SITE_SUBTITLE,
};

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container header-inner">
                <a class="brand" href="/" aria-label="Ir al inicio de CODE">
                    <img
                        class="brand-emblem"
                        src="images/generated/emblem-code.webp"
                        alt="Emblema original del guild CODE"
                        width="56"
                        height="56"
                    />
                    <span class="brand-copy">
                        <strong>{SITE_NAME}</strong>
                        <small>{SITE_SUBTITLE}</small>
                    </span>
                </a>

                <nav class="desktop-nav" aria-label="Navegación principal">
                    <For
                        each=move || NAV_ITEMS.iter().copied()
                        key=|item| item.href
                        children=move |item| {
                            view! { <a class="nav-link" href=item.href>{item.label}</a> }
                        }
                    />
                    <a class="button button-small button-primary" href="/reclutamiento">
                        "Unirme a CODE"
                    </a>
                </nav>

                <details class="mobile-nav">
                    <summary aria-label="Abrir navegación">
                        <span></span><span></span><span></span>
                    </summary>
                    <nav aria-label="Navegación móvil">
                        <For
                            each=move || NAV_ITEMS.iter().copied()
                            key=|item| item.href
                            children=move |item| {
                                view! { <a class="mobile-nav-link" href=item.href>{item.label}</a> }
                            }
                        />
                    </nav>
                </details>
            </div>
        </header>
    }
}

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container footer-grid">
                <div>
                    <a class="brand brand-footer" href="/">
                        <img
                            class="brand-emblem"
                            src="images/generated/emblem-code.webp"
                            alt=""
                            width="48"
                            height="48"
                        />
                        <span class="brand-copy">
                            <strong>{SITE_NAME}</strong>
                            <small>{SITE_SUBTITLE}</small>
                        </span>
                    </a>
                    <p class="footer-motto">{MOTTO}</p>
                </div>

                <div>
                    <h2 class="footer-title">"Contacto en el juego"</h2>
                    <p><strong>"Guild Master: "</strong>{GUILD_MASTER}</p>
                    <p><strong>"Guild Manager: "</strong>{GUILD_MANAGER}</p>
                </div>

                <div>
                    <h2 class="footer-title">"Enlaces"</h2>
                    <a href="/academia">"Academia CODE"</a>
                    <a href="/seguridad">"Seguridad"</a>
                    <a href="https://cabal.playthisgame.com/es/Classes" target="_blank" rel="noreferrer external">
                        "Clases oficiales de CABAL NA ↗"
                    </a>
                </div>
            </div>

            <div class="container footer-bottom">
                <p>"© 2026 CODE Guild. Sitio creado por fans y para jugadores."</p>
                <p>
                    "No afiliado a ESTsoft ni PlayThisGame. CABAL Online y sus marcas pertenecen a sus respectivos propietarios."
                </p>
            </div>
        </footer>
    }
}
