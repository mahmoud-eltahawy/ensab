use std::str::FromStr;

use contracts::member::RawMember;
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

mod member;
use member::Member;

enum IdName {
    Id(Uuid),
    Name(String),
}

#[derive(Clone, Copy)]
struct ActionsWaitlist(RwSignal<Vec<Uuid>>);

impl ActionsWaitlist {
    fn new() -> Self {
        ActionsWaitlist(RwSignal::new(Vec::new()))
    }
    fn take(&self, id: Uuid) {
        self.0.update(|xs| xs.push(id));
    }
    fn redraw(&self, id: Uuid) {
        self.0.update(|xs| xs.retain(|x| *x != id));
    }
    fn check(&self, id: Uuid) -> bool {
        self.0.get().last().is_some_and(|x| *x == id)
    }
}

#[component]
pub fn MemberNode() -> impl IntoView {
    let params = use_params_map();
    let id_or_name = move || {
        let params = params.get();
        let name = params.get("name").unwrap();
        match Uuid::from_str(name) {
            Ok(id) => IdName::Id(id),
            Err(_) => IdName::Name(name.to_string()),
        }
    };

    provide_context(ActionsWaitlist::new());

    match id_or_name() {
        IdName::Id(id) => {
            view! {
                <ServerNode id=id/>
            }
        }
        IdName::Name(name) => {
            view! {
                <ClientNode name=name/>
            }
        }
    }
}

#[component]
fn ServerNode(id: Uuid) -> impl IntoView {
    #[server]
    async fn get_member(id: Uuid) -> Result<RawMember, ServerFnError> {
        use db::{member::read, Pool, Postgres};
        let pool = expect_context::<Pool<Postgres>>();
        match read(&pool, id).await {
            Ok(member) => Ok(member),
            Err(err) => Err(ServerFnError::ServerError(err.to_string())),
        }
    }

    let member_resource = Resource::once(move || get_member(id));
    let member = move || {
        member_resource
            .get()
            .map(|x| x.ok())
            .flatten()
            .map(|x| Member::from_raw(x))
            .unwrap_or_default()
    };

    view! {
    <section class="grid justify-items-center overflow-auto">
        <h1 class="text-center m-5 text-3xl">تعديل الشجرة</h1>
        <Suspense>
            <Node member=member()/>
        </Suspense>
    </section>
    }
}

#[component]
fn ClientNode(name: String) -> impl IntoView {
    let member = Member::new(name);
    view! {
    <section class="grid justify-items-center overflow-auto">
        <h1 class="text-center m-5 text-3xl">بناء الشجرة</h1>
        <Node member=member/>
    </section>
    }
}

#[component]
fn Node(member: Member) -> impl IntoView {
    provide_context(member);
    let actions_waitlist = expect_context::<ActionsWaitlist>();
    let on_click = move |_| {
        actions_waitlist.take(member.id);
    };

    view! {
    <div class="flex flex-col my-10 flex-nowrap">
      <button
        on:click=on_click
        class="pt-3 pb-1 mx-5 size-50 rounded-full"
      >
        {move || member.name.get()}
      </button>
      <Show
          when=move || !member.sons.get().is_empty()>
        <div class="flex flex-row overflow-auto border-t-2 rounded-t-lg border-black">
          <For
              each=move || member.sons.get()
              key=move |x| x.id
              let:son
          >
              <Node member=son/>
          </For>
        </div>
      </Show>
    </div>
    <Show when=move || actions_waitlist.check(member.id)>
        <Action/>
    </Show>
    }
}

