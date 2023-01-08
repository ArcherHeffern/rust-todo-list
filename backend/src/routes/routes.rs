use crate::{models::{todo::Todo, count::Count}, db::db::MongoRepo};
use mongodb::{results::*, bson::oid::ObjectId};/// bson::oid::ObjectId
use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;
// use rocket::http::hyper::Request;
use rocket::Request;

// gets todos: If starred true, will get only the starred todos
// TODO: had random headers which were messing up, would not get a result
#[get("/<starred>")]
    pub fn get_todos(
        db: &State<MongoRepo>,
        starred: bool,
    ) -> Result<Json<Vec<Todo>>, Status> {
        let db_response = db.get_todos(starred);
        // print!("We got a response {}", db_response);
        match db_response {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError),
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

#[put("/resetCount")]
    pub fn reset_count(
        db: &State<MongoRepo>
    ) -> Result<Json<DeleteResult>, Status> {
        let db_response = db.delete_count();
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
    id: String,
    starred: bool
}

#[put("/toggleStarred", data="<payload>")]
    pub fn toggle_starred(
        db: &State<MongoRepo>,
        payload: Json<Task>
    ) -> Result<Json<UpdateResult>, Status> {
        let db_response = db.toggle_starred(&payload.id, payload.starred);
        match db_response {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

// updates content and title of a TODO
#[put("/updateTodo/<id>", data = "<new_todo>")]
    pub fn update_todo(
        db: &State<MongoRepo>,
        id: String,
        new_todo: Json<Todo>
    ) -> Result<Json<UpdateResult>, Status> {
        let new_todo: Todo = Todo {
          id: Some(ObjectId::parse_str(&id).unwrap()), 
          title: new_todo.title.to_owned(),
          content: new_todo.content.to_owned(),
          starred: new_todo.starred  
        };
        let todo_detail = db.update_content(&id, new_todo);
        match todo_detail {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

// increments counter by 1
#[put("/incrementCounter")]
    pub fn increment_counter(
        db: &State<MongoRepo>,
    ) -> Result<Json<i32>, Status> {

        let counter_detail = db.increment_counter();

        match counter_detail {
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

#[delete("/deleteTodo/<id>")]
    pub fn delete_todo(db: &State<MongoRepo>, id: String) -> Result<Json<DeleteResult>, Status> {
        let db_result = db.delete_todo(&id);
        match db_result {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

#[delete("/deleteAllTodos")]
    pub fn delete_all_todos(db: &State<MongoRepo>) -> Result<Json<DeleteResult>, Status> {
        let db_result = db.delete_all_todos();
        match db_result {
            Ok(res) => Ok(Json(res)),
            Err(_) => Err(Status::InternalServerError)
        }
    }

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
// deletes a todo
// #[Delete("/:id")]