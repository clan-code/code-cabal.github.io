use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::PageBanner;

#[component]
pub fn SecurityPage() -> impl IntoView {
    view! {
        <Title text="Seguridad de cuenta | CODE" />
        <PageBanner
            eyebrow="Tu cuenta es tu responsabilidad"
            title="CODE nunca te pedirá credenciales"
            lead="Una web de guild debe ayudarte, no parecer una pantalla de ingreso. Desconfía de cualquier persona que solicite datos de tu cuenta."
            image="images/generated/security.webp"
            image_alt="Un escudo digital con un candado representa la protección de la cuenta"
        />

        <section class="section security-section">
            <div class="container security-grid">
                <article class="danger-panel">
                    <p class="eyebrow">"Nunca compartas"</p>
                    <h2>"Información que CODE no necesita"</h2>
                    <ul class="deny-list">
                        <li>"ID de PlayThisGame"</li>
                        <li>"Contraseña o sub-password"</li>
                        <li>"Códigos de autenticación 2FA"</li>
                        <li>"Respuestas secretas"</li>
                        <li>"Correo asociado a la cuenta"</li>
                        <li>"eCoins, pagos o acceso remoto a tu PC"</li>
                    </ul>
                </article>

                <article class="panel">
                    <p class="eyebrow">"Antes de abrir un enlace"</p>
                    <h2>"Detente y verifica"</h2>
                    <ol class="security-steps">
                        <li><span>"1"</span><div><strong>"Revisa el dominio"</strong><p>"Una letra cambiada puede llevarte a una copia falsa."</p></div></li>
                        <li><span>"2"</span><div><strong>"No ingreses desde mensajes urgentes"</strong><p>"Premios y amenazas son tácticas comunes para apresurarte."</p></div></li>
                        <li><span>"3"</span><div><strong>"Abre el sitio oficial por tu cuenta"</strong><p>"Escribe la dirección conocida o usa un marcador guardado."</p></div></li>
                        <li><span>"4"</span><div><strong>"Pregunta antes de actuar"</strong><p>"Consulta a zRust o zPython si un mensaje usa el nombre de CODE."</p></div></li>
                    </ol>
                </article>
            </div>

            <div class="container official-callout">
                <div>
                    <h2>"Usa fuentes oficiales para tu cuenta"</h2>
                    <p>"Este sitio no contiene login. Para soporte, descargas o información de cuenta, ve directamente al portal oficial de CABAL Online NA."</p>
                </div>
                <a class="button button-secondary" href="https://www.playthisgame.com/CM/en/Support/Home" target="_blank" rel="noreferrer external">
                    "Abrir soporte oficial ↗"
                </a>
            </div>
        </section>
    }
}
