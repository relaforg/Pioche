use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    server::auth::Logout,
    server::session::{current_user, CurrentUser},
    views::{colors::Color, components::button_link::ButtonLink},
};

#[component]
pub fn Header() -> impl IntoView {
    let user = current_user();
    view! {
        <div class="flex justify-between mt-5 mb-15">
            <A href="/" attr:class="flex gap-3 items-center">
                <Logo />
                <h2 class="text-3xl text-shadow-blue-3">"Pioche !"</h2>
            </A>
            {match user {
                Some(user) => {
                    view! {
                        <div class="flex gap-5 items-center">
                            <ButtonLink
                                link=""
                                label="+ Nouveau tirage"
                                bg_color=Color::Raspberry
                                shadow_color=Color::Ink
                            />
                            <UserMenu user=user />
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="flex gap-3 items-center">
                            <ButtonLink
                                link="/connexion"
                                label="Se connecter"
                                bg_color=Color::Surface
                                shadow_color=Color::Blue
                            />
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="size-13 bg-butter-500 rounded-logo border border-line shadow-ink-4 grid place-items-center">
            <div class="size-5 bg-blue-500 rounded-full border border-line"></div>
        </div>
    }
}

#[island]
fn UserMenu(user: CurrentUser) -> impl IntoView {
    let (open, set_open) = signal(false);
    let logout = ServerAction::<Logout>::new();

    view! {
        <div class="relative">
            <button
                on:click=move |_| set_open.update(|o| *o = !*o)
                class="cursor-pointer border border-line text-surface bg-blue-500 btn-press-ink rounded-full size-12 grid place-items-center text-lg"
            >
                {user.name.to_uppercase().chars().next().unwrap_or('?')}
            </button>
            <Show when=move || open.get()>
                <div class="fixed inset-0 z-9" on:click=move |_| set_open.set(false)></div>
                <div class="absolute right-0 top-full mt-2 border w-60 bg-surface rounded-blob shadow-ink-5 z-10">
                    <div class="pt-4 px-5">
                        <h4 class="text-lg">{user.name.clone()}</h4>
                        <p class="text-sm">{user.email.clone()}</p>
                    </div>

                    <div class="p-2">
                        <hr class="border-dashed border-t-2 border-line" />
                        <a
                            href=""
                            class="block font-display font-semibold text-left px-4 py-2 border border-transparent cursor-pointer my-1 w-full rounded-logo hover:border-ink hover:bg-butter-500"
                        >
                            "Mes tirages"
                        </a>
                        <ActionForm action=logout>
                            <button
                                class="block font-display font-semibold text-left px-4 py-2 border border-transparent cursor-pointer w-full rounded-logo hover:border-ink hover:bg-neutral-200"
                                type="submit"
                            >
                                "Se déconnecter"
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </Show>
        </div>
    }
}
