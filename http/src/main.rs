#[macro_use]
extern crate rocket;


mod routes;


#[launch]
fn rocket() -> _ {
    routes::register(rocket::build())
}
