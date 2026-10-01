use leptos::prelude::*;

use crate::{
    server::auth::Connect,
    views::{colors::Color, components::button_link::ButtonLink},
};

#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <div class="max-w-110 border shadow-blue-5 rounded-blob bg-surface">
            <div class="p-7">
                <div class="flex gap-3 mb-7">
                    <div class="flex-1">
                        <ButtonLink
                            label="Connexion"
                            link="/connexion"
                            bg_color=Color::Butter
                            shadow_color=Color::Ink
                            class="w-full"
                        />
                    </div>
                    <div class="flex-1">
                        <ButtonLink
                            label="Créer un compte"
                            link="/inscription"
                            bg_color=Color::Surface
                            shadow_color=Color::Blue
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
    let login = ServerAction::<Connect>::new();
    let error = move || login.value().get().and_then(|res| res.err());
    view! {
        <ActionForm action=login>
            <label for="email">"E-mail"</label>
            <input
                name="email"
                type="email"
                placeholder="manon@exemple.fr"
                class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-3"
                required
            />
            <label for="password">"Mot de passe"</label>
            <input
                name="password"
                type="password"
                placeholder="••••••••"
                class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-3"
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
