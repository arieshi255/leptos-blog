use crate::error_template::*;
use crate::layouts::{Default, DefaultProps};
use crate::providers::provide_color_scheme;
use crate::routes::blog::*;
use crate::routes::{
  Home, HomeProps
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod functions;
pub mod models;
pub mod components;
pub mod layouts;
pub mod routes;
pub mod providers;
pub mod bindings;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  _ = provide_color_scheme(cx);
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context(cx);

  view! {
    cx,

    <Stylesheet href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/styles/default.min.css"/>

    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

    <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/highlight.min.js"></script>

    // sets the document title
    <Title text="Welcome to Leptos"/>

    // content for this welcome page
    <Router>
      <main>
        <Routes>
          <Route
            path=""
            view=|cx| {
                view! { cx,
                  <Default>
                      <ErrorBoundary fallback=|cx, errors| {
                        view! { cx, <ErrorTemplate errors=errors/> }
                      }>
                        <Outlet/>
                      </ErrorBoundary>
                  </Default>
                }
              }
          >
            <Route path="" view=|cx| view! { cx, <Home/> }/>
            <Route path="posts" view=|cx| view!{ cx, <Blog/> } ssr=SsrMode::PartiallyBlocked />
            <Route path="posts/:slug" view=|cx| view!{ cx, <Post/> } ssr=SsrMode::PartiallyBlocked />
          </Route>
        </Routes>
      </main>
    </Router>
  }
}