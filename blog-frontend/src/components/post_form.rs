use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::post::Categories;
use crate::api::post::{api_post_create, api_post_update, BlogPost};
use crate::components::textarea::Textarea;
use crate::components::{alert::Alert, input::Input};
use crate::contexts::CurrentUserContext;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: Option<BlogPost>,
}

#[function_component(PostForm)]
pub fn post_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not avaible");

    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let title_handle = use_state(|| {
        if let Some(p) = &props.post {
            return p.post_title.clone();
        }
        String::default()
    });

    let title = (*title_handle).clone();

    let category_handle = use_state(|| Categories::World);
    let category = (*category_handle).clone();

    let short_description_handle = use_state(|| {
        if let Some(p) = &props.post {
            return p.post_short_description.clone();
        }
        String::default()
    });

    let short_description = (*short_description_handle).clone();

    let description_handle = use_state(|| {
        if let Some(p) = &props.post {
            return p.post_description.clone();
        }
        String::default()
    });

    let description = (*description_handle).clone();

    let photo_handle = use_state(|| {
        if let Some(p) = &props.post {
            return p.post_photo.clone();
        }
        String::default()
    });

    let photo = (*photo_handle).clone();

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let title_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            title_handle.set(input.value());
        }
    });

    let category_changed = {
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            let value = input.value();
            let category = match value.as_str() {
                "World" => Categories::World,
                "Sport" => Categories::Sport,
                "Entertainment" => Categories::Entertainment,
                _ => Categories::World, // Default case
            };
            category_handle.set(category);
        })
    };

    let short_description_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(input) = target {
            short_description_handle.set(input.value());
        }
    });

    let description_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(input) = target {
            description_handle.set(input.value());
        }
    });

    let photo_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            photo_handle.set(input.value());
        }
    });

    let cloned_title = title.clone();
    let cloned_category = category.clone();
    let cloned_short_description = short_description.clone();
    let cloned_description = description.clone();
    let cloned_photo = photo.clone();

    let cloned_error_message_handle = error_message_handle.clone();
    let cloned_user_ctx = current_user_ctx.clone();
    let cloned_post = props.post.clone();

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_title = cloned_title.clone();
        let cloned_category = cloned_category.clone();
        let cloned_short_description = cloned_short_description.clone();
        let cloned_description = cloned_description.clone();
        let cloned_photo = cloned_photo.clone();
        let cloned_error_message_handle = cloned_error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = cloned_user_ctx.clone();
        let cloned_post = cloned_post.clone();

        match &cloned_user_ctx.token {
            Some(token) => {
                let cloned_token = token.clone();

                spawn_local(async move {
                    if let Some(post) = cloned_post {
                        match api_post_update(
                            &cloned_token,
                            post.id.clone(),
                            cloned_title.clone(),
                            cloned_category.to_string(),
                            cloned_short_description.clone(),
                            cloned_description.clone(),
                            cloned_photo.clone(),
                        )
                        .await
                        {
                            Ok(_post) => cloned_navigator.push(&Route::Admin),
                            Err(e) => cloned_error_message_handle.set(e.to_string()),
                        }
                    } else {
                        match api_post_create(
                            &cloned_token,
                            cloned_title.clone(),
                            cloned_category.to_string(),
                            cloned_short_description.clone(),
                            cloned_description.clone(),
                            cloned_photo.clone(),
                        )
                        .await
                        {
                            Ok(_rustacean) => cloned_navigator.push(&Route::Admin),
                            Err(e) => cloned_error_message_handle.set(e.to_string()),
                        }
                    };
                });
            }
            None => {
                cloned_error_message_handle.set("Session expired. Please login again".to_string())
            }
        }
    });
    html! {
        <form onsubmit = {onsubmit}>

        if error_message.len() > 0 {
            <Alert
            alert_type = {"danger"}
            message ={error_message}
            />
        }
            <div class="mb-3">
                <Input
                input_type="text"
                name="title"
                label="Title"
                value={title}
                onchange={title_changed}
                />
            </div>
            <div class="mb-3">
            <label for="category-select">{ "Select a category: " }</label>
            <select id="category-select" onchange={category_changed}>
            <option value={Categories::World.to_string()}>{ "World" }</option>
            <option value={Categories::Sport.to_string()}>{ "Sport" }</option>
            <option value={Categories::Entertainment.to_string()}>{ "Entertainment" }</option>
            </select>
            </div>
            <div class="mb-3">
                <Textarea
                name="short_description"
                label="Short Description"
                value={short_description}
                onchange={short_description_changed}
                rows="5"
                />
            </div>
            <div class="mb-3">
                <Textarea
                name="description"
                label="Description"
                value={description}
                onchange={description_changed}
                rows="10"
                />
            </div>
            <div class="mb-3">
                <Input
                input_type="text"
                name="photo"
                label="Photo"
                value={photo}
                onchange={photo_changed}
                />
            </div>
            <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}
