use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::{
    server::{draw_kind::DrawKindDto, draws::AddDraw, session::current_user},
    views::{
        colors::Color,
        components::{
            button_link::ButtonLink,
            create_draw::{
                exclusions_field::ExclusionsField, kind_picker::KindPicker, name_field::NameField,
                participants_field::ParticipantsField, paste_dialog::PasteDialog,
                secret_santa_options::SecretSantaOption, submit_bar::SubmitBar,
                teams_options::TeamsOption,
            },
        },
    },
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
    let kind = RwSignal::new(DrawKindDto::SecretSanta);
    let participants = RwSignal::new(Vec::<String>::new());
    let dialog = NodeRef::<leptos::html::Dialog>::new();

    let missing = Signal::derive(move || {
        kind.get()
            .min_participants()
            .saturating_sub(participants.with(Vec::len))
    });

    view! {
        <ActionForm action=add_draw>
            <KindPicker kind />

            <div class="bg-surface border shadow-butter-5 p-5 rounded-blob">
                <NameField kind />

                <ParticipantsField participants dialog />

                <hr class="border-dashed border-t-2 border-line my-7" />
                <ExclusionsField participants />
                {move || match kind.get() {
                    DrawKindDto::SecretSanta => view! { <SecretSantaOption /> }.into_any(),
                    DrawKindDto::Teams => view! { <TeamsOption /> }.into_any(),
                }}
            </div>
            <SubmitBar add_draw missing />

        </ActionForm>
        <PasteDialog dialog participants />
    }
}
