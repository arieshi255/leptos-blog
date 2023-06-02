use crate::components::ProgressBar;
use crate::error_template::*;
use crate::layouts::{Default, DefaultProps};
use crate::providers::{provide_color_scheme, AuthContext};
use crate::routes::blog::*;
use crate::routes::auth::*;
use crate::routes::{
  Home, HomeProps
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use providers::provide_auth_context;

pub mod error_template;
pub mod functions;
pub mod models;
pub mod components;
pub mod layouts;
pub mod routes;
pub mod providers;
pub mod bindings;

#[server(ReadUser, "/api")]
pub async fn read_user(cx: Scope) -> Result<bool, ServerFnError> {
  Ok(true)
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  _ = provide_color_scheme(cx);
  provide_auth_context(cx);
  provide_meta_context(cx);

  let (is_routing, set_is_routing) = create_signal(cx, false);
  let auth_context = use_context::<AuthContext>(cx).expect("Failed to find AuthContext");

  view! {
    cx,

    <Stylesheet href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/styles/default.min.css"/>

    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/leptos-blog.css"/>

    <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/highlight.min.js"></script>

    // sets the document title
    <Title text="Welcome to Leptos"/>

    // content for this welcome page
    <Router set_is_routing>
      <ProgressBar is_routing class="turbo-progress-bar" max_time=std::time::Duration::from_millis(2000)/>
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
            <Route path="posts" view=|cx| view!{ cx, <Blog/> }/>
            <Route path="posts/:slug" view=|cx| view!{ cx, <Post/> }/>
            <Route path="auth/login" view=|cx| view!{ cx, <Login/> }/>
            <Route path="auth/signup" view=|cx| view!{ cx, <Signup/> }/>
            <ProtectedRoute
              path="auth/settings"
              redirect_path="/"
              condition=move |cx| auth_context.user.read(cx).map_or(false, |r| r.map_or(false, |u| u.is_some()))
              view=|cx| view! { cx, <h1>"Settings"</h1> }
            />
          </Route>
        </Routes>
      </main>
    </Router>
  }
}