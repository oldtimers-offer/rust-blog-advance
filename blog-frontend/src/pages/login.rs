use crate::components::login_form::LoginForm;
use crate::components::menu::Menu;
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    //access to global context
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("can't acces to global context");

    match &current_user_ctx.user {
        Some(_) => html! {
            <Redirect<Route> to={Route::Home}/>
        },
        None => html! {
            <div class="container">
                <div class="row min-vh-100 justify-content-center align-items-center">
                    <div class="col-md-4">
                        <Menu />
                        <p class="text-center">
                            <img src="/login-image.svg" alt="login-logo" />
                        </p>
                        <LoginForm />
                    </div>
                </div>
            </div>
        },
    }
}
