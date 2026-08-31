use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Página no encontrada | CODE" />
        <section class="not-found-page">
            <div class="container narrow">
                <span class="not-found-code">"404"</span>
                <p class="eyebrow">"Ruta desconocida"</p>
                <h1>"Esta parte de Nevareth aún no está en el mapa"</h1>
                <p>"Vuelve al inicio o entra a la Academia para continuar tu recorrido."</p>
                <div class="inline-actions">
                    <a class="button button-primary" href="/">"Volver al inicio"</a>
                    <a class="button button-secondary" href="/academia">"Ir a la Academia"</a>
                </div>
            </div>
        </section>
    }
}
