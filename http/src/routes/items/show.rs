use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetItemResponse {
    id: i32,
    name: String,
    url: String,
}


#[get("/<id>")]
pub fn get_item(id: i32) -> Json<GetItemResponse> {
    Json(
        GetItemResponse {
            id,
            name: "Item".to_string(),
            url: "https://example.com".to_string(),
        }
    )
}