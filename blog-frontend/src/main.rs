use crate::contexts::CurrentUserProvider;
use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod components;
mod contexts;
mod hooks;
mod pages;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[at("/add")]
    AddPost,
    #[at("/post/:id/edit")]
    PostEdit { id: i32 },
    #[at("/post/:id/delete")]
    PostDelete { id: i32 },
    #[at("/post/:id")]
    PostPage { id: i32 },
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::Login => html! { <pages::login::Login /> },
        Route::Admin => html! { <pages::admin::Admin /> },
        Route::AddPost => html! { <pages::post::add_post::AddPost /> },
        Route::PostEdit { id } => {
            html! { <pages::post::edit_post::PostEdit post_id= {id} /> }
        }
        Route::PostDelete { id } => {
            html! { <pages::post::delete_post::PostDelete post_id= {id} /> }
        }
        Route::PostPage { id } => {
            html! { <pages::post_page::PostPage post_id= {id} /> }
        }
        Route::NotFound => html! { <pages::not_found::NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <CurrentUserProvider> //GLOBAL CONTEXT
                <Switch<Route> render={switch}/>
            </CurrentUserProvider> //GLOBAL CONTEXT
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
