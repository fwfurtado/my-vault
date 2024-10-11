use rocket::{Build, Rocket};

mod items;


pub fn register(builder: Rocket<Build>) -> Rocket<Build> {
    items::register(builder)
}