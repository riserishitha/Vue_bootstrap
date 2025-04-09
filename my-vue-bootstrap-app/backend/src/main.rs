use actix_web::{web, App, HttpServer, Responder, get};
use mongodb::{Client, options::ClientOptions};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello from Rust Mongo API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    for db in client.list_database_names(None, None).await.unwrap() {
        println!("Found database: {}", db);
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
