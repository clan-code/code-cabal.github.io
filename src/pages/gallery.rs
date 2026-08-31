use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::PageBanner;

#[derive(Clone, Copy)]
struct GalleryItem {
    src: &'static str,
    alt: &'static str,
    title: &'static str,
    caption: &'static str,
}

const ITEMS: &[GalleryItem] = &[
    GalleryItem {
        src: "images/gallery/guild-first-roster.webp",
        alt: "Primera organización del roster del guild CODE dentro de CABAL",
        title: "Los primeros personajes",
        caption: "La etapa inicial de CODE antes de comenzar el reclutamiento público.",
    },
    GalleryItem {
        src: "images/gallery/guild-information.webp",
        alt: "Ventana de edición de información del guild CODE",
        title: "Definiendo la identidad",
        caption: "El espacio donde se configuraron bienvenida, introducción y futura web.",
    },
    GalleryItem {
        src: "images/gallery/guild-structure.webp",
        alt: "Roster de CODE con los grupos Core, Senior, Junior y Alts",
        title: "La estructura de CODE",
        caption: "Rangos por función dentro de la comunidad y no por CP.",
    },
];

#[component]
pub fn GalleryPage() -> impl IntoView {
    view! {
        <Title text="Galería | CODE" />
        <PageBanner
            eyebrow="Nuestra historia"
            title="CODE también se construye con pequeños momentos"
            lead="Capturas del crecimiento del guild y arte original creado para darle una identidad propia."
            image="images/generated/hero-code.webp"
            image_alt="La hermandad CODE reunida alrededor de una fogata"
        />

        <section class="section">
            <div class="container gallery-grid">
                <For
                    each=move || ITEMS.iter().copied()
                    key=|item| item.src
                    children=move |item| {
                        view! {
                            <figure class="gallery-card">
                                <a href=item.src target="_blank" rel="noreferrer">
                                    <img loading="lazy" src=item.src alt=item.alt />
                                </a>
                                <figcaption>
                                    <h2>{item.title}</h2>
                                    <p>{item.caption}</p>
                                </figcaption>
                            </figure>
                        }
                    }
                />
            </div>
        </section>
    }
}