#[component]
fn Action() -> impl IntoView {
    #[component]
    fn Preview() -> impl IntoView {
        #[component]
        fn AButton(value: String, action: member::Action) -> impl IntoView {
            let member = expect_context::<Member>();
            let on_click = move |_| {
                member.action.set(action);
            };
            view! {
                 <button
                    on:click=on_click
                    class="p-5 w-96 m-2 border-2 border-gray-400 bg-gray-950 hover:border-gray-950 rounded-lg"
                >{value}</button>
            }
        }

        let member = expect_context::<Member>();
        let actions_waitlist = expect_context::<ActionsWaitlist>();
        view! {
            <div
              class="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-3xl text-pretty text-zinc-300 rounded-lg bg-gray-700 border-gray-400 hover:border-gray-700 grid justify-content-center justify-items-center gap-5 p-5 mx-32 my-10 border-4 z-10"
            >
              <h2 class="text-center">{move || member.name.get()}</h2>
              <AButton value="اضافة ابن".to_string() action=member::Action::Add/>
              <AButton value="حذف الابن".to_string() action=member::Action::Remove/>
              <AButton value="تحديث بيانات".to_string() action=member::Action::Update/>
              <button
                  class="p-5 w-96 border-2 hover:border-red-950 bg-red-950 border-red-400 rounded-lg"
                  on:click=move |_| actions_waitlist.redraw(member.id)
              >
                  الغاء
              </button>
            </div>
        }
    }
    #[component]
    fn ActionDiv<F>(submit: F, children: Children) -> impl IntoView
    where
        F: Fn() + 'static + Clone + Copy,
    {
        let member = expect_context::<Member>();
        let actions_waitlist = expect_context::<ActionsWaitlist>();

        view! {
        <div
          class="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-3xl text-pretty text-zinc-300 rounded-lg bg-gray-700 border-gray-500 hover:border-gray-700 grid justify-content-center justify-items-center gap-5 p-5 border-4 z-10"
        >
          {children()}
          <button
              class="bg-green-950 border-green-600 hover:border-green-950 border-2 w-56 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg"
              on:click=move |_| {submit();actions_waitlist.redraw(member.id)}
          >
              تاكيد
          </button>
          <button
              class="bg-red-950 border-red-600 hover:border-red-950 border-2 w-56 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg"
              on:click=move |_| actions_waitlist.redraw(member.id)
          >
              الغاء
          </button>
        </div>
        }
    }

    #[component]
    fn Add() -> impl IntoView {
        let is_only = RwSignal::new(true);
        let names = RwSignal::new(String::new());
        let select_ref = create_node_ref::<html::Select>();
        let on_input = move |ev| {
            let value = event_target_value(&ev);
            if value.contains(',') {
                is_only.set(false);
            } else {
                is_only.set(true);
            }
            names.set(value);
        };

        let member = expect_context::<Member>();
        let submit = move || {
            let value: bool = select_ref.get().unwrap().value().parse().unwrap();
            names.get().split(',').for_each(|name| {
                let new_member = Member::create_from_name(name);
                new_member.is_male.set(value);
                member.add_son(new_member);
            });
        };

        view! {
        <ActionDiv submit>
          <input
              on:input=on_input
              class="col-span-4 placeholder:text-center placeholder-gray-400 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 p-2 text-4xl rounded-lg w-96"
              placeholder="الاسم"
              required
          />
          <select
            node_ref=select_ref
            class=" col-span-4 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 text-4xl rounded-lg w-56"
          >
              <option value="true" class="text-center p-5 text-4xl">
                  {move || if is_only.get() {"ذكر"} else {"ذكور"} }
              </option>
              <option value="false" class="text-center p-5 text-4xl">
                  {move || if is_only.get()  {"انثي"} else {"اناث"}}
              </option>
          </select>
        </ActionDiv>
        }
    }

    #[component]
    fn Remove() -> impl IntoView {
        let member = expect_context::<Member>();
        let removed = RwSignal::new(Vec::new());
        let get_restored = move || {
            member
                .sons
                .get()
                .into_iter()
                .filter(|x| !removed.get().contains(&x.id))
                .collect::<Vec<_>>()
        };
        let get_removed = move || {
            member
                .sons
                .get()
                .into_iter()
                .filter(|x| removed.get().contains(&x.id))
                .collect::<Vec<_>>()
        };
        let remove = move |id: Uuid| {
            removed.update(|xs| xs.push(id));
        };
        let restore = move |id: Uuid| {
            removed.update(|xs| xs.retain(|x| *x != id));
        };

        let submit = move || {
            member
                .sons
                .update(|xs| xs.retain(|x| !removed.get_untracked().contains(&x.id)));
        };
        view! {
        <ActionDiv submit>
          <div class="border-2 col-span-4 flex flex-wrap rounded-lg bg-gray-950 border-gray-500 hover:border-gray-950">
              <For
                  each=get_restored
                  key=|x| x.id
                  let:son
              >
                <button
                  on:click=move |_| {remove(son.id)}
                  class="m-5 p-5 border-4 h-20 rounded-full bg-lime-700 hover:bg-red-700"
                >{move || son.name.get()}</button>
              </For>
              <For
                  each=get_removed
                  key=|x| x.id
                  let:son
              >
                <button
                  on:click=move |_| {restore(son.id)}
                  class="m-5 p-5 border-4 h-20 rounded-full bg-red-700 hover:bg-lime-700"
                >{move || son.name.get()}</button>
              </For>
          </div>
        </ActionDiv>
        }
    }
    #[component]
    fn Update() -> impl IntoView {
        let member = expect_context::<Member>();
        let name_ref = create_node_ref::<html::Input>();
        let gender_ref = create_node_ref::<html::Select>();
        let submit = move || {
            let name = name_ref.get().unwrap().value();
            let gender: bool = gender_ref.get().unwrap().value().parse().unwrap();
            if name.is_empty() {
                return;
            }
            member.name.set(name);
            member.is_male.set(gender);
        };
        view! {
        <ActionDiv submit>
          <input
              class="col-span-4 placeholder:text-center placeholder-gray-400 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 p-2 text-4xl rounded-lg w-96"
              placeholder=move || member.name.get()
              node_ref=name_ref
          />
          <select
            class="col-span-4 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 text-4xl rounded-lg w-56"
            node_ref=gender_ref
          >
            <option value="true" class="text-center p-5 text-4xl">ذكر</option>
            <option value="false" class="text-center p-5 text-4xl">انثي</option>
          </select>
        </ActionDiv>
        }
    }

    let member = expect_context::<Member>();
    let result = move || match member.action.get() {
        member::Action::Preview => view! { <Preview/> },
        member::Action::Add => view! { <Add/> },
        member::Action::Remove => view! { <Remove/> },
        member::Action::Update => view! { <Update/> },
    };
    result
}
