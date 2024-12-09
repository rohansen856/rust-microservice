use actix_web::{
    get, post,
    web::{Data, Json},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Serialize, FromRow)]
struct Todo {
    id: i32,
    title: String,
    description: Option<String>,       // Description can be NULL in the schema
    status: String,                    // Representing 'pending', 'completed', etc.
    due_date: Option<NaiveDate>,       // Date can be NULL
    created_at: NaiveDateTime,         // Timestamp for creation
    updated_at: NaiveDateTime,         // Timestamp for last update
}

#[derive(Deserialize)]
pub struct CreateTodoBody {
    pub title: String,
    pub description: Option<String>,   // Accepting description field for task creation
    pub status: Option<String>,        // Allow users to provide a status (optional)
    pub due_date: Option<NaiveDate>,   // Due date can be optional
}

#[get("/")]
pub async fn test_server() -> impl Responder {
    HttpResponse::Ok().json("server 1 is running")
}

#[get("/todos")]
pub async fn fetch_todos(state: Data<AppState>) -> impl Responder {
    let kafka_producer = &state.kafka_producer;
    kafka_producer.produce("test-topic", "fetch").await;

    match sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, status, due_date, created_at, updated_at FROM todo"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::NotFound().json("No todos found"),
    }
}

#[post("/todo")]
pub async fn create_todo(state: Data<AppState>, body: Json<CreateTodoBody>) -> impl Responder {
    let kafka_producer = &state.kafka_producer;
    kafka_producer.produce("test-topic", "fetch").await;

    let status = body.status.clone().unwrap_or_else(|| "pending".to_string());

    match sqlx::query_as::<_, Todo>(
        "INSERT INTO todo (title, description, status, due_date) 
        VALUES ($1, $2, $3, $4) 
        RETURNING id, title, description, status, due_date, created_at, updated_at"
    )
    .bind(&body.title)
    .bind(&body.description)
    .bind(&status)
    .bind(&body.due_date)
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json("Failed to create Todo")},
    }
}
