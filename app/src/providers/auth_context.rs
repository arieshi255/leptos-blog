use leptos::*;

use crate::{functions::auth::Signup, functions::{user::get_user, auth::{Logout, Login}}, error_template::AppError, models::User};

#[derive(Clone)]
pub struct AuthContext {
  pub user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
  pub login: Action<Login, Result<Result<(), AppError>, ServerFnError>>,
  pub signup: Action<Signup, Result<Result<(), AppError>, ServerFnError>>,
  pub logout: Action<Logout, Result<(), ServerFnError>>
}

pub fn provide_auth_context(cx: Scope) {
  let login = create_server_action::<Login>(cx);
  let signup = create_server_action::<Signup>(cx);
  let logout = create_server_action::<Logout>(cx);

  let user = create_blocking_resource(
    cx,
    move || {
      (
        login.version().get(),
        signup.version().get(),
        logout.version().get()
      )
    },
    move |_| get_user(cx),
  );

  provide_context(
    cx,
    AuthContext {
      user,
      login,
      signup,
      logout
    },
  );
}