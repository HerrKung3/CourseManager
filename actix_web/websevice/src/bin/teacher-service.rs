use actix_web::{web, App, HttpServer, http};
use std::io;
use std::sync::Mutex;
use routers::*;
use state::AppState;
use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use crate::errors::MyError;
use actix_cors::Cors;

#[path = "../state.rs"]
mod state;
#[path = "../routers.rs"]
mod routers;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path="../errors.rs"]
mod errors;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    //read env var
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE Not found in .env");

    //connect to database
    let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

    //init a app state
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    //instance a app and register routes
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
            .wrap(cors)
    };
    //run a http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}