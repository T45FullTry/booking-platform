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
                    .service(
                        web::scope("/organizations")
                            .route("/types", web::get().to(handlers::get_organization_types))
                            .route("", web::post().to(handlers::create_organization))
                            .route("", web::get().to(handlers::get_organizations))
                            .route("/{id}", web::get().to(handlers::get_organization))
                            .route("/{id}", web::put().to(handlers::update_organization))
                            .route("/{id}", web::delete().to(handlers::delete_organization))
                    )
                    .service(
                        web::scope("/patients")
                            .route("/{id}/employments", web::post().to(handlers::create_patient_employment))
                            .route("/{id}/employments", web::get().to(handlers::get_patient_employments))
                    )
                    .service(
                        web::scope("/clinicians")
                            .route("/{id}/affiliations", web::post().to(handlers::create_clinician_affiliation))
                            .route("/{id}/affiliations", web::get().to(handlers::get_clinician_affiliations))
                    )
                    .service(
                        web::scope("/bookings")
                            .route("/{id}/insurance", web::post().to(handlers::create_booking_insurance))
                            .route("/{id}/insurance", web::get().to(handlers::get_booking_insurance))
                    )
                    .service(
                        web::scope("/documents")
                            .route("/{id}/issuers", web::post().to(handlers::create_document_issuer))
                            .route("/{id}/issuers", web::get().to(handlers::get_document_issuers))
                    )
                    .service(
                        web::scope("/services")
                            .route("/{id}/rules", web::post().to(handlers::create_service_rule))
                            .route("/{id}/rules", web::get().to(handlers::get_service_rules))
                            .route("/rules/{id}", web::put().to(handlers::update_service_rule))
                            .route("/rules/{id}", web::delete().to(handlers::delete_service_rule))
                            .route("/{id}/eligibility", web::post().to(handlers::check_service_eligibility))
                            .route("/{patient_id}/available", web::get().to(handlers::get_available_services))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}