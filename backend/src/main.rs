#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[put("/")]
fn put() -> &'static str {
    "Updated"
}

#[post("/<id>")]
fn post(id: &str) -> String {
    format!("{}", id)
}

#[delete("/<id>")]
fn delete(id: &str) -> String {
    format!("{id}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index, put, post, delete])
}