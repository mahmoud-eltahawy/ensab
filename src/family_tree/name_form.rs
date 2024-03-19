use leptos::{ev::SubmitEvent, *};

use super::member::FamilyMember;

#[component]
pub fn NameForm(member: RwSignal<FamilyMember>, is_visualed: RwSignal<bool>) -> impl IntoView {
    let name: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let person = FamilyMember::create_from_name(name().expect("<input> to exist").value(), 1);
        member.set(person);
        is_visualed.update(|x| *x = !*x);
    };

    view! {
        <form
            on:submit=on_submit
            class="grid gap-5 grid-cols-1 m-10 border-2 rounded-lg p-10 text-3xl"
        >
            <input
                node_ref=name
                class="border-2 p-5 text-center"
                required
            />
            <button
                class="border-2 p-5 rounded-lg hover:text-5xl"
            >"ارني"</button>
        </form>
    }
}
