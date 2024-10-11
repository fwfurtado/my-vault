use log::debug;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateItemRequest {
    name: String,
    url: String,
    secret: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateItemResponse {
    id: i32,
    name: String,
    url: String,
}


#[post("/", data = "<new_item>")]
pub fn new_item(new_item: Json<CreateItemRequest>) -> Json<CreateItemResponse> {
    info!("Creating new item: {new_item:?}");

    debug!("That is the secret: {secret}", secret = new_item.secret);

    Json(
        CreateItemResponse {
            id: 1,
            name: new_item.name.clone(),
            url: new_item.url.clone(),
        }
    )
}