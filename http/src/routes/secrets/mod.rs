use rocket::fairing::AdHoc;
use rocket::routes;

mod create;
mod show;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Secret Routes", |rocket| async {
        rocket.mount("/secrets", routes![create::create_secret, show::get_item])
    })
}
