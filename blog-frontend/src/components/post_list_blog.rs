use crate::hooks::use_posts;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PostListBlog)]
pub fn post_list_blog() -> Html {
    // Fetching posts from hook, assuming it returns `Result<Vec<Post>, SomeErrorType>`
    let posts = match use_posts() {
        Ok(posts) => posts,
        Err(_) => vec![], // Handle error gracefully, show empty vector
    };

    html! {
        <div class="container mt-5">
            <h1 class="mb-4">{"Blog Posts"}</h1>
            { for posts.into_iter().map(|post| html! {
                <div class="card mb-3">
                    <img src={post.post_photo.clone()} class="card-img-top" alt="Post Image" />
                    <div class="card-body">
                        <h5 class="card-title">{ &post.post_title }</h5>
                        <p class="card-text">{ &post.post_short_description }</p>
                        <Link<Route> to={Route::PostPage { id: post.id }} classes="btn btn-primary">
                            { "Read More" }
                        </Link<Route>>
                    </div>
                </div>
            }) }

            // Pagination (this is a simple example, you can improve it)
            <nav>
                <ul class="pagination">
                    <li class="page-item"><a class="page-link" href="/page/{0}">{"Previous"}</a></li>
                    <li class="page-item"><a class="page-link" href="/page/{1}">{"Next"}</a></li>
                </ul>
            </nav>
        </div>
    }
}
