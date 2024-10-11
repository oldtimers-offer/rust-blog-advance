use crate::api::db::{server_error, DbConn};
use crate::models::User;
use crate::models::{BlogPosts, NewPost};
use crate::repo::blog_repo::BlogRepo;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/posts")]
pub async fn get_posts(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    BlogRepo::load(&mut db, 100)
        .await
        .map(|posts| json!(posts))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/post/<id>")]
pub async fn view_post(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    BlogRepo::find(&mut db, id)
        .await
        .map(|post| json!(post))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/post", format = "json", data = "<new_post>")]
pub async fn create_post(
    mut db: Connection<DbConn>,
    new_post: Json<NewPost>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    BlogRepo::create(&mut db, new_post.into_inner())
        .await
        .map(|new_post| Custom(Status::Created, json!(new_post)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/post/<id>", format = "json", data = "<post>")]
pub async fn update_post(
    mut db: Connection<DbConn>,
    id: i32,
    post: Json<BlogPosts>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    BlogRepo::update(&mut db, id, post.into_inner())
        .await
        .map(|post| json!(post))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/post/<id>")]
pub async fn delete_post(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    BlogRepo::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
