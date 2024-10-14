use core::gateway::{new_key_gateway, new_user_gateway, KeyGateway, UserGateway};
use core::use_case::SignUp;
use database::repository::key::new_key_repository;
use database::repository::user::new_user_repository;
use database::repository::{KeyRepository, UserRepository};
use rocket::fairing::AdHoc;
use rocket::routes;
use std::sync::Arc;

pub mod sign_up;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users State", |rocket| async {
        let user_repository = Arc::new(new_user_repository());
        let key_repository = Arc::new(new_key_repository());

        let user_gateway = Arc::new(new_user_gateway(user_repository));
        let key_gateway = Arc::new(new_key_gateway(key_repository));


        let signup_use_case = Arc::new(SignUp::new(user_gateway, key_gateway));

        rocket.mount("/users", routes![sign_up::endpoint])
            .manage(Arc::clone(&signup_use_case))
    })
}