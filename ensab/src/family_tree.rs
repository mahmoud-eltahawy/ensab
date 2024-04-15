use std::str::FromStr;

use contracts::member::RawMember;
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

use crate::family_tree::member::Member;

mod member;

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
    let on_click = move |_| logging::log!("member().action()");

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
    }
}
