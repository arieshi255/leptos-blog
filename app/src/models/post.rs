use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;
cfg_if! {
  if #[cfg(feature = "ssr")] {
    #[derive(Deserialize, Default)]
    pub struct PostMatter {
      pub title: String,
      pub author: String,
      pub date: String
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
  pub title: String,
  pub author: String,
  pub created_at: String,
  pub created_at_pretty: String,
  pub content: String,
  pub html: String,
  pub toc: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostMetadata {
  pub title: String,
  pub path: String,
}