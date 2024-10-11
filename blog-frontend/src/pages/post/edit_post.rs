use crate::components::post_form::PostForm;
use crate::components::{header::Header, menu::Menu};
use crate::contexts::CurrentUserContext;
use crate::hooks::use_post;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: i32,
}

#[function_component(PostEdit)]
pub fn post_edit(props: &Props) -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.token {
        Some(_token) => {
            let loading = html! {
                <p>{"Loading ..."}</p>
            };

            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                        <Menu />
                        </div>
                        <div class="col mt-3">
                        <Header/>
                        <Suspense fallback={loading}>
                        <PostEditForm
                        post_id={props.post_id}
                        />
                        </Suspense>
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
        <Redirect<Route> to={Route::Login}/>
        },
    }
}

#[derive(Properties, PartialEq)]
struct PostEditFormProps {
    pub post_id: i32,
}

#[function_component(PostEditForm)]
fn post_edit_form(props: &PostEditFormProps) -> HtmlResult {
    let post = use_post(props.post_id.clone())?;

    Ok(html! {
        <PostForm post={post} />
    })
}
