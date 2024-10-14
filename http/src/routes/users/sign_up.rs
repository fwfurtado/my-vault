use crate::routes::RouteError;
use core::use_case::{SignUp, UserError};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, Responder, State};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SignUpRequest {
    username: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SignUpResponseBody {
    id: i32,
    username: String,
}

impl Into<SignUpResponder> for SignUpResponseBody {
    fn into(self) -> SignUpResponder {
        SignUpResponder {
            inner: Json(self),
        }
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct SignUpResponder {
    inner: Json<SignUpResponseBody>,
}

#[post("/signup", data = "<new_user>")]
pub fn endpoint(new_user: Json<SignUpRequest>, use_case: &State<Arc<SignUp>>) -> Result<SignUpResponder, RouteError> {
    match use_case.execute(new_user.username.clone()) {
        Ok(user) => Ok(
            SignUpResponseBody {
                id: user.id(),
                username: user.username().clone(),
            }.into()
        ),
        Err(UserError::AlreadyExists(message)) => Err(RouteError::Conflict(message)),
    }
}