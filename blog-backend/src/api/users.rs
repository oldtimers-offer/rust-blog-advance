use crate::access::hash::hash_password;
use crate::api::db::{server_error, CacheConn, DbConn};
use crate::models::NewUser;
use crate::models::User;
use crate::repo::user_repo::TokenResponse;
use crate::repo::user_repo::UserRepo;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket_db_pools::Connection;

use rocket::serde::json::{json, Value};

use rocket::{get, post};

#[derive(Deserialize)]
pub struct RegisterInput {
    username: String,
    password: String,
}

#[post("/register", data = "<user>")]
pub async fn register(
    user: Json<RegisterInput>,
    mut conn: Connection<DbConn>,
) -> Result<Custom<Value>, Custom<Value>> {
    let hashed_password = hash_password(&user.password);
    let new_user = NewUser {
        username: user.username.clone(),
        password_hash: hashed_password,
    };

    UserRepo::create(&mut conn, new_user.into())
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(
    mut conn: Connection<DbConn>,
    login: Json<LoginInput>,
    cash: Connection<CacheConn>,
) -> Result<Json<TokenResponse>, status::Custom<&'static str>> {
    UserRepo::login(&mut conn, login, cash).await
}

#[get("/protected")]
pub fn protected_route(_user: User) -> &'static str {
    "This is a protected route!"
}

#[get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}
