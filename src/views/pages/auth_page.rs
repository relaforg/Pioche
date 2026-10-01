use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn AuthPage() -> impl IntoView {
    let steps = [
        (
            "Tous tes tirages au même endroit",
            "Noël, anniversaires, d’une année à l’autre.",
        ),
        (
            "Modifie sans tout recommencer",
            "Ajoute un retardataire ou relance le tirage en un clic.",
        ),
        (
            "Suis la participation",
            "Tu vois qui a ouvert son lien — jamais qui a pioché qui.",
        ),
    ];
    view! {
        <div class="flex flex-col-reverse lg:flex-row justify-between items-center gap-30">
            <div>
                <span class="font-extrabold text-sm border rounded-full py-1 px-2">
                    "ESPACE ORGANISATEUR"
                </span>
                <h2 class="text-5xl text-shadow-butter-3 mb-5 mt-7">
                    "Tu gères le tirage. Le secret reste secret."
                </h2>
                <p class="text-lg font-bold my-5">
                    "Seul l’organisateur a besoin d’un compte. Les participants n’ont qu’un lien."
                </p>
                <ol class="space-y-3">
                    {steps
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
            <Outlet />
        </div>
    }
}
