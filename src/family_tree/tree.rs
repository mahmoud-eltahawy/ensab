use leptos::*;

use super::member::FamilyMember;

mod member_action;
use member_action::MemberAction;

#[component]
pub fn Tree(name: RwSignal<FamilyMember>) -> impl IntoView {
    let take_action = RwSignal::new(false);

    view! {
        <>
        <MemberAction name=name take_action=take_action/>
        <div class="flex flex-col m-10 flex-nowrap">
            <button
                on:click=move |_| {take_action.update(|x| *x = !*x)}
                class="pt-3 pb-1 mx-5 size-50 rounded-full"
            >{name.get().name}</button>
            <Sons name=name/>
        </div>
        </>
    }
}

#[component]
fn Sons(name: RwSignal<FamilyMember>) -> impl IntoView {
    let sons = {
        let name = name.clone();
        move || name.get().sons
    };

    let key = |k: &RwSignal<FamilyMember>| k.get().key();

    let when = {
        let sons = sons.clone();
        move || sons().len() > 0
    };
    view! {
        <Show when=when>
            <div class="flex flex-row flex-nowrap gap-5 flex-auto justify-center border-t-2 rounded-t-lg px-4 mx-4 border-black">
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
