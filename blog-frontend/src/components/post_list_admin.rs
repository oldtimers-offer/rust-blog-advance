use crate::hooks::use_posts;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(PostListAdmin)]
pub fn post_list_admin() -> HtmlResult {
    let posts = use_posts()?;

    Ok(html! {
    <>
    <p>
        <Link<Route> to={Route::AddPost}>
        {"+ Add new post"}
        </Link<Route>>
    </p>
    <h1>{ "Blog Posts" }</h1>
        <table class="table">
            <thead>
                <th>{"Tilte"}</th>
                <th>{"Description"}</th>
                <th>{"Photo"}</th>
                <th>{"Created at"}</th>
                <th>{"Operations"}</th>
            </thead>
            <tbody>
            {posts.into_iter().map(|post|{

                html! {
                    <tr>

                        <td>{post.post_title.clone()}</td>
                        <td>{post.post_description.clone()}</td>
                        <td><img src={ post.post_photo.clone() } alt={ post.post_title.clone() } /></td>
                        <td>{post.created_at.to_string()}</td>

                        <td>
                        <Link<Route> to={Route::PostEdit { id: post.id }} classes="link-secondary">
                        {"edit"}
                        </Link<Route>>
                        <spawn_local class="mx-1">{"/"}</spawn_local>
                        <Link<Route> to={Route::PostDelete { id: post.id }} classes="link-danger">
                        {"delete"}
                        </Link<Route>>
                        </td>
                    </tr>
                }
            }).collect::<Html>()
            }
            </tbody>
        </table>
    </>
    })
}
