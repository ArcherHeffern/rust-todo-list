    use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{doc, extjson::de::Error, oid::ObjectId},
        results::{ InsertOneResult, UpdateResult},
        sync::{Client, Collection, Cursor},
        
    };
    use rocket::serde::json::Json;
    use crate::models::{todo::Todo, count::Count};

    pub struct MongoRepo {
        col: Collection<Todo>,
        col2: Collection<Count>
    }

    impl MongoRepo {
        // connects to database
        pub fn init() -> Self {
            dotenv().ok();
            let uri = match env::var("MONGO_URI") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
            let client = Client::with_uri_str(uri).unwrap();    // !bad error handling
            let db = client.database("cluster0");
            let col: Collection<Todo> = db.collection("Todo");
            let col2: Collection<Count> = db.collection("Count");
            MongoRepo { col, col2 }
        }
        
        // gets all todos 
        pub fn get_todos(&self) -> Result<Cursor<Todo>, mongodb::error::Error> {
            self
            .col
            .find(doc!{}, None)
        } 

        // gets all starred todos
        pub fn get_starred_todos(&self) -> Result<Cursor<Todo>, mongodb::error::Error> {
            let filter = doc! {};
            self
            .col
            .find(filter, None)
        } 

        // gets counter value: If DB does not find count doc, we will create a new 
        // doc with initial value set to 0

        // sets a todo as starred or unstarred - gets the todo by ID

        // updates content and title of a todo - gets the todo by ID

        // increments counter by 1: If document does not exist, It will be created and set to 1
        pub fn increment_counter(&self) -> Result<i32, mongodb::error::Error> {
            // find count
            let doc = self
            .col2
            .find_one(doc! {} , None)?; // if error, return Err

            if let Some(v) = doc {  // if DB found document: Update count by 1
                let new_doc = doc! {
                    "count": v.count + 1,
                    "id": v.id
                };
            self
            .col2
            .update_one(doc!{}, new_doc, None)?;    // if error, return Err
            return Ok(v.count + 1);
            } else {    // otherwise: Create document and set value to 1
                self
                .col2
                .insert_one(Count {id : None, count: 1}, None)?;    // if error, return Err
                return Ok(1);
            }
        }

     pub fn create_todo(& self, new_todo: Todo) -> Result<InsertOneResult, mongodb::error::Error> {
            let new_todo = Todo {
                id: None,
                ..new_todo
            };
            self
                .col
                .insert_one(new_todo, None)
        }
    
    // deletes a todo by ID

    }