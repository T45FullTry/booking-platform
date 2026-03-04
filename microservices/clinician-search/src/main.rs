use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::get_pool(&database_url);
    
    println!("Starting clinician search microservice on port 8081...");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(handlers::search_clinicians)
            .service(handlers::get_clinician_details)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}