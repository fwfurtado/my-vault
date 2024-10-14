use crate::routes::RouteError;
use core::use_case::{SecretError, SecretUseCase};
use log::debug;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{info, post, Responder, State};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateSecretRequest {
    url: String,
    secret: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateSecretResponseBody {
    id: i32,
}

impl Into<CreateSecretResponse> for CreateSecretResponseBody {
    fn into(self) -> CreateSecretResponse {
        CreateSecretResponse { inner: Json(self) }
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateSecretResponse {
    inner: Json<CreateSecretResponseBody>,
}

#[post("/", data = "<new_secret>")]
pub fn create_secret(
    new_secret: Json<CreateSecretRequest>,
    use_case: &State<Arc<SecretUseCase>>,
) -> Result<CreateSecretResponse, RouteError> {
    info!("Creating new item: {new_secret:?}");

    debug!("That is the secret: {secret}", secret = new_secret.secret);

    let username = "user".to_string().clone(); //TODO: get username from token
    let url = new_secret.url.clone();
    let password = new_secret.secret.clone();

    match use_case.create_secret(username, url, password) {
        Ok(id) => Ok(CreateSecretResponseBody { id }.into()),
        Err(SecretError::UserNotFound) => Err(RouteError::NotFound("not found".to_string())),
        Err(_) => Err(RouteError::InternalServerError(
            "internal server error".to_string(),
        )),
    }
}
