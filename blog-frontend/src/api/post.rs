use super::APP_HOST;
use chrono::NaiveDateTime;
use gloo_net::http::Request;
use gloo_net::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Clone, PartialEq)]
pub enum Categories {
    World,
    Sport,
    Entertainment,
}

// Implement ToString for converting enum variants to string
impl ToString for Categories {
    fn to_string(&self) -> String {
        match self {
            Categories::World => "World".to_string(),
            Categories::Sport => "Sport".to_string(),
            Categories::Entertainment => "Entertainment".to_string(),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct BlogPost {
    pub id: i32,
    pub post_title: String,
    pub post_category: Categories,
    pub post_short_description: String,
    pub post_description: String,
    pub post_photo: String,
    pub created_at: NaiveDateTime,
}

pub async fn get_posts() -> Result<Vec<BlogPost>, Error> {
    let response = Request::get(&format!("{}/posts", APP_HOST)).send().await?;
    response.json::<Vec<BlogPost>>().await
}

pub async fn api_post_create(
    token: &String,
    p_title: String,
    p_category: String,
    p_short_description: String,
    p_description: String,
    p_photo: String,
) -> Result<BlogPost, Error> {
    let response = Request::post(&format!("{}/post", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "post_title": p_title,
            "post_category": p_category,
            "post_short_description": p_short_description,
            "post_description": p_description,
            "post_photo": p_photo
        }))?
        .send()
        .await?;

    response.json::<BlogPost>().await
}

pub async fn api_post_update(
    token: &String,
    id: i32,
    p_title: String,
    p_category: String,
    p_short_description: String,
    p_description: String,
    p_photo: String,
) -> Result<BlogPost, Error> {
    let response = Request::put(&format!("{}/post/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
                "post_title": p_title,
                "post_category": p_category,
                "post_short_description": p_short_description,
                "post_description": p_description,
                "post_photo": p_photo
        }))?
        .send()
        .await?;

    response.json::<BlogPost>().await
}

pub async fn api_post_delete(token: &String, id: i32) -> Result<(), Error> {
    let _ = Request::delete(&format!("{}/post/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    Ok(())
}

pub async fn api_post_show(id: i32) -> Result<BlogPost, Error> {
    let response = Request::get(&format!("{}/post/{}", APP_HOST, id))
        .send()
        .await?;

    response.json::<BlogPost>().await
}
