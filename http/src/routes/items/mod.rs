use rocket::fairing::AdHoc;
use rocket::routes;

mod create;
mod show;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Items State", |rocket| async {
        rocket.mount("/items", routes![create::new_item, show::get_item])
    })
}