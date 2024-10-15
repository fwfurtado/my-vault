use crate::routes::secrets::create::CreateSecretResponse;
use crate::routes::secrets::CreateSecretResponseBody;
use core::use_case::SecretUseCase;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, Responder, State};
use std::sync::Arc;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetSecretResponseBody {
    url: String,
    value: String,
}

impl Into<GetSecretResponse> for GetSecretResponseBody {
    fn into(self) -> GetSecretResponse {
        GetSecretResponse { inner: Json(self) }
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetSecretResponse {
    inner: Json<GetSecretResponseBody>,
}

#[get("/?<url>")]
pub fn get_item_by_url(
    url: String,
    use_case: &State<Arc<SecretUseCase>>,
) -> Option<GetSecretResponse> {
    use_case.show_secret(url.clone()).ok().map(|secret| {
        GetSecretResponseBody {
            url: secret.url().clone(),
            value: secret.value().clone(),
        }
        .into()
    })
}
