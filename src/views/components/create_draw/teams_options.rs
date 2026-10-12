use leptos::prelude::*;

use crate::server::team_mode::TeamModeDto;

#[component]
pub fn TeamsOption(participant_count: Signal<usize>) -> impl IntoView {
    let mode = RwSignal::new(TeamModeDto::Count);
    let max = move || (participant_count.get() / 2).max(2);
    let value = RwSignal::new(2);
    view! {
        <h3 class="font-semibold text-lg">"Je définis..."</h3>
        <fieldset class="grid gap-5 sm:grid-cols-2 my-2">
            <RadioField mode value=TeamModeDto::Count label="Nombre d'équipes" />
            <RadioField mode value=TeamModeDto::Size label="Taille des équipes" />
        </fieldset>
        <div class="flex items-center gap-4 mt-4">
            <input type="range" min="2" prop:max=max prop:value=value on:input:target=move |ev| {
                if let Ok(v) = ev.target().value().parse::<usize>() {
                    value.set(v);
                };
            } style:--fill=move || format!("{}%", ((value.get() - 2) * 100).checked_div(max() - 2).unwrap_or(5)) />
            <output class="grid place-items-center size-14 rounded-chip bg-butter-500 border shadow-ink-3 font-display font-bold text-2xl">
                {value}
            </output>
        </div>
    }
}

#[component]
fn RadioField(
    mode: RwSignal<TeamModeDto>,
    value: TeamModeDto,
    label: &'static str,
) -> impl IntoView {
    view! {
        <label class="cursor-pointer btn-press-blue-500 p-2 rounded-full bg-neutral-200 border has-checked:bg-butter-500 has-checked:btn-press-ink">
            <input
                type="radio"
                name="team_mode"
                value=value.form_value()
                class="sr-only"
                checked=mode.get_untracked() == value
                on:change=move |_| mode.set(value)
            />
            <h3 class="font-semibold text-center text-base">{label}</h3>
        </label>
    }
}
