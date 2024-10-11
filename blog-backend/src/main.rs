mod access;
mod api;
mod models;
mod repo;
mod schema;

use access::auth::{options, Cors};
use api::db::{CacheConn, DbConn};
use api::posts::{create_post, delete_post, get_posts, update_post, view_post};
use api::users::{login, me, protected_route, register};

use rocket::{launch, routes};
use rocket::{Build, Rocket};

use rocket_db_pools::Database;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                register,
                login,
                protected_route,
                create_post,
                delete_post,
                get_posts,
                view_post,
                update_post,
                me,
                options
            ],
        )
        .attach(Cors)
        .attach(DbConn::init())
        .attach(CacheConn::init())
}
