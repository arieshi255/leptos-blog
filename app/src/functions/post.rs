use crate::models::{Post, PostMetadata};
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use crate::models::post::PostMatter;
    use femark::HTMLOutput;
    use gray_matter::Matter;
    use gray_matter::engine::YAML;
    use chrono::naive::NaiveDate;

    fn into_post(md: String) -> Post {
      let matter = Matter::<YAML>::new();
      let result = matter.parse(&md);
      let HTMLOutput{content, toc} = femark::process_markdown_to_html(result.content.clone()).unwrap_or_default();
      let post_matter: PostMatter = match result.data {
        Some(data) => data.deserialize().unwrap_or_default(),
        None => PostMatter::default()
      };

      Post {
        title: post_matter.title,
        author: post_matter.author,
        created_at: post_matter.date.clone(),
        created_at_pretty: NaiveDate::parse_from_str(&post_matter.date, "%Y-%m-%d").unwrap_or_default().format("%a %b %d %Y").to_string(),
        content: result.content,
        html: content,
        toc
      }
    }
  }
}

#[server(GetPostMetadata, "/api", "GetJson")]
pub async fn get_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {
  use serde_json::Error;
  use tokio::fs::read_to_string;

  let data = read_to_string("./posts/index.json")
    .await
    .expect("Error reading file");
  let json: Result<Vec<PostMetadata>, Error> = serde_json::from_str(&data);

  std::thread::sleep(std::time::Duration::from_millis(1250));

  json.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
  use crate::functions::post::into_post;
  use tokio::fs::read_to_string;

  let data = read_to_string(format!("./posts/{}.md", slug)).await;

  let post = match data {
    Ok(r) => Some(into_post(r)),
    Err(_) => None,
  };

  std::thread::sleep(std::time::Duration::from_millis(700));

  Ok(post)
}