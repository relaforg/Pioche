use std::cmp::Ordering;

use chrono::{DateTime, FixedOffset, NaiveDate};
use leptos::prelude::*;

use crate::server::exclusions::ExclusionDto;
use crate::server::participants::{ParticipantDto, UpdateParticipantEmail};
use crate::server::santa_draw::{DrawSanta, SendSantaLinks};
use crate::server::santa_edit::{AddSantaParticipant, RemoveSantaParticipant, UpdateSantaSettings};
use crate::server::secret_santa_option::SecretSantaOptions;
use crate::views::colors::Color;
use crate::views::components::{
    card::Card,
    create_draw::exclusions_field::ExclusionsField,
    draw_view::{
        header::{BackButton, DeleteDrawButton, DrawHeader},
        ACCENT_COLORS,
    },
};

/// Étapes du parcours ; la valeur est l'index de l'étape dans la frise.
#[derive(Clone, Copy, PartialEq)]
enum SantaStep {
    /// Liste et réglages modifiables, jusqu'au tirage.
    Participants = 0,
    /// Tirage fait : on saisit les emails, jusqu'à l'envoi des liens.
    Emails = 1,
    /// Liens envoyés : tout est figé, on attend l'échange.
    Exchange = 2,
}

fn santa_step(is_drawn: bool, links_sent: bool) -> SantaStep {
    match (is_drawn, links_sent) {
        (false, _) => SantaStep::Participants,
        (true, false) => SantaStep::Emails,
        (true, true) => SantaStep::Exchange,
    }
}

