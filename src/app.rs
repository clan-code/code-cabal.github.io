use leptos::prelude::*;
use leptos_meta::{Meta, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::{SiteFooter, SiteHeader},
    pages::{
        AcademyPage, FaqPage, GalleryPage, GuidePage, GuildPage, HomePage, NotFoundPage,
        RecruitmentPage, SecurityPage, StartPage,
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="CODE | Guild Lv.12 de CABAL Online NA" />
        <Meta
            name="description"
            content="CODE es un Guild Lv.12 de CABAL Online NA enfocado en nuevos jugadores, aprendizaje y progreso en comunidad."
        />

        <Router>
            <a class="skip-link" href="#contenido">"Saltar al contenido"</a>
            <SiteHeader />
            <main id="contenido">
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/empieza-aqui") view=StartPage />
                    <Route path=path!("/academia") view=AcademyPage />
                    <Route path=path!("/academia/:slug") view=GuidePage />
                    <Route path=path!("/guild") view=GuildPage />
                    <Route path=path!("/reclutamiento") view=RecruitmentPage />
                    <Route path=path!("/faq") view=FaqPage />
                    <Route path=path!("/seguridad") view=SecurityPage />
                    <Route path=path!("/galeria") view=GalleryPage />
                </Routes>
            </main>
            <SiteFooter />
        </Router>
    }
}
