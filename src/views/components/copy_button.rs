use leptos::prelude::*;

#[island]
pub fn CopyButton(text: String) -> impl IntoView {
    let (label, set_label) = signal("Copier");

    let copy = move |_| {
        let _ = window().navigator().clipboard().write_text(&text);
        set_label.set("Copié !");
    };

    view! {
        <button
            type="button"
            class="cursor-pointer border rounded-full py-2 px-5 bg-raspberry-500 btn-press-ink"
            on:click=copy
        >
            {label}
        </button>
    }
}
