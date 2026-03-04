use actix_web::{web, HttpResponse, Result, Query};
use sqlx::PgPool;

use crate::models::{SearchQuery, SearchResponse, ClinicianDetails, ClinicianResponse};

pub async fn search_clinicians(
    pool: web::Data<PgPool>,
    query: Query<SearchQuery>,
) -> Result<HttpResponse> {
    match crate::db::search_clinicians(&pool, &query.into_inner()).await {
        Ok(results) => {
            let response = SearchResponse {
                results,
                total_count: 0, // In a real implementation, this would be calculated
                page: 1,
                has_more: false,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_clinician_details(
    pool: web::Data<PgPool>,
    path: web::Path<uuid::Uuid>,
) -> Result<HttpResponse> {
    let clinician_id = path.into_inner();
    
    match crate::db::get_clinician_details(&pool, clinician_id).await {
        Ok(Some(details)) => Ok(HttpResponse::Ok().json(ClinicianResponse { clinician: details })),
        Ok(None) => Ok(HttpResponse::NotFound().json("Clinician not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}