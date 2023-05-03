pub mod post;
pub mod dark_mode;

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use leptos::*;

    pub fn register_server_functions() {
      _ = post::GetPostMetadata::register();
      _ = post::GetPost::register();
      _ = dark_mode::ToggleDarkMode::register();
    }
  }
}