use core::gateway::{
    new_key_gateway, new_secret_gateway, new_user_gateway, KeyGateway, SecretGateway, UserGateway,
};
use core::use_case::{SecretUseCase, SignUp};
use database::repository::{
    new_key_repository, new_secret_repository, new_user_repository, KeyRepository,
    SecretRepository, UserRepository,
};
use rocket::{Build, Rocket};
use std::sync::Arc;

pub mod routes;

pub fn make_rocket_server() -> Rocket<Build> {
    let user_repository: Arc<dyn UserRepository> = Arc::new(new_user_repository());
    let key_repository: Arc<dyn KeyRepository> = Arc::new(new_key_repository());

    let user_gateway: Arc<dyn UserGateway> = Arc::new(new_user_gateway(user_repository));
    let key_gateway: Arc<dyn KeyGateway> = Arc::new(new_key_gateway(key_repository));

    let signup_use_case = Arc::new(SignUp::new(
        Arc::clone(&user_gateway),
        Arc::clone(&key_gateway),
    ));

    let secret_repository: Arc<dyn SecretRepository> = Arc::new(new_secret_repository());

    let secret_gateway: Arc<dyn SecretGateway> = Arc::new(new_secret_gateway(secret_repository));

    let secret_use_case = Arc::new(SecretUseCase::new(
        Arc::clone(&secret_gateway),
        Arc::clone(&key_gateway),
    ));

    rocket::build()
        .attach(routes::users::stage())
        .attach(routes::secrets::stage())
        .manage(Arc::clone(&signup_use_case))
        .manage(Arc::clone(&secret_use_case))
}
