use leptos::{
    ev::{Event, SubmitEvent},
    *,
};

use super::member::FamilyMember;

#[component]
pub fn NameForm(name: RwSignal<FamilyMember>, is_visualed: RwSignal<bool>) -> impl IntoView {
    let on_input = move |ev: Event| {
        let mut names = event_target_value(&ev)
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut person = FamilyMember::default();
        person.name = names.pop().unwrap();
        person.with_sons(&mut names, person.generation + 1);
        name.set(person);
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        is_visualed.update(|x| *x = !x.clone());
        //
    };

    view! {
        <form on:submit=on_submit class="grid grid-cols-1">
            <input on:input=on_input required/>
            <button>"ارني"</button>
        </form>
    }
}
