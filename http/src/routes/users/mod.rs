use rocket::fairing::AdHoc;
use rocket::routes;

pub mod sign_up;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users Routes", |rocket| async {
        rocket.mount("/users", routes![sign_up::endpoint])
    })
}
