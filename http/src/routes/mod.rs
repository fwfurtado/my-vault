use rocket::Responder;

pub mod secrets;
pub mod users;

#[derive(Responder, Debug)]
pub enum RouteError {
    #[response(status = 409, content_type = "json")]
    Conflict(String),

    #[response(status = 404, content_type = "json")]
    NotFound(String),

    #[response(status = 500, content_type = "json")]
    InternalServerError(String),
}
