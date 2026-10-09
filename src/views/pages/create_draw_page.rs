use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::{
    server::{
        draws::{parse_name, parse_names, AddDraw, DrawKind, MAX_PARTICIPANTS},
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
    let participants = RwSignal::new(Vec::<String>::new());
    let draft = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);

    let server_error = move || add_draw.value().get().and_then(Result::err);
    let count = move || participants.with(Vec::len);
    let missing = move || kind.get().min_participants().saturating_sub(count());

    let name_placeholder = move || match kind.get() {
        DrawKind::SecretSanta => "Secret Santa - Bureau 2026",
        DrawKind::Teams => "Foot de dimanche",
    };

    let try_add = move || -> Result<(), String> {
        let name = draft.with(|d| parse_name(d)).map_err(|e| e.to_string())?;
        let key = name.to_lowercase();

        if participants.with(|list| list.iter().any(|p| p.to_lowercase() == key)) {
            return Err(format!("'{name}' est déjà dans la liste"));
        }
        if count() >= MAX_PARTICIPANTS {
            return Err(format!("{MAX_PARTICIPANTS} participants maximum"));
        }
        participants.update(|list| list.push(name));
        draft.set(String::new());
        Ok(())
    };
    let add = move || local_error.set(try_add().err());

    let dialog = NodeRef::<leptos::html::Dialog>::new();

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
            <div class="bg-surface border shadow-butter-3 p-5 rounded-blob">
                <label for="name">"Nom du tirage"</label>
                <input
                    class="mt-2 mb-5"
                    id="name"
                    name="name"
                    type="text"
                    placeholder=name_placeholder
                    required
                />

                <div class="flex items-baseline gap-2">
                    <label for="participant-draft">"Participants"</label>
                    <span class="font-bold text-fg-soft">{count} " dans la liste"</span>
                </div>
                <ul class="flex flex-wrap content-start gap-2 min-h-32 mt-2 mb-3 p-3 bg-neutral-200 border rounded-blob">
                    {move || {
                        participants
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, name)| {
                                view! {
                                    <li class="flex items-center gap-2 h-fit px-3 py-1 bg-surface border rounded-full font-bold">
                                        {name.clone()}
                                        <input
                                            type="hidden"
                                            name=format!("participants[{i}]")
                                            value=name.clone()
                                        />
                                        <button
                                            type="button"
                                            class="cursor-pointer text-raspberry-500"
                                            aria-label=format!("Retirer {name}")
                                            on:click=move |_| {
                                                participants
                                                    .update(|list| {
                                                        list.remove(i);
                                                    })
                                            }
                                        >
                                            "x"
                                        </button>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <div class="flex gap-3">
                    <input
                        id="participant-draft"
                        type="text"
                        placeholder="Prénom puis Entrée"
                        class="flex-1"
                        bind:value=draft
                        on:input=move |_| local_error.set(None)
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                ev.prevent_default();
                                add();
                            }
                        }
                    />
                    <button
                        type="button"
                        class="cursor-pointer border rounded-full px-5 bg-raspberry-500 btn-press-ink"
                        on:click=move |_| add()
                    >
                        "Ajouter"
                    </button>
                    <button
                        type="button"
                        class="cursor-pointer border rounded-full px-5 bg-surface btn-press-blue-500"
                        on:click=move |_| {
                            if let Some(d) = dialog.get() {
                                let _ = d.show_modal();
                            }
                        }
                    >
                        "Coller une liste"
                    </button>
                </div>
                {move || {
                    local_error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })
                }}
            </div>
            <div class="flex flex-col items-center gap-2 my-7">
                <input
                    type="submit"
                    value="Créer le tirage"
                    class="cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink font-display font-semibold w-full disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=move || { missing() > 0 || add_draw.pending().get() }
                />
                <Show when=move || { missing() > 0 }>
                    <p class="text-fg-soft">"Encore " {missing} " participant(s) minimum"</p>
                </Show>
            </div>
            {move || {
                server_error()
                    .map(|e| view! { <p class="text-center text-red-600">{e.to_string()}</p> })
            }}
        </ActionForm>
        <PasteDialog dialog participants />
    }
}

#[component]
fn PasteDialog(
    dialog: NodeRef<leptos::html::Dialog>,
    participants: RwSignal<Vec<String>>,
) -> impl IntoView {
    let text = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    let close = move || {
        if let Some(d) = dialog.get() {
            d.close();
        }
    };

    let try_paste = move || -> Result<(), String> {
        let merged = participants
            .with(|current| {
                text.with(|t| {
                    let pasted = t.split(['\n', ',', ';']).filter(|s| !s.trim().is_empty());
                    parse_names(current.iter().map(String::as_str).chain(pasted))
                })
            })
            .map_err(|e| e.to_string())?;
        participants.set(merged);
        text.set(String::new());
        Ok(())
    };

    let submit = move |_| match try_paste() {
        Ok(()) => close(),
        Err(e) => error.set(Some(e)),
    };
    view! {
        <dialog
            node_ref=dialog
            class="m-auto w0full max-w-110 p-7 bg-surface text-fg border rounded-blob shadow-butter-5 backdrop:bg-ink/50"
            on:close=move |_| error.set(None)
        >
            <h3 class="font-bold text-2xl mb-5">"Coller une liste"</h3>
            <label for="paste-list">"Un prénom par ligne, ou séparés par des virgules"</label>
            <textarea
                id="paste-list"
                rows="8"
                placeholder="Léa\nTom\nAmir"
                class="w-full mt-2 p-3 bg-neutral-200 border rounded-blob font-bold resize-none"
                autofocus
                bind:value=text
                on:input=move |_| error.set(None)
            />
            {move || error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })}
            <div class="flex justify-end gap-3 mt-5">
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-surface btn-press-blue-500"
                    on:click=move |_| close()
                >
                    "Annuler"
                </button>
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-surface btn-press-raspberry-500"
                    on:click=submit
                >
                    "Ajouter à la liste"
                </button>
            </div>
        </dialog>
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
