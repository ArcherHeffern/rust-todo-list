mod db;
mod models;
mod routes;

use db::db::MongoRepo;

#[macro_use]
extern crate rocket;
// get_todos, get_starred, get_counts, set_starred, update_content, increment_count, create_todo, delete_todo}
use routes::routes::{create_todo, get_todos};

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init();
    print!("Connected to the database\n");
    rocket::build()
    .manage(db)
    .mount("/", routes![create_todo, get_todos])
    // .register("/", catchers![default_catcher])
}