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
    let name = move || member.name.get();
    let on_click = move |_| {
        member.action.update(|x| {
            *x = match x {
                Some(_) => None,
                None => Some(member::Action::Preview),
            }
        })
    };

    let sons = {
        let sons = member.sons.get();
        move || sons.clone()
    };
    view! {
    <div class="flex flex-col my-10 flex-nowrap">
      <button
        on:click=on_click
        class="pt-3 pb-1 mx-5 size-50 rounded-full"
      >
        {name}
      </button>
      <Show
          when=move || !member.sons.get().is_empty()>
        <div class="flex flex-row overflow-auto border-t-2 rounded-t-lg border-black">
          <For
              each=sons.clone()
              key=move |x| x.id
              let:son
          >
              <Node member=son/>
          </For>
        </div>
      </Show>
    </div>
    <Action action=member.action/>
    }
}

#[component]
fn Action(action: RwSignal<Option<member::Action>>) -> impl IntoView {
    #[component]
    fn Preview(action: RwSignal<Option<member::Action>>) -> impl IntoView {
        #[component]
        fn AButton<F>(value: String, click: F) -> impl IntoView
        where
            F: Fn() + 'static + Clone + Copy,
        {
            view! {
                 <button
                    on:click=move |_| click()
                    class="p-5 w-96 m-2 border-2 border-gray-400 bg-gray-950 hover:border-gray-950 rounded-lg"
                >{value}</button>
            }
        }

        view! {
            <div
              class="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-3xl text-pretty text-zinc-300 rounded-lg bg-gray-700 border-gray-400 hover:border-gray-700 grid justify-content-center justify-items-center gap-5 p-5 mx-32 my-10 border-4 z-10"
            >
              <AButton value="اضافة ابن".to_string() click=move || action.set(Some(member::Action::Add))/>
              <AButton value="حذف الابن".to_string() click=move || action.set(Some(member::Action::Remove))/>
              <AButton value="تحديث بيانات".to_string() click=move || action.set(Some(member::Action::Update))/>
              <button
                  class="p-5 w-96 border-2 hover:border-red-950 bg-red-950 border-red-400 rounded-lg"
                  on:click=move |_| action.set(None)
              >
                  الغاء
              </button>
            </div>
        }
    }
    #[component]
    fn Action(action: RwSignal<Option<member::Action>>, children: Children) -> impl IntoView {
        view! {
        <div
          class="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-3xl text-pretty text-zinc-300 rounded-lg bg-gray-700 border-gray-500 hover:border-gray-700 grid justify-content-center justify-items-center gap-5 p-5 border-4 z-10"
        >
          {children()}
          <button
              class="bg-green-950 border-green-600 hover:border-green-950 border-2 w-56 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg"
              on:click=move |_| {}
          >
              تاكيد
          </button>
          <button
              class="bg-red-950 border-red-600 hover:border-red-950 border-2 w-56 h-20 col-span-2 text-2xl p-5 m-5 rounded-lg"
              on:click=move |_| action.set(None)
          >
              الغاء
          </button>
        </div>
        }
    }

    #[component]
    fn Add(action: RwSignal<Option<member::Action>>) -> impl IntoView {
        let is_only = RwSignal::new(true);
        let on_input = move |ev| {
            logging::log!("{}", event_target_value(&ev));
        };

        view! {
        <Action action=action>
          <input
              on:input=on_input
              class="col-span-4 placeholder:text-center placeholder-gray-400 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 p-2 text-4xl rounded-lg w-96"
              placeholder="الاسم"
              required
          />
          <select
            class=" col-span-4 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 text-4xl rounded-lg w-56"
          >
              <option value="1" class="text-center p-5 text-4xl">
                  {if is_only.get() {"ذكر"} else {"ذكور"} }
              </option>
              <option value="" class="text-center p-5 text-4xl">
                  {if is_only.get()  {"انثي"} else {"اناث"}}
              </option>
          </select>
        </Action>
        }
    }

    #[component]
    fn Remove(action: RwSignal<Option<member::Action>>) -> impl IntoView {
        let get_restored = move || Vec::<Member>::new();
        let get_removed = move || Vec::<Member>::new();
        let remove = |id: Uuid| {};
        let restore = |id: Uuid| {};

        view! {
        <Action action=action>
          <div class="border-2 col-span-4 flex flex-wrap rounded-lg bg-gray-950 border-gray-500 hover:border-gray-950">
              <For
                  each=get_restored
                  key=|x| x.id
                  let:son
              >
                <button
                  on:click=move |_| {remove(son.id)}
                  class="m-5 p-5 border-4 h-20 rounded-full bg-lime-700 hover:bg-red-700 hover:text-white"
                >{move || son.name.get()}</button>
              </For>
              <For
                  each=get_removed
                  key=|x| x.id
                  let:son
              >
                <button
                  on:click=move |_| {restore(son.id)}
                  class="m-5 p-5 border-4 h-20 rounded-full bg-lime-700 hover:bg-red-700 hover:text-white"
                >{move || son.name.get()}</button>
              </For>
          </div>
        </Action>
        }
    }
    #[component]
    fn Update(action: RwSignal<Option<member::Action>>) -> impl IntoView {
        view! {
        <Action action=action>
          <input
              class="col-span-4 placeholder:text-center placeholder-gray-400 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 p-2 text-4xl rounded-lg w-96"
              required
          />
          <select
            class=" col-span-4 bg-gray-800 border-gray-500 hover:border-gray-800 text-center border-2 mx-5 text-4xl rounded-lg w-56"
          >
            <option value="1" class="text-center p-5 text-4xl">ذكر</option>
            <option value="" class="text-center p-5 text-4xl">انثي</option>
          </select>
        </Action>
        }
    }

    view! {
    <Show when=move || action.get().is_some()>
    {
        match action.get().unwrap() {
            member::Action::Preview => view! { <Preview action=action/> },
            member::Action::Add => view! { <Add action=action/> },
            member::Action::Remove => view! { <Remove action=action/> },
            member::Action::Update => view! { <Update action=action/> },
        }
    }
    </Show>
    }
}
