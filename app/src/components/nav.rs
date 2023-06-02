use leptos::*;
use leptos_router::*;

use crate::{components::{DarkModeToggle, DarkModeToggleProps}, providers::AuthContext};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
  let auth_context = use_context::<AuthContext>(cx).expect("AuthContext not found");

  view! { cx,
    <div class="top-nav">
      <div>
        "Logo Here"
      </div>
      <input id="menu-toggle" type="checkbox"/>
      <label class="menu-button-container" for="menu-toggle">
      <div class="menu-button"/>
    </label>
      <ul class="menu">
        <li>
          <A href="/">"Home"</A>
        </li>
        <li>
          <A href="/posts">"Posts"</A>
        </li>
        <li>
          <A href="/auth/signup">"Sign up"</A>
        </li>
        <Transition fallback=move || ()>
          {move || {
            let user = move || match auth_context.user.read(cx) {
              Some(Ok(Some(user))) => Some(user),
              _ => None
            };

            if user().is_some() {
              view! { cx,
                <li>
                  <A href="/auth/settings">"Settings"</A>
                </li>
              }
            } else {
              view! { cx,
                <li>
                  <A href="/auth/login">"Login"</A>
                </li>
              }
            }
          }}
        </Transition>
        <DarkModeToggle/>
      </ul>
    </div>
  }
}