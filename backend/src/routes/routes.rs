use crate::{models::{todo::Todo, count::Count}, db::db::MongoRepo};
use mongodb::{results::*, bson::oid::ObjectId};/// bson::oid::ObjectId
use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

// gets todos: If starred true, will get only the starred todos
#[get("/<starred>")]
    pub fn get_todos(
        db: &State<MongoRepo>,
        starred: bool,
    ) -> Result<Json<Vec<Todo>>, Status> {
        let db_response = db.get_todos(starred);
        match db_response {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

// gets count of total todos completed
#[get("/getCount")]
    pub fn get_counts(
        db: &State<MongoRepo>
    ) -> Result<Json<i32>, Status> {
        let db_response = db.get_count();
        match db_response {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }
// updates a TODO to be starred or unstarred
// params: id, starred: Boolean

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    id: Option<ObjectId>,
    starred: bool
}

#[put("/toggle_starred", data="<payload>")]
    pub fn toggle_starred(
        db: &State<MongoRepo>,
        payload: Json<Task>
    ) -> Result<Json<UpdateResult>, Status> {
        let db_response = db.toggle_starred(payload.id, payload.starred);
        match db_response {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

// updates content and title of a TODO
//TODO

// increments counter by 1
#[put("/counter")]
    pub fn update_counter(
        db: &State<MongoRepo>,
    ) -> Result<Json<i32>, Status> {

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
// #[Delete("/:id")]