use crate::{
    error_template::{AppError, ErrorTemplate},
    family_tree::MemberNode,
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
            <main class="bg-gray-600">
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="exists/:id" view=MemberNode/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Home"</h1> }
}
