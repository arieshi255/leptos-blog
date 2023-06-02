use cfg_if::cfg_if;
use http::request::Parts;
use leptos::*;
use crate::{error_template::AppError, models::{SignupUser, ValidationErrors, ValidationError}};

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::async_trait;
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use crate::functions::{pool, auth};
    use crate::models::User;
    use argon2::{
      password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
      Argon2
    };
    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
    use axum::{RequestPartsExt, extract::FromRequestParts, TypedHeader, body::Body};
    use rand_core::OsRng;

    /// Hash Argon2 password
    pub fn hash_password(password: &[u8]) -> Result<String, AppError> {
      let argon2 = Argon2::default();
      let salt = SaltString::generate(&mut OsRng);
      let password_hash = argon2.hash_password(password, &salt)?.to_string();
      Ok(password_hash)
    }
    /// Verify Password
    pub fn verify_password(password: String, password2: String) -> Result<(), AppError> {
      let argon2 = Argon2::default();
      // Verify password against PHC string
      let parsed_hash = PasswordHash::new(&password)?;
      Ok(argon2.verify_password(password2.as_bytes(), &parsed_hash)?)
    }

    #[async_trait]
    impl Authentication<User, i64, SqlitePool> for User {
      async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get(userid, pool)
          .await
          .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
      }

      fn is_authenticated(&self) -> bool {
        true
      }

      fn is_active(&self) -> bool {
        true
      }

      fn is_anonymous(&self) -> bool {
        true
      }
    }

    #[derive(sqlx::FromRow,Debug, Clone)]
    pub struct SqlPermissionTokens {
      pub token: String,
    }
  }
}

#[server(Signup, "/api/auth")]
pub async fn signup(cx: Scope, username: String, password: String, password_confirm: String, redirect: Option<bool>) -> Result<Result<(), AppError>, ServerFnError> {
  use validator::Validate;

  let pool = pool(cx)?;
  let auth = auth(cx)?;

  let user = SignupUser {
    username,
    password,
    password_confirm,
    // roles: vec![
    //   UserRole{ name: "be".to_owned(), identifiers: vec![] },
    //   UserRole{ name: "b2e".to_owned(), identifiers: vec![ UserIdentifier{ id: 100000 } ] },
    //   UserRole{ name: "be".to_owned(), identifiers: vec![] },
    //   UserRole{ name: "b23333e".to_owned(), identifiers: vec![ UserIdentifier{ id: 100 } ] },
    // ]
  };

  match user.validate() {
    Ok(_) => (),
    Err(errs) => {
      let errs: ValidationErrors = (&errs).into();
      let errors = errs.parse();
      if let Some(true) = redirect {
        let errors_query = serde_qs::to_string(&errors).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
        leptos_axum::redirect(cx, format!("/auth/signup?{}", errors_query).as_str());
      }
      return Ok(Err(AppError::ValidationError(errors)))
    }
  };

  let hashed_password = hash_password(user.password.as_bytes()).unwrap();

  sqlx::query("INSERT INTO users (username, display_name, password) VALUES (?,?,?)")
    .bind(user.username.clone())
    .bind(String::from("Test"))
    .bind(hashed_password)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  let user = User::get_from_username(user.username, &pool)
    .await
    .ok_or("Signup failed")
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  auth.login_user(user.id);

  leptos_axum::redirect(cx, "/");
  
  Ok(Ok(()))
}

// pub async fn extract(cx: Scope) -> Option<(Parts, Body)> {
//   let req = use_context::<leptos_axum::LeptosRequest<Body>>(cx)?;
//   let owned_req = req.take_request()?;
//   Some(owned_req.into_parts())
// }

#[server(Login, "/api/auth")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<Result<(), AppError>, ServerFnError> {
  use crate::models::FieldErrors;

  let pool = pool(cx)?;
  let auth = auth(cx)?;

  let errors = FieldErrors{ errors: vec![ValidationError::simple("Username or password is incorrect.".to_string())] };

  let Some(user) = User::get_from_username(username, &pool)
    .await else { return Ok(Err(AppError::ValidationError(errors))) };

  match verify_password(user.password, password) {
    Ok(_) => {
      auth.login_user(user.id);
      leptos_axum::redirect(cx, "/");
      Ok(Ok(()))
    },
    Err(_) => {
      let errors_query = serde_qs::to_string(&errors).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
      leptos_axum::redirect(cx, format!("/auth/login??{}", errors_query).as_str());
      Ok(Err(AppError::ValidationError(errors)))
    }
  }
}

#[server(Logout, "/api/auth")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
  let auth = auth(cx)?;

  auth.logout_user();

  leptos_axum::redirect(cx, "/");

  Ok(())
}