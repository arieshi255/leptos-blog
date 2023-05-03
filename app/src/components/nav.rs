use leptos::*;
use leptos_router::*;

use crate::components::{DarkModeToggle, DarkModeToggleProps};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
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
        <DarkModeToggle/>
      </ul>
    </div>
  }
}