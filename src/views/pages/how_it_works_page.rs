use leptos::prelude::*;

#[component]
pub fn HowItWorksPage() -> impl IntoView {
    let organizer_steps = [
        (
            "Tu crées ton compte",
            "Juste pour retrouver et modifier tes tirages plus tard.",
        ),
        (
            "Tu colles la liste",
            "Un nom par ligne. Puis budget, date, questions à poser.",
        ),
        (
            "Tu partages les liens",
            "Équipes : un seul lien pour tout le groupe. Secret Santa : un lien perso par participant, pour que personne n'aille fouiner chez les autres.",
        ),
        (
            "Tu suis l'avancement",
            "Qui a rempli sa fiche — jamais qui offre à qui.",
        ),
    ];

    let participant_steps = [
        (
            "Tu ouvres ton lien",
            "Rien à installer, rien à créer. En Secret Santa, il est à ton nom : garde-le pour toi.",
        ),
        (
            "Tu remplis ta fiche",
            "Secret Santa uniquement. Taille, pointure, envies : visible seulement par la personne qui t'offre.",
        ),
        (
            "Tu découvres ton résultat",
            "Équipes : la composition de toutes les équipes. Secret Santa : la personne à qui tu offres, et rien d'autre.",
        ),
    ];

    let faq = [
        (
            "Peut-on se tirer soi-même ?",
            "Non. Le tirage exclut d'office chacun de sa propre pioche.",
        ),
        (
            "L'organisateur voit-il les binômes ?",
            "Jamais. Il voit seulement qui a rempli sa fiche.",
        ),
        (
            "Et si quelqu'un perd son résultat ?",
            "Il rouvre son lien perso, depuis n'importe quel appareil : son résultat l'y attend.",
        ),
        (
            "Peut-on exclure deux personnes ?",
            "Oui, on peut empêcher un couple ou une paire de tomber ensemble.",
        ),
    ];

    view! {
        <div class="max-w-100">
            <h2 class="text-5xl text-shadow-butter-3 mb-5">"Comment ça marche"</h2>
            <p class="text-lg font-bold">
                "Deux rôles, deux parcours. L'organisateur a un compte, les participants n'ont qu'un lien."
            </p>
        </div>
        <div class="flex flex-col lg:flex-row gap-5 my-10">
            <div class="flex-1 rounded-blob border overflow-hidden shadow-butter-5 bg-surface">
                <div class="flex justify-between bg-butter-500 p-5 border-b items-center">
                    <h3 class="text-2xl">"Tu organises"</h3>
                    <span class="font-extrabold rounded-full border px-3 py-1 bg-surface">
                        "COMPTE REQUIS"
                    </span>
                </div>
                <ol class="p-5 space-y-6">
                    {organizer_steps
                        .into_iter()
                        .enumerate()
                        .map(|(i, (title, desc))| {
                            view! {
                                <li class="flex gap-4 items-start">
                                    <span class="shrink-0 size-11 rounded-full border grid place-items-center bg-butter-500 font-semibold font-display">
                                        {i + 1}
                                    </span>
                                    <div>
                                        <h4 class="font-semibold text-xl">{title}</h4>
                                        <p class="font-semibold">{desc}</p>
                                    </div>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </div>
            <div class="flex-1 rounded-blob border overflow-hidden shadow-blue-5 bg-surface">
                <div class="flex justify-between bg-blue-500 p-5 border-b items-center">
                    <h3 class="text-2xl text-bg">"Tu participes"</h3>
                    <span class="font-extrabold rounded-full border px-3 py-1 bg-surface">
                        "AUCUN COMPTE"
                    </span>
                </div>
                <ol class="p-5 space-y-6">
                    {participant_steps
                        .into_iter()
                        .enumerate()
                        .map(|(i, (title, desc))| {
                            view! {
                                <li class="flex gap-4 items-start">
                                    <span class="shrink-0 size-11 rounded-full border border-ink grid place-items-center bg-blue-500 font-semibold font-display text-bg">
                                        {i + 1}
                                    </span>
                                    <div>
                                        <h4 class="font-semibold text-xl">{title}</h4>
                                        <p class="font-semibold">{desc}</p>
                                    </div>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </div>
        </div>
        <h3 class="text-3xl font-semibold my-5">"Les questions qu'on nous pose"</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
            {faq
                .into_iter()
                .map(|(title, desc)| {
                    view! {
                        <div class="border bg-surface rounded-blob shadow-raspberry-5 flex items-center">
                            <div class="px-5 py-3 space-y-3">
                                <h4 class="font-semibold text-xl">{title}</h4>
                                <p class="font-semibold">{desc}</p>
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
