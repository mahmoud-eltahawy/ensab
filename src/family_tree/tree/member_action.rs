use leptos::{ev::Event, *};

use crate::family_tree::member::FamilyMember;

#[component]
pub fn MemberAction(member: RwSignal<FamilyMember>, take_action: RwSignal<bool>) -> impl IntoView {
    let actions = ActionSignals::default();

    view! {
        <>
            <Buttons
                take_action=take_action
                action_signals=actions
                member_name=move || member.get().name.get()
            />
            <Actions member=member take_action=take_action actions=actions/>
        </>
    }
}

#[component]
fn Actions(
    take_action: RwSignal<bool>,
    member: RwSignal<FamilyMember>,
    actions: ActionSignals,
) -> impl IntoView {
    let ActionSignals { add_person } = actions;
    let cancel_add_son = move || {
        add_person.set(false);
        take_action.set(true);
    };
    view! {
        <>
            <AddMember member=member add_person=add_person cancel=cancel_add_son/>
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
        <Show when=when>
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    on_submit();
                }
                class="grid grid-cols-4 justify-content-center justify-items-center gap-5 p-10 m-28 border-4 inset-0 rounded-lg z-50 absolute bg-white"
            >
                {children()}
                <button
                    class="border-2 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
                    type="submit"
                >
                    "تاكيد"
                </button>
                <button
                    class="border-2 col-span-2 text-2xl p-5 m-5 rounded-lg hover:rounded-full"
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
}

impl Default for ActionSignals {
    fn default() -> Self {
        Self {
            add_person: RwSignal::new(false),
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
    let ActionSignals { add_person } = action_signals;
    view! {
        <Show when=take_action>
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
                <button class="p-5 border-2 rounded-lg">"حذف الابن"</button>
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
fn AddMember<F>(
    member: RwSignal<FamilyMember>,
    add_person: RwSignal<bool>,
    cancel: F,
) -> impl IntoView
where
    F: Fn() + Clone + Copy + 'static,
{
    let name: NodeRef<html::Input> = create_node_ref();
    let gender: NodeRef<html::Select> = create_node_ref();

    let is_only = RwSignal::new(true);

    let on_submit = move || {
        let name = name().expect("<input> should be mounted").value();
        let names = name.split(',').map(|x| x.to_string()).collect::<Vec<_>>();
        let is_male: bool = gender().unwrap().value().parse().unwrap_or(true);

        for name in names {
            member.update(|x| x.add_son(name, is_male));
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
