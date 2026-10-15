use leptos::prelude::*;

use crate::server::participants::ParticipantDto;
use crate::server::secret_santa_option::SecretSantaOptions;
use crate::views::colors::Color;
use crate::views::components::{
    card::Card,
    draw_view::{
        header::{BackButton, DeleteDrawButton, DrawHeader},
        ACCENT_COLORS,
    },
};

#[component]
pub fn SecretSantaDraw(
    name: String,
    participants: Vec<ParticipantDto>,
    options: SecretSantaOptions,
) -> impl IntoView {
    let participant_count = participants.len();
    let completed = participants
        .iter()
        .filter(|p| p.form_completed_at.is_some())
        .count();

    let subtitle = [
        Some(format!("{participant_count} participants")),
        options
            .budget_cents
            .map(|c| format!("budget {}", format_euros(c))),
        options.exchange_date.map(|d| {
            format!(
                "échange le {}",
                d.format_localized("%-d %B", chrono::Locale::fr_FR)
            )
        }),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ");

    view! {
        <div class="flex flex-col gap-8">
            <DrawHeader name=name.clone() subtitle color=Color::Raspberry>
                <BackButton />
                <DeleteDrawButton draw_name=name />
            </DrawHeader>
            <PersonalLinks missing_emails=participant_count />
            <div>
                <div class="flex flex-wrap items-baseline gap-3 mb-4">
                    <h3 class="text-2xl">"Participants"</h3>
                    <p class="text-sm font-bold text-fg-soft">
                        {completed} "/" {participant_count}
                        " fiches remplies · tu ne vois jamais qui offre à qui"
                    </p>
                </div>
                <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-4">
                    {participants
                        .into_iter()
                        .zip(ACCENT_COLORS.into_iter().cycle())
                        .map(|(participant, color)| {
                            view! { <SantaParticipantCard participant color /> }
                        })
                        .collect_view()}
                </div>
            </div>
            <div class="flex flex-wrap gap-4">
                <button
                    type="button"
                    class="cursor-pointer border rounded-full py-2 px-5 bg-surface btn-press-blue-500"
                >
                    "Ajouter un participant"
                </button>
                <button
                    type="button"
                    class="cursor-pointer border rounded-full py-2 px-5 bg-surface btn-press-blue-500"
                >
                    "Relancer les retardataires"
                </button>
                <button
                    type="button"
                    class="cursor-pointer border rounded-full py-2 px-5 bg-raspberry-500 btn-press-ink"
                >
                    "Lancer le tirage"
                </button>
            </div>
        </div>
    }
}

fn format_euros(cents: i32) -> String {
    match cents % 100 {
        0 => format!("{} €", cents / 100),
        rest => format!("{},{rest:02} €", cents / 100),
    }
}

#[component]
fn PersonalLinks(missing_emails: usize) -> impl IntoView {
    view! {
        <Card bg_color=Color::Surface shadow_color=Color::Raspberry>
            <div class="flex flex-wrap items-center justify-between gap-5">
                <div class="max-w-xl">
                    <h3 class="text-xl">"Un lien perso par participant"</h3>
                    <p class="mt-1 text-sm">
                        "Chacun reçoit son lien par email. Toi non plus, tu ne le vois pas : impossible de jeter un œil au binôme d'un autre, même par tentation."
                    </p>
                </div>
                <div class="flex items-center gap-4">
                    {(missing_emails > 0)
                        .then(|| {
                            view! {
                                <p class="text-sm font-bold">
                                    {missing_emails} " emails manquants"
                                </p>
                            }
                        })}
                    <button
                        type="button"
                        class="cursor-pointer border rounded-full py-2 px-5 bg-raspberry-500 btn-press-ink"
                    >
                        "Envoyer les liens"
                    </button>
                </div>
            </div>
        </Card>
    }
}

#[component]
fn SantaParticipantCard(participant: ParticipantDto, color: Color) -> impl IntoView {
    let initial = participant
        .name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string());
    let (status, status_class, detail) = match participant.form_completed_at {
        Some(at) => (
            "Fiche remplie",
            "bg-blue-200",
            format!(
                "Remplie le {}",
                at.format_localized("%-d %B", chrono::Locale::fr_FR)
            ),
        ),
        None => (
            "En attente",
            "bg-butter-200",
            "N'a pas encore rempli sa fiche".to_string(),
        ),
    };
    let email_id = format!("email-{}", participant.id);

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
                <h4 class="text-lg truncate">{participant.name}</h4>
            </div>
            <span class=format!(
                "w-fit px-3 py-0.5 text-sm border rounded-full font-extrabold {status_class}",
            )>{status}</span>
            <p class="text-sm">{detail}</p>
            <hr class="border-t-2 border-dotted border-line" />
            <div class="flex items-center justify-between gap-2">
                <label for=email_id.clone() class="text-base">
                    "Email"
                </label>
                <span class="px-2 py-0.5 text-xs border rounded-full font-extrabold bg-surface">
                    "Email manquant"
                </span>
            </div>
            <input id=email_id type="email" placeholder="prenom@exemple.fr" class="bg-surface" />
        </div>
    }
}
