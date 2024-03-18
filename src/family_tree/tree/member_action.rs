use leptos::*;

use crate::family_tree::member::FamilyMember;

#[component]
pub fn MemberAction(name: RwSignal<FamilyMember>, take_action: RwSignal<bool>) -> impl IntoView {
    let handle_person = move |person: RwSignal<FamilyMember>| {
        logging::log!(
            "generation : {}\nsibling order {}",
            person.get().generation + 1,
            person.get().sons.len() + 1
        );

        person.update(|x| {
            x.sons.push(RwSignal::new(FamilyMember {
                sibling_order: x.sons.len() as i32 + 1,
                generation: x.generation + 1,
                ..Default::default()
            }))
        });
    };

    view! {
        <div
            class="grid justify-content-center justify-items-center gap-5 p-10 m-5 border-4 size-50 rounded-lg z-10 absolute place-self-center bg-white"
        >
            <h2>{name.get().name}</h2>
            <button
                on:click=move |_| {handle_person(name)}
                class="p-5 m-2 border-2 rounded-lg"
            >"اضافة ابن"</button>
            <button
                class="p-5 border-2 rounded-lg"
            >"حذف الابن"</button>
            <button
                class="p-5 border-2 rounded-lg"
            >"تغيير الاسم"</button>
            <button
                class="p-5 border-2 rounded-lg"
                on:click=move |_| {take_action.set(false)}
            >"الغاء"</button>
        </div>
    }
}
