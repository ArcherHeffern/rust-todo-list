    use std::env;
    extern crate dotenv;
    use dotenv::dotenv;
    use mongodb::{
        bson::{doc, oid::ObjectId},
        results::{ InsertOneResult, UpdateResult, DeleteResult},
        sync::{Client, Collection},
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
            let db = client.database("Cluster0");
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
            let jsonified_res = db_response.map(|doc| doc.unwrap()).collect();
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
// how to handle if the id was invalid
        // sets a todo as starred or unstarred - gets the todo by ID
        pub fn toggle_starred(&self, id: &String, starred: bool) -> Result<UpdateResult, mongodb::error::Error> {
            let obj_id = ObjectId::parse_str(&id).unwrap();
            let filter = doc!{"_id": obj_id};
            let db_res = self
            .col
            .find_one(filter.to_owned(), None)?.unwrap();
            let new_todo = doc! {
                "$set": {
                "id": obj_id,
                "title": db_res.title,
                "content": db_res.content,
                "starred": starred,}
            };

            self
            .col
            .update_one(filter, new_todo, None)

        }

        // updates content and title of a todo - gets the todo by ID
        pub fn update_content(&self, id: &String, new_todo: Todo) -> Result<UpdateResult, Error> {
            let obj_id = ObjectId::parse_str(&id).unwrap();
            let filter = doc!{"_id": obj_id};
            let found = self.col.find_one(filter.to_owned(), None);
            // print!("{}", new_todo.id.);
            if let Ok(val) = found {
                print!("{}", val.unwrap().content); 
                print!("found the value");
            } else {print!("DB error")}
            let new_todo_bson = doc! {  // theres gotta be a better way of doing this
                "$set": {
                "id": new_todo.id,
                "title": new_todo.title,
                "content": new_todo.content,
                "starred": new_todo.starred,
            }
            };

            self
            .col
            .update_one(filter, new_todo_bson, None)
        }

        // increments counter by 1: If document does not exist, It will be created and set to 1
        pub fn increment_counter(&self) -> Result<i32, Error> {
            // find count
            let doc = self
            .col2
            .find_one(doc! {} , None)?; // if error, return Err

            if let Some(v) = doc {  // if DB found document: Update count by 1
                let new_doc = doc! {
                    "$set": {"_id": v.id,
                    "count": v.count + 1,}
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
    pub fn delete_todo(& self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(&id).unwrap(); 
        // converts todo to BSON 
        let query = doc! {
            "_id": obj_id 
        };

        // deletes todo
        self
        .col
        .delete_one(query, None)
    }

    pub fn delete_all_todos(&self) -> Result<DeleteResult, Error> {
        let query = doc!{};
        self
        .col
        .delete_many(query, None)
    }

    pub fn delete_count(&self) -> Result<DeleteResult, Error> {
        self
        .col2
        .delete_many(doc!{}, None)
    }

    }