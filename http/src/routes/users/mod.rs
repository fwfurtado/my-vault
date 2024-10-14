use core::gateway::UserGateway;
use core::use_case::SignUp;
use rocket::fairing::AdHoc;
use rocket::routes;
use std::sync::Arc;

pub mod sign_up;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users State", |rocket| async {
        let gateway = Arc::new(UserGateway::new());
        let signup_use_case = Arc::new(SignUp::new(Arc::clone(&gateway)));

        rocket.mount("/users", routes![sign_up::endpoint])
            .manage(Arc::clone(&signup_use_case))
    })
}