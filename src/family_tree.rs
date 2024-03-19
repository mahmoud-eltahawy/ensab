use leptos::*;

mod member;
mod name_form;
mod tree;
use member::FamilyMember;
use name_form::NameForm;
use tree::Tree;

#[component]
pub fn FamilyTree() -> impl IntoView {
    let name = RwSignal::new(FamilyMember::default());
    let is_visualed = RwSignal::new(false);

    view! {
        <Show
            when=is_visualed
            fallback=move || view! {<NameForm is_visualed=is_visualed member=name/>}
        >
            <Tree name=name/>
        </Show>
    }
}
