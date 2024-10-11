use crate::api::db::CacheConn;
use crate::models::User;
use crate::repo::user_repo::UserRepo;
use crate::DbConn;
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::Response;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // The subject (user identifier)
    pub exp: usize,  // Expiry time
}

pub static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    dotenv().ok(); // Load .env file
    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    key.into_bytes() // Convert String to Vec<u8>
});

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the token from the Authorization header
        let token = match request
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
        {
            Some(token) => token,
            None => return Outcome::Forward(Status::Unauthorized),
        };

        // Decode the token to extract the user ID
        let claims = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(&SECRET_KEY),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => token_data.claims,
            Err(_) => return Outcome::Forward(Status::Unauthorized),
        };

        let user_id = claims.sub; // Extract the user ID from claims

        // Get Postgres connection
        let mut db = request
            .guard::<Connection<DbConn>>()
            .await
            .expect("Can not connect to Postgres in request guard");

        // Get User
        let id = user_id.parse().unwrap();
        let user = UserRepo::find(&mut db, id).await.expect("Missing user");

        // Get Redis connection
        let mut redis_conn = request
            .guard::<Connection<CacheConn>>()
            .await
            .expect("Can not connect to Redis in request guard");

        // Check if token exists in Redis
        let redis_key = format!("user_token:{}", user_id);
        match redis_conn.as_mut().exists(redis_key).await {
            Ok(true) => {
                // Token is valid and exists in Redis, proceed to decode
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&SECRET_KEY),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_) => Outcome::Success(user),

                    Err(_) => Outcome::Forward(Status::Unauthorized),
                }
            }
            Ok(false) => Outcome::Forward(Status::Unauthorized), // Token doesn't exist
            Err(_) => Outcome::Forward(Status::InternalServerError), // Redis error
        }
    }
}

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Just to add CORS header via the fairing.
}
pub struct Cors;
//ALLOW ACCESS FROM FRONTEND
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}
