use rocket::fairing::AdHoc;
use rocket::routes;

mod create;
mod show;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Items State", |rocket| async {
        rocket.mount("/secrets", routes![create::new_item, show::get_item])
    })
}