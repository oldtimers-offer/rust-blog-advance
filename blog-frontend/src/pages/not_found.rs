use crate::components::menu::Menu;
use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="container">
            <div class="row min-vh-100 justify-content-center align-items-center">
                <Menu />
                <img src="/not-found-page.svg" alt="not_found-logo" />
            </div>
        </div>
    }
}
