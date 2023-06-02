use app::{*, functions::auth::AuthSession, models::User};
use axum::{extract::{RawQuery, Path, State}, routing::get, Router, http::{HeaderMap, Request}, body::Body as AxumBody, response::{IntoResponse, Response}, Extension};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use axum_session::{SessionSqlitePool, Session, SessionConfig, SessionLayer, DatabasePool, SessionStore};
use axum_session_auth::{AuthSessionLayer, Authentication, AuthConfig};
use std::sync::Arc;

pub mod fileserv;

async fn server_fn_handler(Extension(pool): Extension<SqlitePool>, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>) -> impl IntoResponse {
  log!("{:?}", path);

  handle_server_fns_with_context(path, headers, raw_query, move |cx| {
    provide_context(cx, auth_session.clone());
    provide_context(cx, pool.clone());
  }, request).await
}

async fn leptos_routes_handler(Extension(pool): Extension<SqlitePool>, auth_session: AuthSession, State(options): State<LeptosOptions>, req: Request<AxumBody>) -> Response {
  let handler = leptos_axum::render_app_to_stream_with_context_and_replace_blocks((options).clone(),
    move |cx| {
      provide_context(cx, auth_session.clone());
      provide_context(cx, pool.clone());
    },
    |cx| view! { cx, <App/> },
    true
  );
  handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
  simple_logger::init_with_level(log::Level::Warn).expect("couldn't initialize logging");

  let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .connect("sqlite:blog.db").await.unwrap();

  let session_config = SessionConfig::default()
    .with_table_name("axum_sessions");
  let auth_config = AuthConfig::<i64>::default();
  let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config);
  session_store.initiate().await.unwrap();

  functions::register_server_functions();

  // Setting get_configuration(None) means we'll be using cargo-leptos's env values
  // For deployment these variables are:
  // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
  // Alternately a file can be specified such as Some("Cargo.toml")
  // The file would need to be included with the executable when moved to deployment
  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

  // build our application with a route
  let app = Router::new()
    .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
    .route("/api/auth/*fn_name", get(server_fn_handler).post(server_fn_handler))
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    .fallback(file_and_error_handler)
    .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone())).with_config(auth_config))
    .layer(SessionLayer::new(session_store))
    .layer(Extension(pool))
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on http://{}", &addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