#[component]
pub fn SecretSantaDraw(
    name: String,
    participants: Vec<ParticipantDto>,
    exclusions: Vec<ExclusionDto>,
    options: SecretSantaOptions,
    drawn_at: Option<DateTime<FixedOffset>>,
    links_sent_at: Option<DateTime<FixedOffset>>,
    draw_id: i32,
) -> impl IntoView {
    let step = santa_step(drawn_at.is_some(), links_sent_at.is_some());
    let participant_count = participants.len();
    let completed = participants
        .iter()
        .filter(|p| p.form_completed_at.is_some())
        .count();
    let missing_emails = participants.iter().filter(|p| p.email.is_none()).count();
    let names = participants.iter().map(|p| p.name.clone()).collect();

    // Ordre alphabétique stable d'une étape à l'autre, sans tenir compte de la casse.
    let mut participants = participants;
    participants.sort_by_cached_key(|p| p.name.to_lowercase());

    let subtitle = [
        Some(plural(participant_count, "participant", "participants")),
        options
            .budget_cents
            .map(|c| format!("budget {}", format_euros(c))),
        options
            .exchange_date
            .map(|d| format!("échange le {}", format_day(d))),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ");

    let step_card = match step {
        SantaStep::Participants => view! {
            <StepCard
                step
                title="Prépare ton tirage"
                action=view! { <LaunchDrawButton draw_id /> }.into_any()
            >
                <p class="text-sm">
                    "Ajoute ou retire des participants et ajuste les réglages. Quand tout est prêt, lance le tirage : la liste sera alors figée."
                </p>
            </StepCard>
        }
        .into_any(),
        SantaStep::Emails => view! {
            <StepCard
                step
                title="Renseigne les emails"
                action=view! { <SendLinksButton draw_id ready=missing_emails == 0 /> }.into_any()
            >
                <p class="text-sm">
                    "Chacun recevra son lien perso pour découvrir son binôme et remplir sa fiche. Une fois les liens envoyés, les emails ne sont plus modifiables."
                </p>
                <ProgressBar
                    done=participant_count - missing_emails
                    total=participant_count
                    label="emails renseignés"
                />
            </StepCard>
        }
        .into_any(),
        SantaStep::Exchange => view! {
            <StepCard step title=countdown(options.exchange_date) action=().into_any()>
                <p class="text-sm">
                    "Les liens sont partis, il n'y a plus qu'à attendre. Tu ne vois jamais qui offre à qui."
                </p>
                <ProgressBar done=completed total=participant_count label="fiches remplies" />
            </StepCard>
        }
        .into_any(),
    };

    let participant_cards = participants
        .into_iter()
        .zip(ACCENT_COLORS.into_iter().cycle())
        .map(|(participant, color)| view! { <SantaParticipantCard participant color step /> })
        .collect_view();

    view! {
        <div class="flex flex-col gap-8">
            <DrawHeader name=name.clone() subtitle color=Color::Raspberry>
                <BackButton />
                <DeleteDrawButton draw_name=name draw_id />
            </DrawHeader>
            {step_card}
            {match step {
                SantaStep::Participants => {
                    view! {
                        <div class="grid gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.3fr)] items-start">
                            <SettingsForm
                                participants=names
                                exclusions
                                budget_cents=options.budget_cents
                                exchange_date=options.exchange_date
                                draw_id
                            />
                            <ParticipantsSection hint="La liste sera figée au lancement du tirage">
                                // `contents` : le formulaire de l'island devient l'élément de grille.
                                <div class="grid gap-4 sm:grid-cols-2 [&>leptos-island]:contents">
                                    {participant_cards} <AddParticipantCard draw_id />
                                </div>
                            </ParticipantsSection>
                        </div>
                    }
                        .into_any()
                }
                _ => {
                    let hint = if step == SantaStep::Emails {
                        "Un email par participant, pour lui envoyer son lien perso"
                    } else {
                        "Tu ne vois jamais qui offre à qui"
                    };
                    view! {
                        <ParticipantsSection hint>
                            <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-4">
                                {participant_cards}
                            </div>
                        </ParticipantsSection>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

/// En français, 0 et 1 prennent le singulier.
fn plural(n: usize, singular: &str, plural: &str) -> String {
    format!("{n} {}", if n > 1 { plural } else { singular })
}

fn format_euros(cents: i32) -> String {
    match cents % 100 {
        0 => format!("{} €", cents / 100),
        rest => format!("{},{rest:02} €", cents / 100),
    }
}

fn format_day(date: NaiveDate) -> String {
    date.format_localized("%-d %B", chrono::Locale::fr_FR)
        .to_string()
}

fn countdown(exchange_date: Option<NaiveDate>) -> String {
    let Some(date) = exchange_date else {
        return "Place à l'échange !".into();
    };
    let today = chrono::Utc::now().date_naive();
    match (date - today).num_days() {
        days if days > 1 => format!("J-{days} avant l'échange du {}", format_day(date)),
        1 => "L'échange, c'est demain !".into(),
        0 => "L'échange, c'est aujourd'hui !".into(),
        _ => format!("L'échange a eu lieu le {}", format_day(date)),
    }
}

/// Frise des 3 étapes, puis ce qu'il y a à faire dans l'étape courante.
#[component]
fn StepCard(
    step: SantaStep,
    #[prop(into)] title: String,
    action: AnyView,
    children: Children,
) -> impl IntoView {
    view! {
        <Card bg_color=Color::Surface shadow_color=Color::Raspberry>
            <ol class="flex flex-wrap gap-2 mb-6">
                <StepPill label="Participants" index=0 current=step as u8 />
                <StepPill label="Emails" index=1 current=step as u8 />
                <StepPill label="Échange" index=2 current=step as u8 />
            </ol>
            <div class="flex flex-wrap items-end justify-between gap-5">
                <div class="flex flex-col gap-3 flex-1 min-w-64">
                    <h3 class="text-2xl">{title}</h3>
                    {children()}
                </div>
                {action}
            </div>
        </Card>
    }
}

#[component]
fn StepPill(label: &'static str, index: u8, current: u8) -> impl IntoView {
    let (mark, class) = match index.cmp(&current) {
        Ordering::Less => ("✓".to_string(), "bg-blue-200"),
        Ordering::Equal => ((index + 1).to_string(), "bg-butter-500 shadow-ink-2"),
        Ordering::Greater => (
            (index + 1).to_string(),
            "bg-surface border-dashed text-fg-soft",
        ),
    };
    view! {
        <li
            class=format!(
                "flex items-center gap-2 px-3 py-1 border rounded-full text-sm font-extrabold {class}",
            )
            aria-current=(index == current).then_some("step")
        >
            <span>{mark}</span>
            {label}
        </li>
    }
}

#[component]
fn ProgressBar(done: usize, total: usize, label: &'static str) -> impl IntoView {
    let percent = (done * 100).checked_div(total).unwrap_or(0);
    view! {
        <div>
            <p class="text-sm font-bold">{done} " / " {total} " " {label}</p>
            <div
                class="mt-2 h-4 bg-neutral-200 border rounded-full overflow-hidden"
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax=total
                aria-valuenow=done
            >
                <div class="h-full bg-blue-500" style:width=format!("{percent}%") />
            </div>
        </div>
    }
}

#[component]
fn ParticipantsSection(hint: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <div class="flex flex-wrap items-baseline gap-3 mb-5">
                <h3 class="text-2xl">"Participants"</h3>
                <p class="text-sm font-bold text-fg-soft">{hint}</p>
            </div>
            {children()}
        </div>
    }
}

#[island]
fn LaunchDrawButton(draw_id: i32) -> impl IntoView {
    let draw_santa = ServerAction::<DrawSanta>::new();
    let server_error = move || draw_santa.value().get().and_then(Result::err);

    view! {
        <ActionForm action=draw_santa>
            <input type="hidden" name="draw_id" value=draw_id />
            <button
                type="submit"
                class="cursor-pointer border rounded-full py-3 px-7 bg-raspberry-500 btn-press-ink"
                disabled=move || draw_santa.pending().get()
            >
                "Lancer le tirage"
            </button>
            {move || {
                server_error()
                    .map(|e| view! { <p class="mt-2 text-raspberry-500">{e.to_string()}</p> })
            }}
        </ActionForm>
    }
}

#[island]
fn SendLinksButton(draw_id: i32, ready: bool) -> impl IntoView {
    let send_links = ServerAction::<SendSantaLinks>::new();
    let server_error = move || send_links.value().get().and_then(Result::err);

    view! {
        <ActionForm action=send_links>
            <input type="hidden" name="draw_id" value=draw_id />
            <button
                type="submit"
                class="border rounded-full py-3 px-7"
                class=("cursor-pointer bg-raspberry-500 btn-press-ink", ready)
                class=("cursor-not-allowed bg-neutral-200 text-fg-soft border-dashed", !ready)
                disabled=move || !ready || send_links.pending().get()
                title=(!ready).then_some("Renseigne d'abord tous les emails")
            >
                "Envoyer les liens"
            </button>
            {move || {
                server_error()
                    .map(|e| view! { <p class="mt-2 text-raspberry-500">{e.to_string()}</p> })
            }}
        </ActionForm>
    }
}

/// Budget, date et exclusions, modifiables jusqu'au tirage.
#[island]
fn SettingsForm(
    participants: Vec<String>,
    exclusions: Vec<ExclusionDto>,
    budget_cents: Option<i32>,
    exchange_date: Option<NaiveDate>,
    draw_id: i32,
) -> impl IntoView {
    let participants = RwSignal::new(participants);
    let update_settings = ServerAction::<UpdateSantaSettings>::new();
    let server_error = move || update_settings.value().get().and_then(Result::err);

    view! {
        <ActionForm action=update_settings>
            <Card bg_color=Color::Surface shadow_color=Color::Blue>
                <input type="hidden" name="draw_id" value=draw_id />
                <h3 class="text-2xl mb-4">"Réglages"</h3>
                <div class="flex flex-wrap gap-5">
                    <div class="flex-1 min-w-40">
                        <label for="budget">"Budget cadeau"</label>
                        <input
                            class="mt-2"
                            id="budget"
                            name="budget"
                            type="number"
                            placeholder="25 €"
                            min=0
                            step=0.01
                            value=budget_cents.map(|c| format!("{}.{:02}", c / 100, c % 100))
                            required
                        />
                    </div>
                    <div class="flex-1 min-w-40">
                        <label for="date">"Date de l'échange"</label>
                        <input
                            class="mt-2"
                            id="date"
                            name="date"
                            type="date"
                            value=exchange_date.map(|d| d.to_string())
                            required
                        />
                    </div>
                </div>
                <hr class="border-dashed border-t-2 border-line my-6" />
                <ExclusionsField participants initial=exclusions />
                <button
                    type="submit"
                    class="mt-4 w-full cursor-pointer border rounded-full py-3 bg-surface btn-press-blue-500"
                    disabled=move || update_settings.pending().get()
                >
                    "Enregistrer les réglages"
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
fn SantaParticipantCard(
    participant: ParticipantDto,
    color: Color,
    step: SantaStep,
) -> impl IntoView {
    let initial = participant
        .name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string());

    let body = match step {
        SantaStep::Participants => None,
        SantaStep::Emails => Some(
            view! { <EmailField participant_id=participant.id email=participant.email /> }
                .into_any(),
        ),
        SantaStep::Exchange => {
            let (status, status_class) = match participant.form_completed_at {
                Some(at) => (
                    format!(
                        "✓ Fiche remplie le {}",
                        at.format_localized("%-d %B", chrono::Locale::fr_FR)
                    ),
                    "bg-blue-200",
                ),
                None => ("En attente de sa fiche".to_string(), "bg-butter-200"),
            };
            Some(
                view! {
                    <span class=format!(
                        "w-fit px-3 py-0.5 text-sm border rounded-full font-extrabold {status_class}",
                    )>{status}</span>
                    <p class="text-sm truncate text-fg-soft">{participant.email}</p>
                }
                    .into_any(),
            )
        }
    };

    view! {
        <div class="flex flex-col gap-3 p-4 bg-surface border rounded-blob shadow-raspberry-5">
            <div class="flex items-center gap-3">
                <span
                    class=format!(
                        "grid place-items-center size-9 shrink-0 border rounded-full font-bold {}",
                        color.bg(),
                    )
                    class=("text-surface", matches!(color, Color::Blue))
                >
                    {initial}
                </span>
                <h4 class="flex-1 min-w-0 text-lg truncate">{participant.name.clone()}</h4>
                {(step == SantaStep::Participants)
                    .then(|| {
                        view! {
                            <RemoveParticipantButton
                                participant_id=participant.id
                                name=participant.name
                            />
                        }
                    })}
            </div>
            {body
                .map(|body| {
                    view! {
                        <hr class="border-t-2 border-dotted border-line" />
                        {body}
                    }
                })}
        </div>
    }
}

#[island]
fn RemoveParticipantButton(participant_id: i32, name: String) -> impl IntoView {
    let remove = ServerAction::<RemoveSantaParticipant>::new();
    let server_error = move || remove.value().get().and_then(Result::err);

    view! {
        <div class="flex flex-col items-end">
            <ActionForm action=remove>
                <input type="hidden" name="participant_id" value=participant_id />
                <button
                    type="submit"
                    class="grid border place-items-center size-8 cursor-pointer rounded-full text-2xl leading-none text-fg-soft hover:bg-raspberry-100 hover:text-raspberry-600"
                    aria-label=format!("Retirer {name}")
                    title=format!("Retirer {name}")
                    disabled=move || remove.pending().get()
                >
                    "×"
                </button>
            </ActionForm>
            {move || {
                server_error()
                    .map(|e| {
                        view! {
                            <p class="mt-1 text-xs text-right text-raspberry-500">
                                {e.to_string()}
                            </p>
                        }
                    })
            }}
        </div>
    }
}

#[island]
fn AddParticipantCard(draw_id: i32) -> impl IntoView {
    let add = ServerAction::<AddSantaParticipant>::new();
    let server_error = move || add.value().get().and_then(Result::err);

    view! {
        <ActionForm action=add>
            // Même hauteur qu'une carte participant : pastille + une ligne.
            <div class="flex flex-col justify-center gap-2 h-full p-4 border border-dashed rounded-blob">
                <input type="hidden" name="draw_id" value=draw_id />
                <div class="flex items-center gap-3">
                    <span class="grid place-items-center size-9 shrink-0 border border-dashed rounded-full font-bold">
                        "+"
                    </span>
                    <input
                        name="name"
                        type="text"
                        placeholder="Prénom"
                        aria-label="Prénom"
                        class="min-w-0 py-1.5 bg-surface"
                        required
                    />
                    <button
                        type="submit"
                        class="shrink-0 cursor-pointer border rounded-full px-4 py-1.5 bg-surface btn-press-blue-500"
                        disabled=move || add.pending().get()
                    >
                        "OK"
                    </button>
                </div>
                {move || {
                    server_error()
                        .map(|e| {
                            view! { <p class="text-sm text-raspberry-500">{e.to_string()}</p> }
                        })
                }}
            </div>
        </ActionForm>
    }
}

#[island]
fn EmailField(participant_id: i32, email: Option<String>) -> impl IntoView {
    let save_email = ServerAction::<UpdateParticipantEmail>::new();
    let server_error = move || save_email.value().get().and_then(Result::err);
    let input_id = format!("email-{participant_id}");
    let missing = email.is_none();

    view! {
        <ActionForm action=save_email>
            <input type="hidden" name="participant_id" value=participant_id />
            <div class="flex items-center justify-between gap-2">
                <label for=input_id.clone() class="text-base">
                    "Email"
                </label>
                {missing
                    .then(|| {
                        view! {
                            <span class="px-2 py-0.5 text-xs border rounded-full font-extrabold bg-butter-200">
                                "À renseigner"
                            </span>
                        }
                    })}
            </div>
            <div class="flex gap-2 mt-2">
                <input
                    id=input_id
                    name="email"
                    type="email"
                    placeholder="prenom@exemple.fr"
                    value=email
                    class="min-w-0 bg-surface"
                    required
                />
                <button
                    type="submit"
                    class="shrink-0 cursor-pointer border rounded-full px-4 bg-surface btn-press-blue-500"
                    disabled=move || save_email.pending().get()
                >
                    "OK"
                </button>
            </div>
            {move || {
                server_error()
                    .map(|e| {
                        view! { <p class="mt-2 text-sm text-raspberry-500">{e.to_string()}</p> }
                    })
            }}
        </ActionForm>
    }
}
