use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::*;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

use crate::models::{ValidationError, FieldErrors};

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum AppError {
  #[error("Not Found")]
  NotFound,
  #[error("Failed to sign up")]
  SignUpError(String),
  #[error("Validation error occurred")]
  ValidationError(FieldErrors),
  #[error("Sqlx error occurred")]
  SqlxError(String),
  #[error("Argon2 error occurred")]
  Argon2Error(String),
  #[error("Error occurred")]
  OtherError(String),
}

impl AppError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      AppError::NotFound => StatusCode::NOT_FOUND,
      AppError::SignUpError(_) => StatusCode::UNAUTHORIZED,
      AppError::ValidationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::Argon2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::OtherError(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
  }
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    impl From<serde_json::Error> for AppError {
      fn from(value: serde_json::Error) -> Self {
          Self::OtherError(value.to_string())
      }
    }
    impl From<ServerFnError> for AppError {
      fn from(value: ServerFnError) -> Self {
          Self::OtherError(value.to_string())
      }
    }
    impl From<sqlx::Error> for AppError {
      fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value.to_string())
      }
    }
    impl From<argon2::password_hash::Error> for AppError {
      fn from(error: argon2::password_hash::Error) -> Self {
        Self::Argon2Error(error.to_string())
      }
    }
  }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
  cx: Scope,
  #[prop(optional)] outside_errors: Option<Errors>,
  #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
  let errors = match outside_errors {
    Some(e) => create_rw_signal(cx, e),
    None => match errors {
      Some(e) => e,
      None => panic!("No Errors found and we expected errors!"),
    },
  };
  // Get Errors from Signal
  let errors = errors.get();

  // Downcast lets us take a type that implements `std::error::Error`
  // let errors: Vec<AppError> = errors
  //   .into_iter()
  //   .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
  //   .collect();
  let errors: Vec<_> = errors
    .into_iter()
    .map(|(_k, v)| v)
    .collect();
  
  println!("Errors: {errors:#?}");

  // Only the response code for the first error is actually sent from the server
  // this may be customized by the specific application
  cfg_if! { if #[cfg(feature="ssr")] {
    let response = use_context::<ResponseOptions>(cx);
    if let Some(response) = response {
      if let Some(app_error) = errors[0].downcast_ref::<AppError>() {
        response.set_status(app_error.status_code());
      } else {
        response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
      }
    }
  }}

  view! { cx,
    <h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
    <For
      // a function that returns the items we're iterating over; a signal is fine
      each= move || {errors.clone().into_iter().enumerate()}
      // a unique key for each item as a reference
      key=|(index, _error)| *index
      // renders each item to a view
      view= move |cx, error| {
        let error_string = error.1.to_string();
        let error_code = error.1.downcast_ref::<AppError>().map(|app_error| app_error.status_code());
        view! { cx,
          <div class="error-container">
            {error_code.map(|ec| view!{ cx, <h2>{ec.to_string()}</h2> }.into_any())}
            <p>"Error: " {error_string}</p>
          </div>
        }
      }
    />
  }
}
