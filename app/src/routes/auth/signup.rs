use leptos::*;
use leptos_router::*;

use crate::{providers::AuthContext, models::{FieldErrors, ValidationErrors}};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
  let auth_context = use_context::<AuthContext>(cx).expect("Failed to find AuthContext");
  let location = use_location(cx);

  let field_errors = move || {
    location.search.with(|query| serde_qs::from_str::<FieldErrors>(query).ok())
  };

  view! { cx,
    <h1>"Sign up"</h1>
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
    <ActionForm action=auth_context.signup>
      <div style="display: flex; flex-direction: column; align-items: center; gap: 5px;">
        <label>"Username:"</label>
        <input type="text" placeholder="Username" maxlength="32" name="username" class="auth-input" />
        {move || field_errors().map(|f| f.field("/username").map(|errs| {
          view! { cx,
            <div class="field-error">
              <ul>
                {errs
                  .iter()
                  .map(|username| {
                    view! { cx, <li>{format!("{}", username)}</li> }.into_view(cx)
                  }).collect::<Vec<_>>()
                }
              </ul>
            </div>
          }})
        )}
        <label>"Password:"</label>
        <input type="password" placeholder="Password" name="password" class="auth-input" />
        <label>"Confirm password:"</label>
        <input type="password" placeholder="Password again" name="password_confirm" class="auth-input" />
        <input type="hidden" name="redirect" value="true" class="auth-input" />
        {move || field_errors().map(|f| f.field("/password").map(|errs| {
          view! { cx,
            <div class="field-error">
              <ul>
                {errs
                  .iter()
                  .map(|password| {
                    view! { cx, <li>{format!("{}", password)}</li> }.into_view(cx)
                  }).collect::<Vec<_>>()
                }
              </ul>
            </div>
          }})
        )}

        <button
          type="submit"
          class="theme-toggle"
        >"Sign Up"</button>
      </div>
    </ActionForm>
  }
}