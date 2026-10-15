use leptos::prelude::*;

use crate::views::{colors::Color, components::button_link::ButtonLink};

#[component]
pub fn DrawHeader(
    name: String,
    subtitle: String,
    color: Color,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!(
                "flex flex-wrap justify-between items-center gap-5 p-7 border border-line rounded-blob shadow-ink-5 {}",
                color.bg(),
            )
            class=("text-surface", matches!(color, Color::Blue))
        >
            <div>
                <h2 class="text-4xl">{name}</h2>
                <p class="mt-1 font-bold">{subtitle}</p>
            </div>
            <div class="flex gap-3 text-fg">{children()}</div>
        </div>
    }
}

#[island]
pub fn DeleteDrawButton(draw_name: String) -> impl IntoView {
    let dialog = NodeRef::<leptos::html::Dialog>::new();
    let close = move || {
        if let Some(d) = dialog.get() {
            d.close();
        }
    };

    view! {
        <button
            type="button"
            class="cursor-pointer border rounded-full py-2 px-4 bg-surface text-raspberry-600 btn-press-ink"
            on:click=move |_| {
                if let Some(d) = dialog.get() {
                    let _ = d.show_modal();
                }
            }
        >
            "Supprimer"
        </button>
        <dialog
            node_ref=dialog
            class="m-auto w-full max-w-110 p-7 bg-surface text-fg border rounded-blob shadow-raspberry-5 backdrop:bg-ink/50"
        >
            <h3 class="font-bold text-2xl mb-3">"Supprimer ce tirage ?"</h3>
            <p>
                "« " <strong>{draw_name}</strong>
                " » sera supprimé avec ses participants et ses résultats. C'est définitif."
            </p>
            <div class="flex justify-end gap-3 mt-6">
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-surface btn-press-blue-500"
                    on:click=move |_| close()
                >
                    "Annuler"
                </button>
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-raspberry-500 btn-press-ink"
                >
                    "Supprimer définitivement"
                </button>
            </div>
        </dialog>
    }
}

#[component]
pub fn BackButton() -> impl IntoView {
    view! {
        <ButtonLink
            link="/mes-tirages"
            label="← Mes tirages"
            bg_color=Color::Surface
            shadow_color=Color::Ink
        />
    }
}
