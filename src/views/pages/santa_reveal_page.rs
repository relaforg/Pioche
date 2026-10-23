use leptos::prelude::*;
use leptos::Params;
use leptos_router::{hooks::use_params, params::Params};

use crate::server::error::AppError;
use crate::server::question_kind::{QuestionKindDto, TSHIRT_SIZES};
use crate::server::santa_reveal::{get_santa_reveal, AnswerDto, SaveSantaAnswers, SantaRevealDto};
use crate::server::secret_santa_option::{format_day, format_euros};
use crate::views::colors::Color;
use crate::views::components::{card::Card, draw_view::header::DrawHeader};

#[derive(Params, PartialEq, Clone)]
struct RevealParams {
    token: Option<String>,
}

#[component]
pub fn SantaRevealPage() -> impl IntoView {
    let params = use_params::<RevealParams>();
    let token = move || params.read().as_ref().ok().and_then(|p| p.token.clone());
    let reveal = Resource::new(token, |token| async move {
        let token = token.ok_or_else(|| AppError::Invalid("Lien invalide".into()))?;
        get_santa_reveal(token).await
    });
    view! {
        <Suspense fallback=|| {
            "Chargement..."
        }>
            {move || Suspend::new(async move {
                match reveal.await {
                    Ok(reveal) => {
                        view! { <SantaReveal reveal token=token().unwrap_or_default() /> }
                            .into_any()
                    }
                    Err(e) => {
                        view! {
                            <div class="p-7 border border-dashed rounded-blob text-center font-bold text-fg-soft">
                                {e.to_string()}
                            </div>
                        }
                            .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn SantaReveal(reveal: SantaRevealDto, token: String) -> impl IntoView {
    // Les enfants de `view!` sont des closures `move` : on sépare les champs
    // pour que chaque partie de la vue ne prenne que ce dont elle a besoin.
    let SantaRevealDto {
        draw_name,
        participant_name,
        receiver_name,
        options,
        questions,
        my_answers,
        my_form_completed,
        receiver_answers,
    } = reveal;
    let receiver_title = receiver_name.clone();

    let subtitle = [
        Some("Secret Santa".to_string()),
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
    let has_questions = !questions.is_empty();

    view! {
        <div class="flex flex-col gap-8 w-full max-w-2xl mx-auto">
            <DrawHeader name=draw_name subtitle color=Color::Raspberry>
                {()}
            </DrawHeader>
            <Card bg_color=Color::Surface shadow_color=Color::Butter class="text-center">
                <p class="font-bold text-fg-soft">
                    "Salut " {participant_name} ", cette année tu offres un cadeau à…"
                </p>
                <h2 class="mt-4 text-5xl text-shadow-butter-3 break-words">
                    {receiver_title}
                </h2>
                <p class="mt-6 text-sm text-fg-soft">
                    "Chut ! Ce lien est personnel : personne d'autre ne sait qui tu as tiré."
                </p>
                {(has_questions && !my_form_completed)
                    .then(|| {
                        view! {
                            <a
                                href="#ma-fiche"
                                class="inline-block mt-5 px-5 py-2 border rounded-full font-display font-semibold bg-butter-500 btn-press-ink"
                            >
                                "Remplis ta fiche ↓"
                            </a>
                        }
                    })}
            </Card>
            {has_questions
                .then(|| {
                    view! {
                        <ReceiverFiche name=receiver_name answers=receiver_answers />
                        <MyFiche
                            token
                            questions
                            answers=my_answers
                            completed=my_form_completed
                        />
                    }
                })}
        </div>
    }
}

/// Les réponses de la personne à qui on offre.
#[component]
fn ReceiverFiche(name: String, answers: Option<Vec<AnswerDto>>) -> impl IntoView {
    let body = match answers {
        None => view! {
            <p class="mt-4 p-5 border border-dashed rounded-blob text-center font-bold text-fg-soft">
                {name.clone()} " n'a pas encore rempli sa fiche. Reviens un peu plus tard !"
            </p>
        }
        .into_any(),
        Some(answers) if answers.is_empty() => view! {
            <p class="mt-4 font-bold text-fg-soft">
                "Fiche remplie… mais vide. À toi de jouer les détectives !"
            </p>
        }
        .into_any(),
        Some(answers) => view! {
            <dl class="flex flex-col gap-4 mt-5">
                {answers
                    .into_iter()
                    .map(|AnswerDto { question, value }| {
                        view! {
                            <div class="pb-4 border-b-2 border-dotted border-line last:border-0 last:pb-0">
                                <dt class="font-display font-semibold text-lg">{question.label()}</dt>
                                <dd class="mt-1 whitespace-pre-line">{value}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
        }
        .into_any(),
    };

    view! {
        <Card bg_color=Color::Surface shadow_color=Color::Blue>
            <h3 class="text-2xl">"La fiche de " {name}</h3>
            {body}
        </Card>
    }
}

/// Le formulaire du participant, modifiable à tout moment.
#[island]
fn MyFiche(
    token: String,
    questions: Vec<QuestionKindDto>,
    answers: Vec<AnswerDto>,
    completed: bool,
) -> impl IntoView {
    let save = ServerAction::<SaveSantaAnswers>::new();
    let server_error = move || save.value().get().and_then(Result::err);
    // Valeurs initiales calculées avant la vue : elle ne peut pas emprunter `answers`.
    let fields: Vec<(usize, QuestionKindDto, String)> = questions
        .into_iter()
        .enumerate()
        .map(|(index, question)| {
            let value = answers
                .iter()
                .find(|a| a.question == question)
                .map(|a| a.value.clone())
                .unwrap_or_default();
            (index, question, value)
        })
        .collect();

    view! {
        <ActionForm action=save attr:id="ma-fiche">
            <Card bg_color=Color::Surface shadow_color=Color::Raspberry>
                <input type="hidden" name="token" value=token />
                <div class="flex flex-wrap items-center justify-between gap-3">
                    <h3 class="text-2xl">"Ta fiche"</h3>
                    {completed
                        .then(|| {
                            view! {
                                <span class="px-3 py-0.5 text-sm border rounded-full font-extrabold bg-blue-200">
                                    "✓ Enregistrée"
                                </span>
                            }
                        })}
                </div>
                <p class="mt-1 text-sm">
                    "Elle aide ton Père Noël secret à trouver le bon cadeau. Lui seul la verra, et tu peux la modifier quand tu veux."
                </p>
                <div class="flex flex-col gap-5 mt-6">
                    {fields
                        .into_iter()
                        .map(|(index, question, value)| {
                            view! { <QuestionField index question value /> }
                        })
                        .collect_view()}
                </div>
                <button
                    type="submit"
                    class="mt-7 w-full cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink"
                    disabled=move || save.pending().get()
                >
                    {if completed { "Mettre à jour ma fiche" } else { "Enregistrer ma fiche" }}
                </button>
                {move || {
                    server_error()
                        .map(|e| {
                            view! {
                                <p role="alert" class="mt-3 text-center font-bold text-raspberry-600">
                                    {e.to_string()}
                                </p>
                            }
                        })
                }}
            </Card>
        </ActionForm>
    }
}

/// Un champ adapté au type de question. Les noms suivent `answers[i][…]`,
/// que le serveur reçoit comme `Vec<AnswerDto>`.
#[component]
fn QuestionField(index: usize, question: QuestionKindDto, value: String) -> impl IntoView {
    let id = format!("answer-{index}");
    let name = format!("answers[{index}][value]");

    let field = match question {
        QuestionKindDto::TshirtSize => view! {
            <select id=id.clone() name=name class="mt-2">
                <option value="">{question.placeholder()}</option>
                {TSHIRT_SIZES
                    .into_iter()
                    .map(|size| view! { <option value=size selected={value == size}>{size}</option> })
                    .collect_view()}
            </select>
        }
        .into_any(),
        QuestionKindDto::ShoeSize => view! {
            <input
                id=id.clone()
                name=name
                type="number"
                min=15
                max=55
                step=0.5
                inputmode="decimal"
                placeholder=question.placeholder()
                value=value
                class="mt-2"
            />
        }
        .into_any(),
        QuestionKindDto::Allergies | QuestionKindDto::Wishes | QuestionKindDto::Dislikes => view! {
            <textarea
                id=id.clone()
                name=name
                rows=3
                maxlength=500
                placeholder=question.placeholder()
                class="mt-2 w-full p-3 bg-neutral-200 border rounded-chip font-bold"
            >
                {value}
            </textarea>
        }
        .into_any(),
    };

    view! {
        <div>
            <input
                type="hidden"
                name=format!("answers[{index}][question]")
                value=question.form_value()
            />
            <label for=id>{question.label()}</label>
            {field}
        </div>
    }
}
