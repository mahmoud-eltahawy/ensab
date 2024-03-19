use leptos::*;

use crate::family_tree::member::FamilyMember;

#[component]
pub fn MemberAction(name: RwSignal<FamilyMember>, take_action: RwSignal<bool>) -> impl IntoView {
    let add_person = RwSignal::new(false);

    view! {
    <>
        <Show
            when=take_action
        >
            <div
                class="grid justify-content-center justify-items-center gap-5 p-10 m-5 border-4 size-50 rounded-lg z-10 absolute place-self-center bg-white"
            >
                <h2>{name.get().name}</h2>
                <button
                    on:click=move |_| {
                        add_person.set(true);
                        take_action.set(false);
                    }
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
        </Show>
        <AddSon
            member=name
            add_person=add_person
            take_action=take_action
        />
    <>
    }
}

#[component]
fn AddSon(
    member: RwSignal<FamilyMember>,
    add_person: RwSignal<bool>,
    take_action: RwSignal<bool>,
) -> impl IntoView {
    let name: NodeRef<html::Input> = create_node_ref();
    let gender: NodeRef<html::Select> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let name = name().expect("<input> should be mounted").value();
        let is_male: bool = gender().unwrap().value().parse().unwrap_or(true);

        member.update(|x| x.add_son(name, is_male));
        add_person.set(false);
        logging::log!("father {:#?}", member.get());
    };

    view! {
        <Show
            when=add_person
        >
            <form
                on:submit=on_submit
                class="grid grid-cols-4 justify-content-center justify-items-center gap-5 p-10 m-5 border-4 size-50 rounded-lg z-20 absolute place-self-center bg-white"
            >
                <input
                    class="col-span-4 text-center border-2 m-5 p-5 text-3xl"
                    node_ref=name
                    placeholder="الاسم"
                    required
                />
                <select
                    class="col-span-4 text-center border-2 m-5 p-5 text-3xl"
                    node_ref=gender
                >
                    <option
                        value="true"
                        class="text-center border-2 m-5 p-5 text-3xl"
                    >"ذكر"</option>
                    <option
                        value="false"
                        class="text-center border-2 m-5 p-5 text-3xl"
                    >"انثي"</option>
                </select>
                <button
                    class="border-2 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
                    on:click=move |_| {
                        add_person.set(false);
                        take_action.set(true);
                    }
                >"الغاء"</button>
                <button
                    class="border-2 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
                    type="submit"
                >"تاكيد"</button>
            </form>
        </Show>
    }
}
