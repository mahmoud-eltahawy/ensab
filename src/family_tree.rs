use std::str::FromStr;

use crate::db::member::RawMember;
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

mod member;
mod member_actions;
use member::Member;
use member_actions::*;

use self::member::Updates;

#[server(encoding = "Cbor")]
async fn get_member(id: Uuid) -> Result<RawMember, ServerFnError> {
    use crate::db::{member::read, Pool, Postgres};
    let pool = expect_context::<Pool<Postgres>>();
    match read(&pool, id).await {
        Ok(member) => Ok(member),
        Err(err) => Err(ServerFnError::ServerError(err.to_string())),
    }
}

#[component]
pub fn MemberNode() -> impl IntoView {
    let updates = RwSignal::new(Updates::default());
    let params = use_params_map();
    let id = move || {
        let id = params.with(|x| x.get("id").cloned()).unwrap();
        Uuid::from_str(&id).unwrap()
    };

    provide_context(member_actions::ActionsWaitlist::new());

    let member_resource = Resource::once(move || get_member(id()));
    Effect::new(move |_| {
        let member = member_resource
            .get()
            .and_then(Result::ok)
            .map(Member::from_raw)
            .unwrap_or_default();
        updates.set(member::Updates::init(member));
    });

    view! {
        <ServerComp updates=updates/>
    }
}

#[component]
fn ServerComp(updates: RwSignal<Updates>) -> impl IntoView {
    let save = move |_| {
        spawn_local(async move {
            updates.get().commit().await.unwrap();
        });
    };

    let reset = move |_| {
        updates.get().discard();
    };

    move || {
        let member = move || updates.get().copy.get();
        view! {
            <section class="grid justify-items-center overflow-auto">
                <h1 class="text-center m-5 text-3xl">تعديل الشجرة</h1>
                <Node member=member()/>
                <div class="grid justify-items-center overflow-auto">
                    <button on:click=save>"save"</button>
                    <button on:click=reset>"reset"</button>
                </div>
            </section>
        }
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
    <Action/>
    }
}
