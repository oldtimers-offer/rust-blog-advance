//use crate::components::header::Header;
use crate::components::menu::Menu;
use crate::components::post_list_blog::PostListBlog;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
                <div class="row">
                    <div class="col-sm-auto">
                        <Menu/>
                    </div>
                    <div class="col-mt-3">
                        // <Header />
                        <PostListBlog />
                    </div>
                </div>
        </div>
    }
}
