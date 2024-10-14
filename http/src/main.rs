use http::make_rocket_server;

#[rocket::main]
async fn main() {
    make_rocket_server()
        .launch()
        .await
        .expect("valid rocket server");
}