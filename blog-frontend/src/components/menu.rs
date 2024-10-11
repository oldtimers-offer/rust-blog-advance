use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let current_route = use_route::<Route>().expect("No current route defined");

    let home_classes = {
        if current_route == Route::Home {
            classes!("dropdown-item", "active")
        } else {
            classes!("dropdown-item")
        }
    };

    let login_classes = {
        if current_route == Route::Login {
            classes!("dropdown-item", "active")
        } else {
            classes!("dropdown-item")
        }
    };

    let admin_classes = {
        if current_route == Route::Admin {
            classes!("dropdown-item", "active")
        } else {
            classes!("dropdown-item")
        }
    };

    html! {
    <nav class="navbar navbar-light">
        <ul class="nav navbar-nav">
            <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_classes}>
                        {"Home"}
                    </Link<Route>>
                </li>

            <li class="nav-item">
                    <Link<Route> to={Route::Login} classes={login_classes}>
                        {"Login"}
                    </Link<Route>>
                </li>

            <li class="nav-item">
                <Link<Route> to={Route::Admin} classes={admin_classes}>
                    {"Admin"}
                </Link<Route>>
            </li>

        </ul>
    </nav>
    }
}
