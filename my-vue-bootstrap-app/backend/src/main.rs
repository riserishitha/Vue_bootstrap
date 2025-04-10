use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, post, put, delete};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;
use futures::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
}

type Db = web::Data<Collection<Task>>;

#[get("/tasks")]
async fn get_tasks(db: Db) -> impl Responder {
    match db.find(None, None).await {
        Ok(cursor) => {
            let tasks: Vec<Task> = cursor.try_collect().await.unwrap_or_default();
            HttpResponse::Ok().json(tasks)
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch tasks"),
    }
}

#[post("/tasks")]
async fn create_task(db: Db, task: web::Json<Task>) -> impl Responder {
    let new_task = Task {
        id: None,
        name: task.name.clone(),
    };

    match db.insert_one(new_task, None).await {
        Ok(_) => HttpResponse::Ok().body("Task added"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add task"),
    }
}

#[put("/tasks/{id}")]
async fn update_task(db: Db, path: web::Path<String>, task: web::Json<Task>) -> impl Responder {
    let id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    let update = doc! { "$set": { "name": &task.name } };

    match db.update_one(doc! { "_id": id }, update, None).await {
        Ok(_) => HttpResponse::Ok().body("Task updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update task"),
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(db: Db, path: web::Path<String>) -> impl Responder {
    let id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    match db.delete_one(doc! { "_id": id }, None).await {
        Ok(_) => HttpResponse::Ok().body("Task deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete task"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("todo_db");
    let collection = db.collection::<Task>("tasks");

    // Optional dummy data (only if collection is empty)
    let count = collection.count_documents(None, None).await.unwrap();
    if count == 0 {
        collection.insert_many(vec![
            Task { id: None, name: "Learn Rust".into() },
            Task { id: None, name: "Practice Vue".into() },
        ], None).await.unwrap();
    }

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(collection.clone()))
            .service(get_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
