use leptos::prelude::*;
use leptos_router::components::Form;

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
                <Form method="POST" action="">
                    <label for="name">"Ton prénom"</label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Manon"
                        class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-3"
                    />
                    <label for="email">"E-mail"</label>
                    <input
                        id="email"
                        type="email"
                        placeholder="manon@exemple.fr"
                        class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-3"
                    />
                    <label for="mdp1">"Mot de passe"</label>
                    <input
                        id="mdp1"
                        type="password"
                        placeholder="••••••••"
                        class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-3"
                    />
                    <label for="mdp2">"Valider mot de passe"</label>
                    <input
                        id="mdp2"
                        type="password"
                        placeholder="••••••••"
                        class="bg-neutral-200 w-full border rounded-full p-3 font-bold mt-2 mb-5"
                    />
                    <div class="flex">
                        <input
                            type="submit"
                            value="Je crée mon compte"
                            class="cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink font-display font-semibold w-full"
                        />
                    </div>
                </Form>
            </div>
        </div>
    }
}
