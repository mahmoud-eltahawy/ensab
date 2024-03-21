use crate::{
    error_template::{AppError, ErrorTemplate},
    family_tree::{FamilyTree, NameForm},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ensab.css"/>

        // sets the document title
        <Title text="انساب"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="ftree" view=NameForm/>
                    <Route path="ftree/:name" view=FamilyTree/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Home"</h1> }
}
