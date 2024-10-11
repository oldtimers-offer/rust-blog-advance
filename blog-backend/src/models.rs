use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct BlogPosts {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub post_title: String,
    pub post_category: String,
    pub post_short_description: String,
    pub post_description: String,
    pub post_photo: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = blog_posts)]
pub struct NewPost {
    pub post_title: String,
    pub post_category: String,
    pub post_short_description: String,
    pub post_description: String,
    pub post_photo: String,
}
