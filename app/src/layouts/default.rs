use leptos::*;
use leptos_meta::*;

use crate::components::{Nav, NavProps};
use crate::providers::{ColorScheme, DummyContext};

#[component]
pub fn Default(cx: Scope, children: Children) -> impl IntoView {
  let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find ColorScheme");

  let theme = move || {
    match color_scheme.prefers_dark.try_get().unwrap_or(false) {
      true => "dark",
      false => ""
    }
    .to_string()
  };

  view! { cx,
    <Html attributes=AdditionalAttributes::from(vec![("data-theme", theme)])/>

    <Nav/>
    <main class="main">
      {children(cx)}
    </main>
  }
}