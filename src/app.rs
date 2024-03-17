use crate::error_template::{AppError, ErrorTemplate};
use leptos::{
    ev::{Event, SubmitEvent},
    logging::log,
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

    Effect::new(move |_| log!("{:#?}", name.get()));

    view! {
        <Show
            when=is_visualed
            fallback=move || view! {<NameForm is_visualed=is_visualed name=name/>}>
            <Visuals name=name.into()/>
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
fn Visuals(name: MaybeSignal<Person>) -> impl IntoView {
    let key = |k: &Person| k.key();
    let sons = {
        let name = name.clone();
        move || name.get().sons
    };
    view! {
        <div class="grid grid-cols-1 gap-5 justify-content-center justify-items-center">
            <button class="p-10 m-5 border-4 size-50 rounded-lg">{name.get().name}</button>
            <div>
                <For
                    each=sons
                    key=key
                    let:child
                >
                    <Visuals name=child.into()/>
                </For>
            </div>
        </div>
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    generation: usize,
    sons: Vec<Person>,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::new(),
            generation: 1,
            sons: vec![],
        }
    }
}

impl Person {
    fn with_sons(&mut self, names: &mut Vec<String>, generation: usize) {
        let Some(name) = names.pop() else {
            return;
        };
        let mut son = Person {
            name,
            generation,
            ..Default::default()
        };
        son.with_sons(names, generation + 1);
        self.sons = vec![son];
    }

    fn key(&self) -> String {
        self.name.clone() + &self.generation.to_string()
    }
}
