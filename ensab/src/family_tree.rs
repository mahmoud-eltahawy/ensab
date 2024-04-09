use leptos::{ev::Event, *};

mod member;
mod tree;
use leptos_router::use_params_map;
use member::FamilyMember;
use tree::Tree;

#[component]
pub fn FamilyTree() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").cloned()).unwrap();
    let member = FamilyMember::new(RwSignal::new(name()));

    view! { <Tree member=member/> }
}

#[component]
pub fn NameForm() -> impl IntoView {
    let name: NodeRef<html::Input> = create_node_ref();
    let submit = move || {
        let name = name.get().expect("<input> to exist").value();
        window()
            .location()
            .set_href(&format!("ftree/{}", name))
            .unwrap();
    };

    let on_input = move |ev: Event| {
        let s = event_target_value(&ev);
        let s = s.trim_start();
        if s.contains(' ') {
            submit();
        }
    };

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                submit()
            }

            class="grid gap-5 grid-cols-1 m-10 border-2 rounded-lg p-10 text-3xl"
        >
            <h2 class="text-center">"اسم رأس العائلة"</h2>
            <input on:input=on_input node_ref=name class="border-2 p-5 text-center" required/>
            <button class="border-2 p-5 rounded-lg hover:text-5xl">"ابدء"</button>
        </form>
    }
}
