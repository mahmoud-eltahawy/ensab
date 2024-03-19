use leptos::*;

use super::member::FamilyMember;

mod member_action;
use member_action::MemberAction;

#[component]
pub fn Tree(name: RwSignal<FamilyMember>) -> impl IntoView {
    let take_action = RwSignal::new(false);

    view! {
        <div class="grid grid-cols-1 m-10">
            <MemberAction name=name take_action=take_action/>
            <button
                on:click=move |_| {take_action.update(|x| *x = !*x)}
                class="pt-3 pb-1 mx-5 border-t-4 size-50 rounded-full"
            >{name.get().name}</button>
            <Sons name=name/>
        </div>
    }
}

#[component]
fn Sons(name: RwSignal<FamilyMember>) -> impl IntoView {
    let sons = {
        let name = name.clone();
        move || name.get().sons
    };

    let key = |k: &RwSignal<FamilyMember>| k.get_untracked().key();

    let when = {
        let sons = sons.clone();
        move || sons().len() > 0
    };
    view! {
        <Show when=when>
            <div class="flex flex-row gap-5 border-t-2 rounded-t-lg px-4 mx-4 border-black">
                <For
                    each=sons.clone()
                    key=key
                    let:member
                >
                    <Tree name=member/>
                </For>
            </div>
        </Show>
    }
}
