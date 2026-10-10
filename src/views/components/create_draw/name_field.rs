use leptos::prelude::*;

use crate::server::draws::DrawKind;

#[component]
pub fn NameField(kind: RwSignal<DrawKind>) -> impl IntoView {
    let name_placeholder = move || match kind.get() {
        DrawKind::SecretSanta => "Secret Santa - Bureau 2026",
        DrawKind::Teams => "Foot de dimanche",
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
