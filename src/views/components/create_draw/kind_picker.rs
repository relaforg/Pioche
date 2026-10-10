use leptos::prelude::*;

use crate::server::draws::DrawKind;

#[component]
pub fn KindPicker(kind: RwSignal<DrawKind>) -> impl IntoView {
    view! {
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
