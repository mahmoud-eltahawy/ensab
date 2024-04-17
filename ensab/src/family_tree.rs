use std::str::FromStr;

use contracts::member::RawMember;
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

mod member;
mod member_actions;
use member::Member;
use member_actions::*;

enum IdName {
    Id(Uuid),
    Name(String),
}

#[derive(Clone)]
pub enum MemberSource {
    Server,
    Client,
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

    provide_context(member_actions::ActionsWaitlist::new());
    provide_context(member::Updates::default());

    match id_or_name() {
        IdName::Id(id) => {
            provide_context(MemberSource::Server);
            view! {
                <ServerNode id=id/>
            }
        }
        IdName::Name(name) => {
            provide_context(MemberSource::Client);
            view! {
                <ClientNode name=name/>
            }
        }
    }
}

pub type OriginalMember = Resource<(), Result<RawMember, ServerFnError>>;

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

    let member_resource: OriginalMember = Resource::once(move || get_member(id));
    let member = move || {
        member_resource
            .get()
            .and_then(|x| x.ok())
            .map(Member::from_raw)
            .unwrap_or_default()
    };
    let updates = expect_context::<member::Updates>();
    provide_context(member_resource);

    view! {
    <section class="grid justify-items-center overflow-auto">
        <h1 class="text-center m-5 text-3xl">تعديل الشجرة</h1>
        <Suspense>
            <Node member=member()/>
        </Suspense>
        <Show when=move || updates.is_dirty()>
            <button>"update"</button>
        </Show>
    </section>
    }
}

#[component]
fn ClientNode(name: String) -> impl IntoView {
    let member = Member::new(name);
    let updates = expect_context::<member::Updates>();
    view! {
    <section class="grid justify-items-center overflow-auto">
        <h1 class="text-center m-5 text-3xl">بناء الشجرة</h1>
        <Node member=member/>
        <Show when=move || updates.is_dirty()>
            <button>"save"</button>
        </Show>
    </section>
    }
}

#[component]
fn Node(member: Member) -> impl IntoView {
    provide_context(member);
    let actions_waitlist = expect_context::<member_actions::ActionsWaitlist>();
    let on_click = move |_| {
        member.action.set(member::Action::default());
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
