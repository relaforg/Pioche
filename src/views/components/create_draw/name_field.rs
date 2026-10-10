use leptos::prelude::*;

use crate::server::draw_kind::DrawKindDto;

#[component]
pub fn NameField(kind: RwSignal<DrawKindDto>) -> impl IntoView {
    let name_placeholder = move || match kind.get() {
        DrawKindDto::SecretSanta => "Secret Santa - Bureau 2026",
        DrawKindDto::Teams => "Foot de dimanche",
    };

    view! {
        <label for="name">"Nom du tirage"</label>
        <input
            class="mt-2 mb-5"
            id="name"
            name="name"
            type="text"
            placeholder=name_placeholder
            required
        />
    }
}
