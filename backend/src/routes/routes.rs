use crate::{models::{todo::Todo, count::Count}, db::db::MongoRepo};
use mongodb::{results::*};/// bson::oid::ObjectId
use rocket::{http::Status, serde::json::Json, State};

// gets todos: If starred true, will get only the starred todos
#[get("/", data="<starred>")]
    pub fn get_todos(
        db: &State<MongoRepo>,
        starred: bool,
    ) -> Result<Json<GetResult>, Status> {

    }

// gets count of total todos completed

// updates a TODO to be starred or unstarred

// updates content and title of a TODO

// increments counter by 1
#[put("/counter")]
    pub fn update_counter(
        db: &State<MongoRepo>,
    ) -> Result<Json<UpdateResult>, Status> {

        let user_detail = db.increment_counter();

        match user_detail {
            Ok(counter) => Ok(Json(counter)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

// creates a new todo item
#[post("/", data = "<new_todo>")]
pub fn create_todo(
    db: &State<MongoRepo>,
    new_todo: Json<Todo>
) -> Result<Json<InsertOneResult>, Status> {
    let data = Todo {
        id: None,
        title: new_todo.title.to_owned(),
        content: new_todo.content.to_owned(),
        starred: new_todo.starred.to_owned(),
    };

    let todo_detail= db.create_todo(data);

    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// deletes a todo