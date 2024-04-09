use leptos::{ev::Event, *};
use uuid::Uuid;

use crate::family_tree::member::FamilyMember;

#[component]
pub fn MemberAction(member: FamilyMember, take_action: RwSignal<bool>) -> impl IntoView {
    let actions = ActionSignals::default();

    view! {
        <>
            <Buttons
                take_action=take_action
                action_signals=actions
                member_name=move || member.name.get()
            />
            <Actions member=member take_action=take_action actions=actions/>
        </>
    }
}

#[component]
fn Actions(
    take_action: RwSignal<bool>,
    member: FamilyMember,
    actions: ActionSignals,
) -> impl IntoView {
    let ActionSignals {
        add_person,
        remove_person,
    } = actions;
    let cancel_add_son = move || {
        add_person.set(false);
        take_action.set(true);
    };
    let cancel_remove_son = move || {
        remove_person.set(false);
        take_action.set(true);
    };
    view! {
        <>
            <AddMember member=member add_person=add_person cancel=cancel_add_son/>
            <RemoveSon member=member remove_person=remove_person cancel=cancel_remove_son/>
        </>
    }
}

#[component]
fn Action<F1, F2>(
    when: RwSignal<bool>,
    on_submit: F1,
    cancel: F2,
    children: ChildrenFn,
) -> impl IntoView
where
    F1: Fn() + Clone + Copy + 'static,
    F2: Fn() + Clone + Copy + 'static,
{
    view! {
        <Show when=move ||when.get()>
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    on_submit();
                }
                class="grid grid-cols-4 justify-content-center justify-items-center gap-5 p-10 m-28 border-4 inset-0 rounded-lg z-50 absolute bg-white"
            >
                {children()}
                <button
                    class="border-2 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
                    type="submit"
                >
                    "تاكيد"
                </button>
                <button
                    class="border-2 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
                    on:click=move |_| { cancel() }
                >

                    "الغاء"
                </button>
            </form>
        </Show>
    }
}

#[derive(Clone, Copy)]
struct ActionSignals {
    add_person: RwSignal<bool>,
    remove_person: RwSignal<bool>,
}

impl Default for ActionSignals {
    fn default() -> Self {
        Self {
            add_person: RwSignal::new(false),
            remove_person: RwSignal::new(false),
        }
    }
}

#[component]
fn Buttons<F>(
    take_action: RwSignal<bool>,
    action_signals: ActionSignals,
    member_name: F,
) -> impl IntoView
where
    F: Fn() -> String + Clone + Copy + 'static,
{
    let ActionSignals {
        add_person,
        remove_person,
    } = action_signals;
    view! {
        <Show when=move || take_action.get()>
            <div class="grid justify-content-center justify-items-center gap-5 p-10 mx-32 my-10 border-4 size-50 rounded-lg z-10 absolute inset-0 bg-white">
                <h2>{move || member_name()}</h2>
                <button
                    on:click=move |_| {
                        add_person.set(true);
                        take_action.set(false);
                    }
                    class="p-5 m-2 border-2 rounded-lg"
                >
                    "اضافة ابن"
                </button>
                <button
                    on:click=move |_| {
                        remove_person.set(true);
                        take_action.set(false);
                    }
                    class="p-5 border-2 rounded-lg"
                >"حذف الابن"</button>
                <button class="p-5 border-2 rounded-lg">"تغيير الاسم"</button>
                <button
                    class="p-5 border-2 rounded-lg"
                    on:click=move |_| { take_action.set(false) }
                >
                    "الغاء"
                </button>
            </div>
        </Show>
    }
}

#[component]
fn AddMember<F>(member: FamilyMember, add_person: RwSignal<bool>, cancel: F) -> impl IntoView
where
    F: Fn() + Clone + Copy + 'static,
{
    let name: NodeRef<html::Input> = create_node_ref();
    let gender: NodeRef<html::Select> = create_node_ref();

    let is_only = RwSignal::new(true);

    let on_submit = move || {
        let name = name.get().expect("<input> should be mounted").value();
        let names = name.split(',').map(|x| x.to_string()).collect::<Vec<_>>();
        let is_male: bool = gender.get().unwrap().value().parse().unwrap_or(true);

        for name in names {
            member.add_son(name, is_male);
        }
        add_person.set(false);
    };

    let on_input = move |ev: Event| {
        let s = event_target_value(&ev);
        if s.contains(',') {
            is_only.set(false);
        } else {
            is_only.set(true);
        }
    };

    view! {
        <Action when=add_person cancel=cancel on_submit=on_submit>
            <input
                class="col-span-4 text-center border-2 mx-5 p-2 text-2xl rounded-lg"
                node_ref=name
                on:input=on_input
                placeholder="الاسم"
                required
            />
            <select class="col-span-4 text-center border-2 m-5 p-5 text-3xl rounded-lg" node_ref=gender>
                <option value="true" class="text-center border-2 mx-5 p-2 text-2xl">
                    {move || if is_only.get() { "ذكر" } else { "ذكور" }}
                </option>
                <option value="false" class="text-center border-2 mx-5 p-2 text-2xl">
                    {move || if is_only.get() { "انثي" } else { "اناث" }}
                </option>
            </select>
        </Action>
    }
}

#[component]
fn RemoveSon<F>(member: FamilyMember, remove_person: RwSignal<bool>, cancel: F) -> impl IntoView
where
    F: Fn() + Clone + Copy + 'static,
{
    let deleted = RwSignal::<Vec<FamilyMember>>::new(vec![]);
    let on_submit = move || {
        let deleted_ids = deleted
            .get_untracked()
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        member.sons.update(|xs| {
            xs.retain(|x| !deleted_ids.contains(&x.id));
        });
        deleted.set(vec![]);
        logging::log!("hello {:#?}", deleted_ids);
    };

    let remove = move |id: Uuid| {
        let member = member
            .sons
            .get()
            .into_iter()
            .filter(|x| x.id == id)
            .collect::<Vec<_>>();
        for member in member {
            deleted.update(|xs| xs.push(member));
        }
        logging::log!("hello {:#?}", deleted.get_untracked());
    };

    let restore = move |id: Uuid| {
        deleted.update(|xs| xs.retain(|x| x.id.to_owned() != id));
    };

    let each_restored = move || {
        let deleted = deleted.get().into_iter().map(|x| x.id).collect::<Vec<_>>();
        member
            .sons
            .get()
            .into_iter()
            .filter(|x| !deleted.contains(&x.id))
            .collect::<Vec<_>>()
    };

    let cancel = move || {
        cancel();
        deleted.set(vec![]);
    };

    view! {
        <Action when=remove_person cancel=cancel on_submit=on_submit>
            <div class="col-span-4 flex flex-wrap">
                <For
                    each=each_restored
                    key=move |k| k.id
                    let:son
                >
                    <button
                        on:click=move |_| {remove(son.id)}
                        class="m-5 p-5 border-4 h-20 rounded-full hover:bg-black hover:text-white"
                    >{move || son.name.get()}</button>
                </For>
                <For
                    each=move || deleted.get()
                    key=move |k| k.id
                    let:son
                >
                    <button
                        on:click=move |_| {restore(son.id)}
                        class="m-5 p-5 border-4 h-20 rounded-full bg-black text-white hover:bg-white hover:text-black"
                    >{move || son.name.get()}</button>
                </For>
            </div>
        </Action>
    }
}
