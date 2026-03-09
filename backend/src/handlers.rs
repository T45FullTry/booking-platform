use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;
use reqwest;
use serde_json::Value;

use crate::models::{
    CreateBookingRequest, BookingResponse, AvailabilityRequest, CancelBookingRequest,
    CreatePatientRequest, PatientResponse, ClinicianSearchRequest, ClinicianSearchResponse,
    SymptomFilter, ServiceWithSymptoms
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
    // First, get patient information to check age/gender requirements
    let patient = match crate::db::get_patient(&pool, booking_data.patient_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(HttpResponse::BadRequest().json("Patient not found")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("Error fetching patient: {}", e))),
    };
    
    // Calculate patient age
    let today = chrono::Utc::now().date_naive();
    let patient_age = today.signed_duration_since(patient.date_of_birth).num_days() / 365;
    
    // Try to reserve the availability slot
    match crate::db::reserve_availability_slot(&pool, booking_data.availability_slot_id, booking_data.patient_id).await {
        Ok(true) => {
            // Slot successfully reserved, now create the booking
            let booking = crate::db::Booking {
                id: Uuid::new_v4(),
                patient_id: booking_data.patient_id,
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
                Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error creating booking: {}", e))),
            }
        },
        Ok(false) => Ok(HttpResponse::Conflict().json("Slot is not available or doesn't meet patient requirements")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error reserving slot: {}", e))),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailabilitySearchRequest {
    pub clinician_id: Uuid,
    pub date: NaiveDate,
    pub patient_id: Uuid, // Needed to check age/gender restrictions
}

pub async fn get_availability(
    pool: web::Data<PgPool>,
    query: web::Query<AvailabilitySearchRequest>,
) -> Result<HttpResponse> {
    // Get patient information to check age/gender requirements
    let patient = match crate::db::get_patient(&pool, query.patient_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(HttpResponse::BadRequest().json("Patient not found")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("Error fetching patient: {}", e))),
    };
    
    // Calculate patient age
    let today = chrono::Utc::now().date_naive();
    let patient_age = (today.signed_duration_since(patient.date_of_birth).num_days() / 365) as i32;
    let patient_gender = patient.gender.clone();
    
    match crate::db::get_available_slots_for_patient(
        &pool, 
        query.clinician_id, 
        query.date, 
        patient_age, 
        patient_gender
    ).await {
        Ok(slots) => {
            let time_slots: Vec<crate::models::TimeSlot> = slots.into_iter().map(|slot| {
                crate::models::TimeSlot {
                    id: slot.id,
                    start_time: slot.start_time,
                    end_time: slot.end_time,
                    available: slot.status == "available" || slot.status == "reserved",
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

// Search clinicians by multiple symptoms
pub async fn search_clinicians_by_symptoms(
    pool: web::Data<PgPool>,
    symptom_filter: web::Json<SymptomFilter>,
) -> Result<HttpResponse> {
    match crate::db::search_clinicians_by_symptoms(&pool, &symptom_filter.symptoms).await {
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
                total_count: results.len(),
                page: 1,
                has_more: false,
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Get services filtered by symptoms
pub async fn get_services_by_symptoms(
    pool: web::Data<PgPool>,
    symptom_filter: web::Json<SymptomFilter>,
) -> Result<HttpResponse> {
    match crate::db::get_services_for_symptoms(&pool, &symptom_filter.symptoms).await {
        Ok(services) => Ok(HttpResponse::Ok().json(services)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Get all symptoms (for autocomplete/filtering UI)
pub async fn get_all_symptoms(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    };
    
    match client.query("SELECT id, name, description, body_system, severity_level FROM symptoms ORDER BY name", &[]).await {
        Ok(rows) => {
            let symptoms: Vec<Value> = rows.into_iter().map(|row| {
                serde_json::json!({
                    "id": row.get::<_, Uuid>("id"),
                    "name": row.get::<_, String>("name"),
                    "description": row.get::<_, Option<String>>("description"),
                    "body_system": row.get::<_, Option<String>>("body_system"),
                    "severity_level": row.get::<_, Option<i32>>("severity_level"),
                })
            }).collect();
            Ok(HttpResponse::Ok().json(symptoms))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Get clinician's symptoms (what they treat)
pub async fn get_clinician_symptoms_handler(
    pool: web::Data<PgPool>,
    clinician_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_clinician_symptoms(&pool, *clinician_id).await {
        Ok(symptoms) => {
            let response = serde_json::json!({
                "clinician_id": clinician_id,
                "symptoms": symptoms
            });
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}