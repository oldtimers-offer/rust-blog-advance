use crate::hooks::use_post;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: i32,
}

#[function_component(SinglePost)]
pub fn single_post(props: &Props) -> HtmlResult {
    // Fetch post data using the post ID
    let post = use_post(props.post_id)?;

    Ok(html! {
        <div class="container mt-5">
            <h1>{ post.post_title.clone() }</h1>
            <img src={post.post_photo.clone()} class="img-fluid mb-4" alt="Post Image" />
            <p>{ post.post_description.clone() }</p>
            <a href="/" class="btn btn-secondary">{"Back to Home"}</a>
        </div>
    })
}
