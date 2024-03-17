use crate::error_template::{AppError, ErrorTemplate};
use leptos::{
    ev::{Event, SubmitEvent},
    *,
};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ensab.css"/>

        // sets the document title
        <Title text="انساب"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let name = RwSignal::new(Person::default());
    let is_visualed = RwSignal::new(false);

    view! {
        <Show
            when=is_visualed
            fallback=move || view! {<NameForm is_visualed=is_visualed name=name/>}>
            <div class="grid grid-cols-1 gap-5 justify-content-center justify-items-center">
                <Visuals name=name/>
            </div>
        </Show>
    }
}

#[component]
fn NameForm(name: RwSignal<Person>, is_visualed: RwSignal<bool>) -> impl IntoView {
    let on_input = move |ev: Event| {
        let mut names = event_target_value(&ev)
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut person = Person::default();
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
            <input on:input=on_input/>
            <button>"ارني"</button>
        </form>
    }
}

#[component]
fn Visuals(name: RwSignal<Person>) -> impl IntoView {
    #[component]
    fn Sons(name: RwSignal<Person>) -> impl IntoView {
        let sons = {
            let name = name.clone();
            move || name.get().sons
        };

        let key = |k: &RwSignal<Person>| k.get().key();

        let when = {
            let sons = sons.clone();
            move || sons().len() > 0
        };
        view! {
            <>
            <Show when=when>
                <div class="flex flex-row gap-5 border-2 p-4 m-4 border-black">
                    <For
                        each=sons.clone()
                        key=key
                        let:child
                    >
                        <Visuals name=child/>
                    </For>
                </div>
            </Show>
            </>
        }
    }

    let handle_person = move |person: RwSignal<Person>| {
        logging::log!(
            "generation : {}\nsibling order {}",
            person.get().generation,
            person.get().sibling_order
        );

        person.update(|x| {
            x.sons.push(RwSignal::new(Person {
                sibling_order: x.sons.len() as i32 + 1,
                generation: x.generation + 1,
                ..Default::default()
            }))
        });
    };

    view! {
        <div class="grid grid-cols-1">
            <button
                on:click=move |_| {handle_person(name)}
                class="p-10 m-5 border-4 size-50 rounded-lg"
            >{name.get().name}</button>
            <Sons name=name/>
        </div>
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    generation: i32,
    sibling_order: i32,
    sons: Vec<RwSignal<Person>>,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::new(),
            generation: 1,
            sibling_order: 0,
            sons: vec![],
        }
    }
}

impl Person {
    fn with_sons(&mut self, names: &mut Vec<String>, generation: i32) {
        let Some(name) = names.pop() else {
            return;
        };
        let mut son = Person {
            name,
            generation,
            ..Default::default()
        };
        son.with_sons(names, generation + 1);
        self.sons = vec![RwSignal::new(son)];
    }

    fn key(&self) -> String {
        self.name.clone() + &self.generation.to_string() + &self.sibling_order.to_string()
    }
}
