use leptos::*;

use crate::functions::post::get_post_metadata;

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
  let posts = create_resource(cx, || (), |_| async { get_post_metadata().await });
  let posts_view = move || {
    posts.with(cx, move |posts| match posts {
      Err(err) => {
        vec![
          view! { cx, <p>"Error: " {err.to_string()}</p> }.into_any()
        ]
      },
      Ok(posts) => {
        if posts.is_empty() {
          vec![
            view! { cx, <p>"No posts found."</p> }.into_any()
          ]
        } else {
          posts
            .iter()
            .map(move |post| {
              view! { cx,
                <section>
                  <a href=format!("/posts/{}", post.path) class="post-link">
                    <li class="post">
                      <div class="post-container">
                        <h4 class="post-title">
                          {&post.title}
                        </h4>
                      </div>
                      <p class="post-excerpt">
                        {"some post content.."}
                      </p>
                    </li>
                  </a>
                </section>
              }.into_any()
            }).collect::<Vec<_>>()
        }
      }
    })
  };

  view! { cx,
    <Transition fallback=move || view! { cx, <p>"Loading posts.."</p> }>
      <div id="content">
        <h1 id="posts-header">
          "Posts"
        </h1>
        <ul class="list-style-none">{posts_view}</ul>
      </div>
    </Transition>
  }
}