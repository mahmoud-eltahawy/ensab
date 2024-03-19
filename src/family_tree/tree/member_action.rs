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
        <AddSon member=name add_person=add_person take_action=take_action/>
        <>
    }
}

#[component]
fn AddSon(
    member: RwSignal<FamilyMember>,
    add_person: RwSignal<bool>,
    take_action: RwSignal<bool>,
) -> impl IntoView {
    // let handle_person = move |person: RwSignal<FamilyMember>| {
    //     logging::log!(
    //         "generation : {}\nsibling order {}",
    //         person.get().generation + 1,
    //         person.get().sons.len() + 1
    //     );

    //     person.update(|x| {
    //         x.sons.push(RwSignal::new(FamilyMember {
    //             sibling_order: x.sons.len() as i32 + 1,
    //             generation: x.generation + 1,
    //             ..Default::default()
    //         }))
    //     });
    // };

    let name: NodeRef<html::Input> = create_node_ref();
    let gender: NodeRef<html::Select> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let name = name().expect("<input> should be mounted").value();
        let gender: bool = gender()
            .expect("<input> should be mounted")
            .value()
            .parse()
            .unwrap();

        member.update(|x| x.get_son(name, gender));
        add_person.set(false);
        logging::log!("father {:#?}", member.get());
    };

    view! {
        <Show
            when=add_person
        >
            <form
                on:submit=on_submit
                class="grid justify-content-center justify-items-center gap-5 p-10 m-5 border-4 size-50 rounded-lg z-20 absolute place-self-center bg-white"
            >
                <input
                    node_ref=name
                    placeholder="الاسم"
                />
                <select
                    node_ref=gender
                >
                    <option value="true">"ذكر"</option>
                    <option value="false">"انثي"</option>
                </select>
                <button type="submit">"تاكيد"</button>
                <button on:click=move |_| {
                    add_person.set(false);
                    take_action.set(true);
                }>"الغاء"</button>
            </form>
        </Show>
    }
}
