use crate::{models::todo::Todo, db::db::MongoRepo};
use mongodb::{results::InsertOneResult};/// bson::oid::ObjectId
use rocket::{http::Status, serde::json::Json, State};

#[post("/", data = "<new_todo>")]
pub fn create_todo(
    db: &State<MongoRepo>,
    new_todo: Json<Todo>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Todo {
        id: None,
        title: new_todo.title.to_owned(),
        content: new_todo.content.to_owned(),
        starred: new_todo.starred.to_owned(),
    };

    let user_detail = db.create_todo(data);

    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}