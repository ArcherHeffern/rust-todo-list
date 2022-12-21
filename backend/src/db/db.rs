    use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{extjson::de::Error},
        results::{ InsertOneResult},
        sync::{Client, Collection},
    };
    use crate::models::todo::Todo;

    pub struct MongoRepo {
        col: Collection<Todo>,
    }

    impl MongoRepo {
        // connects to database
        pub fn init() -> Self {
            dotenv().ok();
            let uri = match env::var("MONGO_URI") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
            let client = Client::with_uri_str(uri).unwrap();
            let db = client.database("rustDB");
            let col: Collection<Todo> = db.collection("User");
            MongoRepo { col }
        }
        
        // creates new TODO
        pub fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error> {
            let new_todo = Todo {
                id: None,
                title: new_todo.title,
                content: new_todo.content,
                starred: new_todo.starred
            };
            let user = self
                .col
                .insert_one(new_todo, None)
                .ok()
                .expect("Error creating user"); // ! better error handeling
            Ok(user)
        }

        // TODO other things!!!

    }