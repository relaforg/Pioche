use leptos::prelude::*;

#[component]
pub fn ExclusionsField(participants: RwSignal<Vec<String>>) -> impl IntoView {
    let select_a = RwSignal::new(String::new());
    let select_b = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);

    let exclusions = RwSignal::new(Vec::<(String, String)>::new());
    let try_add_exclusion = move || -> Result<(), String> {
        let a = select_a.get();
        let b = select_b.get();

        if a.is_empty() || b.is_empty() {
            return Err("Choisissez deux participants à séparer".into());
        }
        if a == b {
            return Err("Choisissez deux participants différents".into());
        }
        if exclusions.with(|list| {
            list.iter()
                .any(|e| (e.0 == a && e.1 == b) || (e.0 == b && e.1 == a))
        }) {
            return Err(format!("'{a}' et '{b}' sont déjà exclus l'un de l'autre"));
        }

        exclusions.update(|list| list.push((a, b)));
        Ok(())
    };

    let add_exclusion = move || local_error.set(try_add_exclusion().err());

    view! {
        <h3 class="font-semibold text-lg">"Exclusions — qui ne doit pas tomber ensemble"</h3>
        <p class="font-semibold text-fg-soft">
            "Pratique pour les couples, les colocs, ou ceux qui se sont déjà offert l'an dernier."
        </p>
        <div class="flex gap-3 my-2">
            <select bind:value=select_a class="flex-1" on:change=move |_| local_error.set(None)>
                <option value="">"Choisir..."</option>
                <ParticipantOptions participants other=select_b />
            </select>
            <p class="font-display text-xl self-center">"x"</p>
            <select bind:value=select_b class="flex-1" on:change=move |_| local_error.set(None)>
                <option value="">"Choisir..."</option>
                <ParticipantOptions participants other=select_a />
            </select>
            <button
                type="button"
                class="cursor-pointer border rounded-full px-5 bg-surface btn-press-blue-500"
                on:click=move |_| add_exclusion()
            >
                "+ Exclure"
            </button>
        </div>
        {move || {
            local_error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })
        }}
        <ExclusionList exclusions />
    }
}

#[component]
fn ExclusionList(exclusions: RwSignal<Vec<(String, String)>>) -> impl IntoView {
    view! {
        <ul class="flex flex-wrap content-start gap-2 mt-2 mb-3">
            {move || {
                exclusions
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, (a, b))| {
                        view! {
                            <li class="flex items-center gap-2 h-fit px-3 py-1 border rounded-full font-bold bg-butter-500 shadow-ink-3">
                                {format!("{a} x {b}")}
                                <input
                                    type="hidden"
                                    name=format!("exclusions[{i}][a]")
                                    value=a.clone()
                                />
                                <input
                                    type="hidden"
                                    name=format!("exclusions[{i}][b]")
                                    value=b.clone()
                                />
                                <button
                                    type="button"
                                    class="cursor-pointer text-raspberry-500"
                                    aria-label=format!("Retirer {a} x {b}")
                                    on:click=move |_| {
                                        exclusions
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
    }
}

#[component]
fn ParticipantOptions(
    participants: RwSignal<Vec<String>>,
    other: RwSignal<String>,
) -> impl IntoView {
    move || {
        participants
            .get()
            .into_iter()
            .map(|p| {
                view! {
                    <option value=p.clone() disabled=move || other.get() == p>
                        {p.clone()}
                    </option>
                }
            })
            .collect_view()
    }
}
