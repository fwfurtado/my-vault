use rocket::{Build, Rocket};

pub mod routes;


pub fn make_rocket_server() -> Rocket<Build> {
    rocket::build()
        .attach(routes::users::stage())
        .attach(routes::items::stage())
}