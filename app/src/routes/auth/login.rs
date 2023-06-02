use leptos::*;
use leptos_router::*;

use crate::{providers::AuthContext, models::{FieldErrors}};

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
  let auth_context = use_context::<AuthContext>(cx).expect("Failed to find AuthContext");
  let location = use_location(cx);

  let field_errors = move || {
    location.search.with(|query| serde_qs::from_str::<FieldErrors>(query).ok())
  };

  view! { cx,
    <h1>"Login"</h1>
    {move || field_errors().map(|fields| {
      view! { cx,
        <div class="error-container field-error">
          <ul>
            {fields
              .errors
              .into_iter()
              .map(move |err| {
                view! { cx, <li>{format!("{}", err)}</li> }.into_any()
              }).collect::<Vec<_>>()
            }
          </ul>
        </div>
      }
  })}
    <ActionForm action=auth_context.login>
      <div style="display: flex; flex-direction: column; align-items: center; gap: 5px;">
        <label>"Username:"</label>
        <input type="text" placeholder="Username" maxlength="32" name="username" class="auth-input" />
        <label>"Password:"</label>
        <input type="password" placeholder="Password" name="password" class="auth-input" />

        <button
          type="submit"
          class="theme-toggle"
        >"Login"</button>
      </div>
    </ActionForm>
  }
}