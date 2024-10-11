use crate::api::post::api_post_delete;
use crate::components::{alert::Alert, header::Header, menu::Menu};
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: i32,
}

#[function_component(PostDelete)]
pub fn post_delete(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not avaible");

    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    match &current_user_ctx.token {
        Some(token) => {
            let _cloned_id = props.post_id.clone();
            let cloned_token = token.to_owned();

            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();

                let clone_navigator = navigator.clone();
                let clone_error_message = error_message_handle.clone();
                let cloned_id = _cloned_id.clone();
                let cloned_token2 = cloned_token.clone();

                spawn_local(async move {
                    match api_post_delete(&cloned_token2, cloned_id).await {
                        Ok(()) => clone_navigator.push(&Route::Admin),
                        Err(e) => clone_error_message.set(e.to_string()),
                    }
                })
            });

            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                        <Menu />
                        </div>
                        <div class="col mt-3">
                        <Header/>
                        if error_message.len() > 0 {
                            <Alert
                            alert_type = {"danger"}
                            message ={error_message}
                            />
                        }
                        <p>
                        {"Are you sure you want to delete #"}
                        {props.post_id.clone()}
                        </p>
                        <button onclick={onclick} class="btn-danger" >{"Delete"}</button>

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
