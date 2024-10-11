//GLOBAL CONTEXT USED IN APP
use crate::api::user::api_me;
use crate::api::user::{LoginResponse, MeResponse, User};
use gloo_storage::SessionStorage;
use gloo_storage::Storage;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::Reducible;

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess => {
                let login_resp = action.login_response.expect("Missing login response");
                let me_resp = action.me_response.expect("Missing me response");
                //Save token via gloo-storage in browser for session
                let _ = SessionStorage::set("user_token", login_resp.token.clone());

                Self {
                    user: Some(User {
                        id: me_resp.id,
                        username: me_resp.username,
                        created_at: me_resp.created_at,
                    }),
                    token: Some(login_resp.token),
                }
                .into()
            }
            CurrentUserActions::LoginFail => {
                //clear token via gloo-storage
                SessionStorage::clear();
                Self {
                    user: None,
                    token: None,
                }
                .into()
            }
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    //session storage
    if user.user.is_none() {
        if let Ok(token) = SessionStorage::get::<String>("user_token") {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => cloned_user.dispatch(CurrentUserDispatchActions {
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(LoginResponse { token }),
                        me_response: Some(me_response),
                    }),
                    Err(_) => SessionStorage::clear(),
                }
            });
        }
    }

    html! {
            <ContextProvider<CurrentUserContext> context={user}>
                {props.children.clone()}
            </ContextProvider<CurrentUserContext>>
    }
}
