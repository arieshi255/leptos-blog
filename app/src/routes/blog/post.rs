use leptos::*;
use leptos_router::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;

use crate::components::{PostContent, PostContentProps};
use crate::functions::post::{get_post, get_post_metadata, GetPostMetadata};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
  pub slug: String,
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
  let params = use_params::<PostParams>(cx);
  let post = create_blocking_resource(
    cx,
    move || params().map(|params| params.slug).ok().unwrap(),
    get_post,
  );

  let other = create_resource(
    cx,
    || (),
    |_| async { get_post_metadata().await }
  );

  view! { cx,
    <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
      {move || post.read(cx).map(|p| match p {
          Ok(Some(post)) => view! { cx, <PostContent post={post}/> }.into_view(cx),
          Ok(None) => view! { cx, <p>"Post not found"</p> }.into_view(cx),
          Err(_) => view! { cx, <p>"Server error occurred"</p> }.into_view(cx)
        })
      }
    </Transition>
    <Transition fallback=move || view! { cx, <p>"Loading other..."</p> }>
      {move || other.read(cx).map(|p| match p {
          Ok(post) => view! { cx, <p>"yay"</p> }.into_view(cx),
          Err(_) => view! { cx, <p>"Server error occurred"</p> }.into_view(cx)
        })
      }
    </Transition>
  }
}