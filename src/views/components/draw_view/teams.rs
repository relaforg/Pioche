use leptos::prelude::*;

use crate::server::draws::TeamDto;
use crate::server::exclusions::ExclusionDto;
use crate::server::participants::ParticipantDto;
use crate::server::teams_draw::UpdateAndDrawTeams;
use crate::server::teams_options::TeamOptions;
use crate::views::colors::Color;
use crate::views::components::{
    card::Card,
    copy_button::CopyButton,
    create_draw::{
        exclusions_field::ExclusionsField,
        participants_field::{try_add_participant, ParticipantList},
        teams_options::{team_split, TeamsOption},
    },
    draw_view::{
        header::{BackButton, DeleteDrawButton, DrawHeader},
        ACCENT_COLORS,
    },
};

const SHARE_BASE: &str = "pioche.fr/e/";

#[component]
pub fn TeamsDraw(
    name: String,
    share_token: Option<String>,
    participants: Vec<ParticipantDto>,
    exclusions: Vec<ExclusionDto>,
    options: TeamOptions,
    teams: Vec<TeamDto>,
    draw_id: i32,
) -> impl IntoView {
    let participant_count = participants.len();
    let subtitle = format!(
        "{participant_count} participants · {} équipes",
        team_count(participant_count, &options, &teams)
    );
    let names = participants.into_iter().map(|p| p.name).collect();

    view! {
        <div class="flex flex-col gap-8">
            <DrawHeader name=name.clone() subtitle=subtitle.clone() color=Color::Blue>
                <BackButton />
                <DeleteDrawButton draw_name=name />
            </DrawHeader>
            {share_token.map(|token| view! { <ShareLink token /> })}
            <div class="grid gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)] items-start">
                <TeamsSettings participants=names exclusions options draw_id />
                <TeamsResult teams subtitle />
            </div>
        </div>
    }
}

/// Équipes réellement tirées si elles existent, sinon celles prévues par les réglages.
fn team_count(participant_count: usize, options: &TeamOptions, teams: &[TeamDto]) -> usize {
    if !teams.is_empty() {
        return teams.len();
    }
    team_split(
        participant_count,
        options.team_mode,
        options.team_value as usize,
    )
    .map_or(0, |(count, _, _)| count)
}

#[component]
fn ShareLink(token: String) -> impl IntoView {
    view! {
        <Card bg_color=Color::Surface shadow_color=Color::Blue>
            <div class="flex flex-wrap items-center justify-between gap-5">
                <div class="max-w-md">
                    <h3 class="text-xl">"Le lien du tirage"</h3>
                    <p class="mt-1 text-sm">
                        "Partage-le : chacun voit sa composition d'équipe, sans compte ni installation."
                    </p>
                </div>
                <div class="flex flex-1 items-center gap-3 min-w-64">
                    <code class="flex-1 px-4 py-2 bg-neutral-200 border border-dashed rounded-full font-bold truncate">
                        {format!("{SHARE_BASE}{token}")}
                    </code>
                    <CopyButton text=format!("{SHARE_BASE}{token}") />
                </div>
            </div>
        </Card>
    }
}

#[island]
fn TeamsSettings(
    participants: Vec<String>,
    exclusions: Vec<ExclusionDto>,
    options: TeamOptions,
    draw_id: i32,
) -> impl IntoView {
    let participants = RwSignal::new(participants);
    let draft = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);
    let count = move || participants.with(Vec::len);
    let add_participant = move || local_error.set(try_add_participant(participants, draft).err());

    let update_and_draw_team = ServerAction::<UpdateAndDrawTeams>::new();
    let server_error = move || update_and_draw_team.value().get().and_then(Result::err);

    view! {
        <ActionForm action=update_and_draw_team>
            <Card bg_color=Color::Surface shadow_color=Color::Blue>
                <input type="hidden" name="draw_id" value=draw_id />
                <h3 class="text-2xl mb-4">"Réglages du tirage"</h3>
                <label for="participant-draft">"Participants (" {count} ")"</label>
                <ParticipantList participants />
                <div class="flex gap-3">
                    <input
                        id="participant-draft"
                        type="text"
                        placeholder="Ajouter un nom puis Entrée"
                        class="flex-1"
                        bind:value=draft
                        on:input=move |_| local_error.set(None)
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                ev.prevent_default();
                                add_participant();
                            }
                        }
                    />
                    <button
                        type="button"
                        class="cursor-pointer border rounded-full px-5 bg-raspberry-500 btn-press-ink"
                        on:click=move |_| add_participant()
                    >
                        "Ajouter"
                    </button>
                </div>
                {move || {
                    local_error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })
                }}
                <hr class="border-dashed border-t-2 border-line my-7" />
                <ExclusionsField participants initial=exclusions />
                <div class="mt-6">
                    <TeamsOption participant_count=Signal::derive(count) initial=options />
                </div>
                <button
                    type="submit"
                    class="mt-6 w-full cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink"
                >
                    "Mélanger !"
                </button>
                {move || {
                    server_error()
                        .map(|e| {
                            view! {
                                <p class="mt-2 text-raspberry-500 text-center">{e.to_string()}</p>
                            }
                        })
                }}
            </Card>
        </ActionForm>
    }
}

#[component]
pub fn TeamsResult(teams: Vec<TeamDto>, subtitle: String) -> impl IntoView {
    view! {
        <div>
            <div class="flex flex-wrap items-baseline gap-3 mb-4">
                <h3 class="text-2xl">"Résultat du tirage"</h3>
                <p class="text-sm font-bold text-fg-soft">{subtitle}</p>
            </div>
            {if teams.is_empty() {
                view! {
                    <div class="p-7 border border-dashed rounded-blob text-center font-bold text-fg-soft">
                        "Pas encore de tirage : règle les équipes puis mélange !"
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div class="grid gap-5 sm:grid-cols-2 items-start">
                        {teams
                            .into_iter()
                            .zip(ACCENT_COLORS.into_iter().cycle())
                            .map(|(team, color)| view! { <TeamCard team color /> })
                            .collect_view()}
                    </div>
                }
                    .into_any()
            }}
        </div>
    }
}

#[component]
fn TeamCard(team: TeamDto, color: Color) -> impl IntoView {
    let size = team.members.len();
    view! {
        <div class=format!("bg-surface border rounded-blob overflow-hidden {}", color.shadow5())>
            <div
                class=format!(
                    "flex justify-between items-center gap-3 px-5 py-3 border-b {}",
                    color.bg(),
                )
                class=("text-surface", matches!(color, Color::Blue))
            >
                <h4 class="text-xl truncate">{team.name}</h4>
                <span class="shrink-0 px-3 py-0.5 text-sm text-fg bg-surface border rounded-full font-display font-semibold">
                    {size} " pers."
                </span>
            </div>
            <ul class="flex flex-col gap-3 px-5 py-4">
                {team.members.into_iter().map(|m| view! { <Member member=m /> }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn Member(member: ParticipantDto) -> impl IntoView {
    let initial = member
        .name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string());
    view! {
        <li class="flex items-center gap-3 font-bold">
            <span class="grid place-items-center size-8 shrink-0 bg-neutral-200 border rounded-full text-sm">
                {initial}
            </span>
            {member.name}
        </li>
    }
}
