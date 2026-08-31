use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::PageBanner,
    data::site::{GUILD_MANAGER, GUILD_MASTER},
};

const CLASSES: &[&str] = &[
    "Aún no lo sé",
    "Guerrero (Warrior)",
    "Espadachín (Blader)",
    "Mago (Wizard)",
    "Arquero Aethral (Force Archer)",
    "Escudero Aethral (Force Shielder)",
    "Luchador Aethral (Force Blader)",
    "Gladiador (Gladiator)",
    "Tirador Aethral (Force Gunner)",
    "Mago Oscuro (Dark Mage)",
];

#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(text: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let _ = window.navigator().clipboard().write_text(text);
    true
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_to_clipboard(_text: &str) -> bool {
    false
}

#[component]
pub fn RecruitmentPage() -> impl IntoView {
    let (character, set_character) = signal(String::new());
    let (class_name, set_class_name) = signal(String::from(CLASSES[0]));
    let (level, set_level) = signal(String::new());
    let (experience, set_experience) = signal(String::from("soy nuevo en CABAL"));
    let (goal, set_goal) = signal(String::from("entender mi progresión"));
    let (copied, set_copied) = signal(false);

    let message = Memo::new(move |_| {
        let character = character.get();
        let level = level.get();
        let character = if character.trim().is_empty() {
            "[mi personaje]"
        } else {
            character.trim()
        };
        let level = if level.trim().is_empty() {
            "[mi nivel]"
        } else {
            level.trim()
        };

        format!(
            "Hola, soy {character}, juego como {} y {}. Actualmente soy nivel {level} y me gustaría aprender sobre {}. Quisiera unirme a CODE. ¿Con quién puedo coordinar?",
            class_name.get(),
            experience.get(),
            goal.get()
        )
    });

    let on_copy = move |_| {
        set_copied.set(copy_to_clipboard(&message.get()));
    };

    view! {
        <Title text="Reclutamiento | Únete a CODE" />
        <PageBanner
            eyebrow="Nuevos jugadores bienvenidos"
            title="No buscamos el personaje perfecto; buscamos personas que quieran crecer"
            lead="CODE es un Guild Lv.12 de CABAL Online NA. No exigimos CP mínimo y no prometemos equipo: ofrecemos comunidad, contexto y aprendizaje."
            image="images/generated/recruitment.webp"
            image_alt="Un nuevo camino se abre entre montañas de fantasía"
        />

        <section class="section recruitment-section">
            <div class="container recruitment-grid">
                <div>
                    <p class="eyebrow">"¿A quién buscamos?"</p>
                    <h2>"Requisitos sencillos y claros"</h2>
                    <ul class="check-list recruitment-requirements">
                        <li>"Jugar en CABAL Online NA."</li>
                        <li>"Mantener una convivencia respetuosa."</li>
                        <li>"Tener disposición para aprender y preguntar."</li>
                        <li>"Compartir conocimiento cuando sea posible."</li>
                        <li>"No utilizar trampas, estafas ni comercio por dinero real."</li>
                        <li><strong>"No se exige CP mínimo."</strong></li>
                    </ul>
                    <div class="contact-panel">
                        <h3>"Contacta dentro del juego"</h3>
                        <p><strong>{GUILD_MASTER}</strong>" · Guild Master"</p>
                        <p><strong>{GUILD_MANAGER}</strong>" · Guild Manager"</p>
                    </div>
                </div>

                <div class="recruitment-form panel">
                    <p class="eyebrow">"Genera tu presentación"</p>
                    <h2>"Crea un mensaje para copiar"</h2>
                    <p class="form-note">"Nada se envía ni se almacena. Todo sucede localmente en tu navegador."</p>

                    <div class="form-grid">
                        <label>
                            <span>"Nombre del personaje"</span>
                            <input
                                type="text"
                                placeholder="Ejemplo: MiPersonaje"
                                prop:value=move || character.get()
                                on:input=move |event| {
                                    set_character.set(event_target_value(&event));
                                    set_copied.set(false);
                                }
                            />
                        </label>
                        <label>
                            <span>"Clase"</span>
                            <select on:change=move |event| {
                                set_class_name.set(event_target_value(&event));
                                set_copied.set(false);
                            }>
                                <For
                                    each=move || CLASSES.iter().copied()
                                    key=|class_name| *class_name
                                    children=move |class_name| view! { <option value=class_name>{class_name}</option> }
                                />
                            </select>
                        </label>
                        <label>
                            <span>"Nivel"</span>
                            <input
                                type="text"
                                inputmode="numeric"
                                placeholder="Ejemplo: 120"
                                prop:value=move || level.get()
                                on:input=move |event| {
                                    set_level.set(event_target_value(&event));
                                    set_copied.set(false);
                                }
                            />
                        </label>
                        <label>
                            <span>"Situación"</span>
                            <select on:change=move |event| {
                                set_experience.set(event_target_value(&event));
                                set_copied.set(false);
                            }>
                                <option value="soy nuevo en CABAL">"Soy nuevo"</option>
                                <option value="estoy regresando a CABAL">"Estoy regresando"</option>
                                <option value="todavía estoy aprendiendo CABAL">"Ya juego, pero sigo aprendiendo"</option>
                            </select>
                        </label>
                        <label class="form-span-2">
                            <span>"¿Qué te gustaría aprender?"</span>
                            <select on:change=move |event| {
                                set_goal.set(event_target_value(&event));
                                set_copied.set(false);
                            }>
                                <option value="entender mi progresión">"Progresión general"</option>
                                <option value="mejorar mi equipo">"Equipo"</option>
                                <option value="aprender dungeons">"Dungeons"</option>
                                <option value="administrar mejor mis Alz">"Economía y Alz"</option>
                                <option value="entender mejor mi clase">"Mi clase"</option>
                            </select>
                        </label>
                    </div>

                    <label class="message-preview">
                        <span>"Vista previa"</span>
                        <textarea rows="8" readonly=true prop:value=move || message.get()></textarea>
                    </label>
                    <button class="button button-primary button-full" type="button" on:click=on_copy>
                        {move || if copied.get() { "¡Mensaje copiado!" } else { "Copiar mensaje" }}
                    </button>
                    <p class="copy-help" aria-live="polite">
                        {move || if copied.get() {
                            "Ahora envíalo dentro del juego a zRust o zPython."
                        } else {
                            "El portapapeles funciona en HTTPS o localhost."
                        }}
                    </p>
                </div>
            </div>
        </section>
    }
}
