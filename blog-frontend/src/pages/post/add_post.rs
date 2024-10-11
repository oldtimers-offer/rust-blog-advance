use crate::components::menu::Menu;
use crate::components::post_form::PostForm;
use yew::prelude::*;

#[function_component(AddPost)]
pub fn add_post() -> Html {
    html! {
        <div class="container">
        <div class="row">
            <div class="col-sm-auto">
            <Menu />
            </div>
            <div class="col mt-3">
                        <PostForm post={None}/>
                        </div>
                </div>
        </div>
    }
}
