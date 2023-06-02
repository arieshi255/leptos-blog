use leptos::*;

use crate::functions::{post::get_post};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(cx, 0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  let other = create_resource(
    cx,
    || (),
    move |_| get_post("blah".to_string())
  );

  view! { cx,
    <h1>"Welcome to Leptos!"</h1>
    <button on:click=on_click>"Click Me: " {count}</button>
    <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
      {_ = other.read(cx)}
    </Transition>
  }
}