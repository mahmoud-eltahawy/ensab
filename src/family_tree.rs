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
    let updates = RwSignal::new(None::<Updates>);
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
        updates.set(Some(member::Updates::init(member)));
    });

    move || {
        view! {
            <Suspense>
                <ServerComp updates=updates/>
            </Suspense>
        }
    }
}

#[component]
fn ServerComp(updates: RwSignal<Option<Updates>>) -> impl IntoView {
    let save = move |_| {
        let Some(updates) = updates.get() else {
            return;
        };
        spawn_local(async move {
            updates.commit().await.unwrap();
        });
        logging::log!("save");
    };

    let reset = move |_| {
        let Some(updates) = updates.get() else {
            return;
        };
        updates.discard();
        logging::log!("reset");
    };

    move || {
        let member = move || {
            let Some(updates) = updates.get() else {
                return None;
            };
            Some(updates.copy.get())
        };
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
fn Node(member: Option<Member>) -> impl IntoView {
    provide_context(member);
    let actions_waitlist = expect_context::<member_actions::ActionsWaitlist>();
    let on_click = move |_| {
        let Some(member) = member else {
            return;
        };
        member.action.set(member::Action::default());
        actions_waitlist.take(member.id);
    };

    view! {
    <div class="flex flex-col my-10 flex-nowrap">
      <button
        on:click=on_click
        class="pt-3 pb-1 mx-5 size-50 rounded-full"
      >
        {move || member.map(|x| x.name.get())}
      </button>
      <Show
          when=move || !member.is_some_and(|x| x.sons.get().is_empty())>
        <div class="flex flex-row overflow-auto border-t-2 rounded-t-lg border-black">
          <For
              each=move || member.map(|x| x.sons.get()).unwrap_or(vec![])
              key=move |x| x.id
              let:son
          >
              <Node member=Some(son)/>
          </For>
        </div>
      </Show>
    </div>
    <Action/>
    }
}
