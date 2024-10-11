use crate::api::post::api_post_show;
use crate::api::post::get_posts;
use crate::api::post::BlogPost;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
#[hook]
pub fn use_posts() -> SuspensionResult<Vec<BlogPost>> {
    let posts_handle = use_state(|| None);
    let posts = (*posts_handle).clone();

    let supsension_handle = use_state(|| {
        Suspension::from_future(async move {
            match get_posts().await {
                Ok(posts) => posts_handle.set(Some(posts)),
                Err(_e) => posts_handle.set(Some(vec![])),
            }
        })
    });

    let suspension = (*supsension_handle).clone();

    if suspension.resumed() {
        return match posts {
            Some(v) => Ok(v),
            None => Err(suspension),
        };
    }
    Err(suspension)
}

#[hook]
pub fn use_post(id: i32) -> SuspensionResult<BlogPost> {
    let post_handle = use_state(|| None);
    let post = (*post_handle).clone();

    let supsension_handle = use_state(|| {
        Suspension::from_future(async move {
            match api_post_show(id).await {
                Ok(post) => post_handle.set(Some(post)),
                Err(_e) => post_handle.set(None),
            }
        })
    });

    let suspension = (*supsension_handle).clone();

    if suspension.resumed() {
        return match post {
            Some(v) => Ok(v),
            None => Err(suspension),
        };
    }
    Err(suspension)
}
