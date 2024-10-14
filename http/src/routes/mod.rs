use rocket::Responder;

pub mod secrets;
pub mod users;


#[derive(Responder, Debug)]
pub enum RouteError {
    #[response(status = 409, content_type = "json")]
    Conflict(String),
}