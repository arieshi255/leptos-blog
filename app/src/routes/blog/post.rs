use leptos::*;
use leptos_router::*;

use crate::components::{PostContent, PostContentProps};
use crate::functions::post::get_post;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
  pub slug: String,
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
  let params = use_params::<PostParams>(cx);
  let post = create_resource(
    cx,
    move || params().map(|params| params.slug).ok().unwrap(),
    get_post,
  );

  view! { cx,
    <script>"hljs.highlightAll();"</script>
    <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
      { move || post.read(cx).map(|p| match p {
          Ok(Some(post)) => view! { cx, <PostContent post={post}/> }.into_view(cx),
          Ok(None) => view! { cx, <p>"Post not found"</p> }.into_view(cx),
          Err(_) => view! { cx, <p>"Server error occurred"</p> }.into_view(cx)
        })
      }
    </Transition>
  }
}