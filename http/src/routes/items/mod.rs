mod create;
mod show;


pub fn register(builder: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    builder.mount("/items", routes![create::new_item, show::get_item])
}