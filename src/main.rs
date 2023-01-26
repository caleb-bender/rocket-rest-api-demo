#[macro_use] extern crate rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::serde::json::Json;
mod sorter;

#[get("/")]
async fn index() -> &'static str {
    "Hello from Rocket REST API!"
}

#[test]
fn index_should_return_200_status() {
    let client = Client::tracked(rocket()).expect("Error instantiating Rocket");
    let response = client.get(uri!(index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/sort", routes![sorter::sort])
        .register("/sort", catchers![sorter::unprocessable_entity])
}