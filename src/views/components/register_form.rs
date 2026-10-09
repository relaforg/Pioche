use leptos::prelude::*;

use crate::server::auth::Register;
use crate::views::{colors::Color, components::button_link::ButtonLink};

#[component]
pub fn RegisterForm() -> impl IntoView {
    view! {
        <div class="max-w-110 border shadow-blue-5 rounded-blob bg-surface">
            <div class="p-7">
                <div class="flex gap-3 mb-7">
                    <div class="flex-1">
                        <ButtonLink
                            label="Connexion"
                            link="/connexion"
                            bg_color=Color::Surface
                            shadow_color=Color::Blue
                            class="w-full"
                        />
                    </div>
                    <div class="flex-1">
                        <ButtonLink
                            label="Créer un compte"
                            link="/inscription"
                            bg_color=Color::Butter
                            shadow_color=Color::Ink
                            class="w-full"
                        />
                    </div>
                </div>
                <h3 class="font-bold text-3xl my-7">"On commence par toi"</h3>
                <FormView />
            </div>
        </div>
    }
}

#[island]
fn FormView() -> impl IntoView {
    let register = ServerAction::<Register>::new();
    let error = move || register.value().get().and_then(|res| res.err());
    let password = signal(String::new());
    view! {
        <ActionForm action=register>
            <label for="name">"Ton prénom"</label>
            <input
                name="name"
                type="text"
                placeholder="Manon"
                class="mt-2 mb-3"
                required
            />
            <label for="email">"E-mail"</label>
            <input
                name="email"
                type="email"
                placeholder="manon@exemple.fr"
                class="mt-2 mb-3"
                required
            />
            <label for="password">"Mot de passe"</label>
            <input
                name="password"
                type="password"
                placeholder="••••••••"
                class="mt-2 mb-3"
                bind:value=password
                required
            />
            <p
                class="text-right text-raspberry-500"
                class=("hidden", move || password.0.with(|p| p.chars().count()) >= 12)
            >
                "12 char. min."
            </p>
            <label for="validation_password">"Valider le mot de passe"</label>
            <input
                name="validation_password"
                type="password"
                placeholder="••••••••"
                class="mt-2 mb-5"
                required
            />
            <div class="flex my-7">
                <input
                    type="submit"
                    value="Je crée mon compte"
                    class="cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink font-display font-semibold w-full"
                />
            </div>
            {move || {
                error().map(|e| view! { <p class="text-center text-red-600">{e.to_string()}</p> })
            }}
        </ActionForm>
    }
}
