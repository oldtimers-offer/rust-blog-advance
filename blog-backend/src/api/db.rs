use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};

use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;
use std::error::Error;

#[derive(Database)]
#[database("my_pg_db_name")]
pub struct DbConn(PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[derive(rocket_db_pools::Database)]
#[database("redis_name_uri")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);
