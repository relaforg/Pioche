use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::{
    server::{
        draws::{AddDraw, DrawKind},
        session::current_user,
    },
    views::{colors::Color, components::button_link::ButtonLink},
};

#[component]
pub fn CreateDrawPage() -> impl IntoView {
    if current_user().is_none() {
        return view! { <Redirect path="/connexion" /> }.into_any();
    }

    view! {
        <div class="max-w-200 mx-auto">
            <div class="flex gap-5 items-center mb-7">
                <ButtonLink
                    link="/mes-tirages"
                    label="← Mes tirages"
                    bg_color=Color::Surface
                    shadow_color=Color::Blue
                />
                <h3 class="text-3xl font-bold text-shadow-butter-3">"Nouveau tirage"</h3>
            </div>
            <FormView />
        </div>
    }
    .into_any()
}

#[island]
fn FormView() -> impl IntoView {
    let add_draw = ServerAction::<AddDraw>::new();
    let kind = RwSignal::new(DrawKind::SecretSanta);
    view! {
        <ActionForm action=add_draw>
            <fieldset class="grid gap-5 sm:grid-cols-2 mb-7">
                <legend class="sr-only">"Type de tirage"</legend>
                <RadioCard
                    kind
                    value=DrawKind::SecretSanta
                    title="Secret Santa"
                    description="Chacun tire une personne à qui offrir, en secret."
                />
                <RadioCard
                    kind
                    value=DrawKind::Teams
                    title="Former des équipes"
                    description="Répartir un groupe en équipes équilibrées."
                />
            </fieldset>
        </ActionForm>
    }
}

#[component]
fn RadioCard(
    kind: RwSignal<DrawKind>,
    value: DrawKind,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <label class="cursor-pointer border rounded-blob p-5 bg-surface has-checked:bg-butter-500 has-checked:btn-press-ink has-checked:text-ink has-focus-visible:outline-2 has-focus-visible:outline-offset-4 has-focus-visible:outline-ink btn-press-blue-500">
            <input
                type="radio"
                name="kind"
                value=value.form_value()
                class="sr-only"
                checked=kind.get_untracked() == value
                on:change=move |_| kind.set(value)
            />
            <h4 class="text-xl">{title}</h4>
            <p class="font-sans font-bold text-sm text-fg-soft has-checked:text-ink">
                {description}
            </p>
        </label>
    }
}
