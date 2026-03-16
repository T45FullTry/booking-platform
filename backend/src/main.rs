use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::get_pool(&database_url);
    
    println!("Starting booking platform backend server...");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/patients")
                            .route("", web::post().to(handlers::create_patient))
                    )
                    .service(
                        web::scope("/bookings")
                            .route("", web::post().to(handlers::create_booking))
                            .route("/{id}", web::get().to(handlers::get_booking))
                            .route("/cancel", web::post().to(handlers::cancel_booking))
                    )
                    .service(
                        web::scope("/availability")
                            .route("", web::get().to(handlers::get_availability))
                    )
                    .service(
                        web::scope("/clinicians")
                            .route("/search", web::get().to(handlers::search_clinicians))
                            .route("/search-db", web::get().to(handlers::search_clinicians_db))
                            .route("/{id}", web::get().to(handlers::get_clinician))
                    )
                    .service(
                        web::scope("/documents")
                            .route("", web::post().to(handlers::create_document))
                            .route("", web::get().to(handlers::get_documents_by_category))
                            .route("/{id}", web::get().to(handlers::get_document))
                            .route("/{id}", web::put().to(handlers::update_document))
                            .route("/{id}", web::delete().to(handlers::delete_document))
                            .route("/{id}/stream", web::get().to(handlers::stream_document))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}