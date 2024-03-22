use leptos::*;

use super::member::FamilyMember;

mod member_action;
use member_action::MemberAction;

#[component]
pub fn Tree(member: FamilyMember) -> impl IntoView {
    let take_action = RwSignal::new(false);

    view! {
        <>
            <div class="flex flex-col m-10 flex-nowrap">
                <button
                    on:click=move |_| { take_action.update(|x| *x = !*x) }
                    class="pt-3 pb-1 mx-5 size-50 rounded-full"
                >
                    {move || member.name.get()}
                </button>
                <Sons name=member/>
            </div>
            <MemberAction member=member take_action=take_action/>
        </>
    }
}

#[component]
fn Sons(name: FamilyMember) -> impl IntoView {
    let sons = move || name.sons.get();

    let key = |k: &FamilyMember| k.id;

    let when = move || !sons().is_empty();
    view! {
        <Show when=when>
            <div class="flex flex-row gap-4 overflow-auto border-t-2 rounded-t-lg px-4 mx-4 border-black">
                <For each=sons key=key let:member>
                    <Tree member=member/>
                </For>
            </div>
        </Show>
    }
}
