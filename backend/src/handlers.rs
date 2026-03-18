use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;
use reqwest;
use serde_json::Value;

use crate::models::{
    CreateBookingRequest, BookingResponse, AvailabilityRequest, CancelBookingRequest,
    CreatePatientRequest, PatientResponse, ClinicianSearchRequest, ClinicianSearchResponse,
    CreateDocumentRequest, UpdateDocumentRequest, DocumentResponse, DocumentListResponse,
    DocumentCategoryFilter,
};
use crate::db;

pub async fn create_patient(
    pool: web::Data<PgPool>,
    patient_data: web::Json<CreatePatientRequest>,
) -> Result<HttpResponse> {
    // Convert request to patient model
    let patient = crate::db::Patient {
        id: Uuid::new_v4(),
        first_name: patient_data.first_name.clone(),
        last_name: patient_data.last_name.clone(),
        date_of_birth: patient_data.date_of_birth,
        gender: patient_data.gender.clone(),
        phone: patient_data.phone.clone(),
        email: patient_data.email.clone(),
        address: patient_data.address.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    match crate::db::create_patient(&pool, &patient).await {
        Ok(patient_id) => {
            let response = PatientResponse {
                id: patient_id,
                first_name: patient.first_name,
                last_name: patient.last_name,
                date_of_birth: patient.date_of_birth,
                gender: patient.gender,
                phone: patient.phone,
                email: patient.email,
                address: patient.address,
                created_at: patient.created_at.to_rfc3339(),
                updated_at: patient.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn create_booking(
    pool: web::Data<PgPool>,
    booking_data: web::Json<CreateBookingRequest>,
) -> Result<HttpResponse> {
    // Convert request to booking model
    let booking = crate::db::Booking {
        id: Uuid::new_v4(),
        patient_id: booking_data.patient_id,
        booker_id: booking_data.booker_id,
        clinician_id: booking_data.clinician_id,
        service_id: booking_data.service_id,
        booking_date: booking_data.booking_date,
        booking_time: booking_data.booking_time,
        duration_minutes: booking_data.duration_minutes,
        status: "confirmed".to_string(),
        symptoms_reported: booking_data.symptoms_reported.clone(),
        consultation_reason: booking_data.consultation_reason.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    match crate::db::create_booking(&pool, &booking).await {
        Ok(booking_result) => {
            let response = BookingResponse {
                id: booking_result.id,
                patient_id: booking_result.patient_id,
                booker_id: booking_result.booker_id,
                clinician_id: booking_result.clinician_id,
                service_id: booking_result.service_id,
                booking_date: booking_result.booking_date,
                booking_time: booking_result.booking_time,
                duration_minutes: booking_result.duration_minutes,
                status: booking_result.status,
                symptoms_reported: booking_result.symptoms_reported,
                consultation_reason: booking_result.consultation_reason,
                created_at: booking_result.created_at.to_rfc3339(),
                updated_at: booking_result.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_booking(
    pool: web::Data<PgPool>,
    booking_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_booking(&pool, *booking_id).await {
        Ok(Some(booking)) => {
            let response = BookingResponse {
                id: booking.id,
                patient_id: booking.patient_id,
                booker_id: booking.booker_id,
                clinician_id: booking.clinician_id,
                service_id: booking.service_id,
                booking_date: booking.booking_date,
                booking_time: booking.booking_time,
                duration_minutes: booking.duration_minutes,
                status: booking.status,
                symptoms_reported: booking.symptoms_reported,
                consultation_reason: booking.consultation_reason,
                created_at: booking.created_at.to_rfc3339(),
                updated_at: booking.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Ok(None) => Ok(HttpResponse::NotFound().json("Booking not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn cancel_booking(
    pool: web::Data<PgPool>,
    booking_data: web::Json<CancelBookingRequest>,
) -> Result<HttpResponse> {
    match crate::db::cancel_booking(&pool, booking_data.booking_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json("Booking cancelled successfully")),
        Ok(false) => Ok(HttpResponse::NotFound().json("Booking not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_availability(
    pool: web::Data<PgPool>,
    query: web::Query<AvailabilityRequest>,
) -> Result<HttpResponse> {
    match crate::db::get_available_slots(&pool, query.clinician_id, query.date).await {
        Ok(slots) => {
            let time_slots: Vec<crate::models::TimeSlot> = slots.into_iter().map(|slot| {
                crate::models::TimeSlot {
                    id: slot.id,
                    start_time: slot.start_time,
                    end_time: slot.end_time,
                    available: slot.status == "available",
                }
            }).collect();
            
            let response = crate::models::AvailabilityResponse {
                date: query.date,
                slots: time_slots,
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Endpoint to search clinicians via database (fallback if microservice is down)
pub async fn search_clinicians_db(
    pool: web::Data<PgPool>,
    query: web::Query<ClinicianSearchRequest>,
) -> Result<HttpResponse> {
    let symptom = query.symptom.as_deref().unwrap_or("");
    
    match crate::db::search_clinicians_by_symptom(&pool, symptom).await {
        Ok(clinicians) => {
            let results: Vec<crate::models::ClinicianResult> = clinicians.into_iter().map(|clinician| {
                crate::models::ClinicianResult {
                    id: clinician.id,
                    name: format!("{} {}", clinician.first_name, clinician.last_name),
                    specialty: clinician.specialty,
                    rating: clinician.rating,
                    available_times: vec!["9:00 AM".to_string(), "11:30 AM".to_string(), "2:00 PM".to_string()], // Mock data
                    location: "Main Clinic".to_string(),
                }
            }).collect();
            
            let response = ClinicianSearchResponse {
                results,
                total_count: 0, // In a real implementation, this would be calculated
                page: 1,
                has_more: false,
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Proxy endpoint to search clinicians via microservice
pub async fn search_clinicians(
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let client = reqwest::Client::new();
    
    // Build query parameters
    let mut params = vec![];
    if let Some(symptom) = query.get("symptom") {
        params.push(("symptom", symptom.as_str()));
    }
    if let Some(condition) = query.get("condition") {
        params.push(("condition", condition.as_str()));
    }
    if let Some(specialty) = query.get("specialty") {
        params.push(("specialty", specialty.as_str()));
    }
    
    match client
        .get("http://localhost:8081/api/search-clinicians")
        .query(&params)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(json) => Ok(HttpResponse::Ok().json(json)),
                Err(_) => {
                    // Fallback to database search if microservice fails
                    Ok(HttpResponse::InternalServerError().json("Microservice unavailable, try direct DB search"))
                }
            }
        }
        Err(_) => {
            // Fallback to database search if microservice fails
            Ok(HttpResponse::InternalServerError().json("Microservice unavailable, try direct DB search"))
        }
    }
}

// Proxy endpoint to get clinician details via microservice
pub async fn get_clinician(
    clinician_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8081/api/clinicians/{}", clinician_id);
    
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(json) => Ok(HttpResponse::Ok().json(json)),
                Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to parse response from search service")),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to connect to search service")),
    }
}

// Document handlers
pub async fn create_document(
    pool: web::Data<PgPool>,
    doc_data: web::Json<CreateDocumentRequest>,
) -> Result<HttpResponse> {
    // Decode base64 content if provided
    let content = if let Some(base64_str) = &doc_data.content_base64 {
        match base64::decode(base64_str) {
            Ok(bytes) => Some(bytes),
            Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid base64 content")),
        }
    } else {
        None
    };

    let file_size = content.as_ref().map(|c| c.len() as i64);

    let doc = crate::db::Document {
        id: Uuid::new_v4(),
        patient_id: doc_data.patient_id,
        clinician_id: doc_data.clinician_id,
        booking_id: doc_data.booking_id,
        consultation_id: doc_data.consultation_id,
        category: doc_data.category.clone(),
        document_type: doc_data.document_type.clone(),
        title: doc_data.title.clone(),
        description: doc_data.description.clone(),
        file_name: doc_data.file_name.clone(),
        mime_type: doc_data.mime_type.clone(),
        content,
        content_text: doc_data.content_text.clone(),
        file_size_bytes: file_size,
        page_count: None,
        status: "active".to_string(),
        is_patient_visible: doc_data.is_patient_visible.unwrap_or(true),
        metadata: doc_data.metadata.clone(),
        created_by: doc_data.clinician_id,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    match crate::db::create_document(&pool, &doc).await {
        Ok(doc_id) => {
            let response = DocumentResponse {
                id: doc_id,
                patient_id: doc.patient_id,
                clinician_id: doc.clinician_id,
                booking_id: doc.booking_id,
                consultation_id: doc.consultation_id,
                category: doc.category,
                document_type: doc.document_type,
                title: doc.title,
                description: doc.description,
                file_name: doc.file_name,
                mime_type: doc.mime_type,
                file_size_bytes: doc.file_size_bytes,
                page_count: doc.page_count,
                status: doc.status,
                is_patient_visible: doc.is_patient_visible,
                metadata: doc.metadata,
                created_by: doc.created_by,
                created_at: doc.created_at.to_rfc3339(),
                updated_at: doc.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_document(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_document(&pool, *document_id).await {
        Ok(Some(doc)) => {
            let response = DocumentResponse {
                id: doc.id,
                patient_id: doc.patient_id,
                clinician_id: doc.clinician_id,
                booking_id: doc.booking_id,
                consultation_id: doc.consultation_id,
                category: doc.category,
                document_type: doc.document_type,
                title: doc.title,
                description: doc.description,
                file_name: doc.file_name,
                mime_type: doc.mime_type,
                file_size_bytes: doc.file_size_bytes,
                page_count: doc.page_count,
                status: doc.status,
                is_patient_visible: doc.is_patient_visible,
                metadata: doc.metadata,
                created_by: doc.created_by,
                created_at: doc.created_at.to_rfc3339(),
                updated_at: doc.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_documents_by_category(
    pool: web::Data<PgPool>,
    filter: web::Query<DocumentCategoryFilter>,
) -> Result<HttpResponse> {
    let page = filter.page.unwrap_or(0);
    let limit = filter.limit.unwrap_or(20);

    match crate::db::get_documents_by_patient(
        &pool,
        filter.patient_id.unwrap_or_else(Uuid::nil),
        filter.category.as_deref(),
        filter.status.as_deref(),
        page,
        limit,
    ).await {
        Ok(docs) => {
            let responses: Vec<DocumentResponse> = docs.into_iter().map(|doc| {
                DocumentResponse {
                    id: doc.id,
                    patient_id: doc.patient_id,
                    clinician_id: doc.clinician_id,
                    booking_id: doc.booking_id,
                    consultation_id: doc.consultation_id,
                    category: doc.category,
                    document_type: doc.document_type,
                    title: doc.title,
                    description: doc.description,
                    file_name: doc.file_name,
                    mime_type: doc.mime_type,
                    file_size_bytes: doc.file_size_bytes,
                    page_count: doc.page_count,
                    status: doc.status,
                    is_patient_visible: doc.is_patient_visible,
                    metadata: doc.metadata,
                    created_by: doc.created_by,
                    created_at: doc.created_at.to_rfc3339(),
                    updated_at: doc.updated_at.to_rfc3339(),
                }
            }).collect();

            let response = DocumentListResponse {
                documents: responses,
                total_count: responses.len(),
                page,
                has_more: responses.len() == limit,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn update_document(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
    update_data: web::Json<UpdateDocumentRequest>,
) -> Result<HttpResponse> {
    match crate::db::update_document(
        &pool,
        *document_id,
        update_data.title.as_deref(),
        update_data.description.as_deref(),
        update_data.category.as_deref(),
        update_data.status.as_deref(),
        update_data.is_patient_visible,
        update_data.metadata.as_ref(),
    ).await {
        Ok(true) => Ok(HttpResponse::Ok().json("Document updated successfully")),
        Ok(false) => Ok(HttpResponse::NotFound().json("Document not found or no changes made")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn delete_document(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::delete_document(&pool, *document_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json("Document deleted successfully")),
        Ok(false) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn stream_document(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::stream_document_content(&pool, *document_id).await {
        Ok(Some(content)) => Ok(HttpResponse::Ok().content_type("application/octet-stream").body(content)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Document content not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}