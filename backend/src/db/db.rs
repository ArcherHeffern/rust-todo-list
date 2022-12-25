    use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{doc, oid::ObjectId},
        results::{ InsertOneResult, UpdateResult, DeleteResult},
        sync::{Client, Collection, Cursor},
        error::Error
        
    };

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
            let client = Client::with_uri_str(uri).unwrap();    // !how to handle None?
            let db = client.database("cluster0");
            let col: Collection<Todo> = db.collection("Todo");
            let col2: Collection<Count> = db.collection("Count");
            MongoRepo { col, col2 }
        }
        
        // gets all starred todos
        pub fn get_todos(&self, starred: bool) -> Result<Vec<Todo>, Error> {
            let filter = doc! {"starred": starred};
            let db_response = self
            .col
            .find(filter, None)?;
            let jsonified_res = db_response.map(|doc| doc.unwrap()).collect(); //! better error handling
            Ok(jsonified_res)
        } 

        // gets counter value: If DB does not find count doc, we will create a new 
        // doc with initial value set to 0
        pub fn get_count(& self) -> Result<i32, mongodb::error::Error> {
            let count = self
            .col2
            .find_one(doc!{}, None)?;

            match count {
                Some(val) => Ok(val.count),
                // if db does not contain any counts: Create a count with initial value set to 0
                None => {
                    let new_count = Count {
                        id: None,
                        count: 0
                    };
                    self
                    .col2
                    .insert_one(new_count, None)?;
                    Ok(0)
                }
            }
        }

        // sets a todo as starred or unstarred - gets the todo by ID
        pub fn toggle_starred(&self, todo: Todo, starred: bool) -> Result<UpdateResult, mongodb::error::Error> {
            let new_todo = doc! {
                "id": todo.id,
                "title": todo.title,
                "content": todo.content,
                "starred": starred,
            };

            self
            .col
            .update_one(doc!{"id": todo.id}, new_todo, None)

        }

        // updates content and title of a todo - gets the todo by ID
        pub fn update_content(&self, new_todo: Todo) -> Result<UpdateResult, Error> {

            let new_todo_bson = doc! {  // theres gotta be a better way of doing this
                "id": new_todo.id,
                "title": new_todo.id,
                "content": new_todo.content,
                "starred": new_todo.starred
            };

            self
            .col
            .update_one(doc! {"id": new_todo.id}, new_todo_bson, None)
        }

        // increments counter by 1: If document does not exist, It will be created and set to 1
        pub fn increment_counter(&self) -> Result<i32, Error> {
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

        // creates a todo
     pub fn create_todo(& self, new_todo: Todo) -> Result<InsertOneResult, Error> {
            let new_todo = Todo {
                id: None,
                ..new_todo
            };
            self
                .col
                .insert_one(new_todo, None)
        }
    
    // deletes a todo by ID
    pub fn delete_todo(& self, id: Option<ObjectId>) -> Result<DeleteResult, Error> {

        // converts todo to BSON 
        let query = doc! {
            "id": id
        };

        // deletes todo
        self
        .col
        .delete_one(query, None)
    }

    }