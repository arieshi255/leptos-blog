use leptos::*;
use leptos_dom::html::Section;
use leptos_meta::*;
use leptos_router::*;

use crate::models::Post;
use crate::bindings::hljs;

#[component]
pub fn PostContent(cx: Scope, post: Post) -> impl IntoView {
  let content_ref = create_node_ref::<Section>(cx);

  create_effect(cx, move |_| {
    log!("Ran effect");
    if let Some(node) = content_ref() {
      node.on_mount(move |node| {
        hljs::highlight_all();
      });
    }
  });

  view! { cx,
    <section id="content">
      <div id="backtoposts">
        <A href="/posts">
          "Back to posts"
        </A>
        <Meta property="og:title" content={post.title.clone()}/>
        <Meta property="og:locale" content="en-us"/>
        <Meta property="og:type" content="article"/>
        <Title text={post.title.clone()}/>
      </div>
      <h1 class="post-head">{post.title.clone()}</h1>
      <h2 class="text">{post.created_at_pretty}</h2>
      <section class="toc">
        <h2 class="toc-head">"Contents"</h2>
        <div
          class="prose toc-content"
          inner_html={post.toc}
        ></div>
      </section>
      <section _ref=content_ref class="post-content prose" inner_html={post.html}>
      </section>
    </section>
  }
}