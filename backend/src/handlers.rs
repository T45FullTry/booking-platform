use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;
use reqwest;
use serde_json::Value;

use crate::models::{
    CreateBookingRequest, BookingResponse, AvailabilityRequest, CancelBookingRequest,
    CreatePatientRequest, PatientResponse, ClinicianSearchRequest, ClinicianSearchResponse,
    CreateDocumentRequest, UpdateDocumentRequest, DocumentResponse, DocumentListResponse,
    DocumentCategoryFilter, CreateOrganizationTypeRequest, CreateOrganizationRequest,
    UpdateOrganizationRequest, OrganizationResponse, OrganizationListResponse, OrganizationFilter,
    CreatePatientEmploymentRequest, PatientEmploymentResponse, CreateClinicianAffiliationRequest,
    ClinicianAffiliationResponse, CreateBookingInsuranceRequest, BookingInsuranceResponse,
    CreateDocumentIssuerRequest, DocumentIssuerResponse, CreateServiceRuleRequest,
    UpdateServiceRuleRequest, ServiceRuleResponse, ServiceRuleListResponse,
    ServiceEligibilityRequest, ServiceEligibilityResponse, AvailableServicesListResponse,
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

// Organization handlers
pub async fn get_organization_types(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match crate::db::get_organization_types(&pool).await {
        Ok(types) => {
            let responses: Vec<crate::models::OrganizationTypeResponse> = types.into_iter().map(|t| {
                crate::models::OrganizationTypeResponse {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.to_rfc3339(),
                }
            }).collect();
            Ok(HttpResponse::Ok().json(responses))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn create_organization(
    pool: web::Data<PgPool>,
    org_data: web::Json<CreateOrganizationRequest>,
) -> Result<HttpResponse> {
    let org = crate::db::Organization {
        id: Uuid::new_v4(),
        name: org_data.name.clone(),
        organization_type_id: org_data.organization_type_id,
        organization_type_name: String::new(), // Will be populated on fetch
        registration_number: org_data.registration_number.clone(),
        tax_id: org_data.tax_id.clone(),
        website: org_data.website.clone(),
        email: org_data.email.clone(),
        phone: org_data.phone.clone(),
        fax: org_data.fax.clone(),
        address: org_data.address.clone(),
        city: org_data.city.clone(),
        state_province: org_data.state_province.clone(),
        postal_code: org_data.postal_code.clone(),
        country: org_data.country.clone(),
        contact_person_name: org_data.contact_person_name.clone(),
        contact_person_email: org_data.contact_person_email.clone(),
        contact_person_phone: org_data.contact_person_phone.clone(),
        notes: org_data.notes.clone(),
        status: "active".to_string(),
        metadata: org_data.metadata.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    match crate::db::create_organization(&pool, &org).await {
        Ok(org_id) => {
            let created_org = crate::db::get_organization(&pool, org_id).await.ok().flatten();
            match created_org {
                Some(o) => {
                    let response = OrganizationResponse {
                        id: o.id,
                        name: o.name,
                        organization_type_id: o.organization_type_id,
                        organization_type_name: o.organization_type_name,
                        registration_number: o.registration_number,
                        tax_id: o.tax_id,
                        website: o.website,
                        email: o.email,
                        phone: o.phone,
                        fax: o.fax,
                        address: o.address,
                        city: o.city,
                        state_province: o.state_province,
                        postal_code: o.postal_code,
                        country: o.country,
                        contact_person_name: o.contact_person_name,
                        contact_person_email: o.contact_person_email,
                        contact_person_phone: o.contact_person_phone,
                        notes: o.notes,
                        status: o.status,
                        metadata: o.metadata,
                        created_at: o.created_at.to_rfc3339(),
                        updated_at: o.updated_at.to_rfc3339(),
                    };
                    Ok(HttpResponse::Created().json(response))
                }
                None => Ok(HttpResponse::InternalServerError().json("Failed to fetch created organization")),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_organization(
    pool: web::Data<PgPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_organization(&pool, *organization_id).await {
        Ok(Some(org)) => {
            let response = OrganizationResponse {
                id: org.id,
                name: org.name,
                organization_type_id: org.organization_type_id,
                organization_type_name: org.organization_type_name,
                registration_number: org.registration_number,
                tax_id: org.tax_id,
                website: org.website,
                email: org.email,
                phone: org.phone,
                fax: org.fax,
                address: org.address,
                city: org.city,
                state_province: org.state_province,
                postal_code: org.postal_code,
                country: org.country,
                contact_person_name: org.contact_person_name,
                contact_person_email: org.contact_person_email,
                contact_person_phone: org.contact_person_phone,
                notes: org.notes,
                status: org.status,
                metadata: org.metadata,
                created_at: org.created_at.to_rfc3339(),
                updated_at: org.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Organization not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_organizations(
    pool: web::Data<PgPool>,
    filter: web::Query<OrganizationFilter>,
) -> Result<HttpResponse> {
    let page = filter.page.unwrap_or(0);
    let limit = filter.limit.unwrap_or(20);

    match crate::db::get_organizations(
        &pool,
        filter.organization_type_name.as_deref(),
        filter.status.as_deref(),
        page,
        limit,
    ).await {
        Ok(orgs) => {
            let responses: Vec<OrganizationResponse> = orgs.into_iter().map(|org| {
                OrganizationResponse {
                    id: org.id,
                    name: org.name,
                    organization_type_id: org.organization_type_id,
                    organization_type_name: org.organization_type_name,
                    registration_number: org.registration_number,
                    tax_id: org.tax_id,
                    website: org.website,
                    email: org.email,
                    phone: org.phone,
                    fax: org.fax,
                    address: org.address,
                    city: org.city,
                    state_province: org.state_province,
                    postal_code: org.postal_code,
                    country: org.country,
                    contact_person_name: org.contact_person_name,
                    contact_person_email: org.contact_person_email,
                    contact_person_phone: org.contact_person_phone,
                    notes: org.notes,
                    status: org.status,
                    metadata: org.metadata,
                    created_at: org.created_at.to_rfc3339(),
                    updated_at: org.updated_at.to_rfc3339(),
                }
            }).collect();

            let response = OrganizationListResponse {
                organizations: responses,
                total_count: responses.len(),
                page,
                has_more: responses.len() == limit,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn update_organization(
    pool: web::Data<PgPool>,
    organization_id: web::Path<Uuid>,
    update_data: web::Json<UpdateOrganizationRequest>,
) -> Result<HttpResponse> {
    match crate::db::update_organization(
        &pool,
        *organization_id,
        update_data.name.as_deref(),
        update_data.organization_type_id,
        update_data.registration_number.as_deref(),
        update_data.tax_id.as_deref(),
        update_data.website.as_deref(),
        update_data.email.as_deref(),
        update_data.phone.as_deref(),
        update_data.address.as_deref(),
        update_data.city.as_deref(),
        update_data.state_province.as_deref(),
        update_data.postal_code.as_deref(),
        update_data.country.as_deref(),
        update_data.contact_person_name.as_deref(),
        update_data.contact_person_email.as_deref(),
        update_data.contact_person_phone.as_deref(),
        update_data.notes.as_deref(),
        update_data.status.as_deref(),
        update_data.metadata.as_ref(),
    ).await {
        Ok(true) => {
            let updated_org = crate::db::get_organization(&pool, *organization_id).await.ok().flatten();
            match updated_org {
                Some(o) => {
                    let response = OrganizationResponse {
                        id: o.id,
                        name: o.name,
                        organization_type_id: o.organization_type_id,
                        organization_type_name: o.organization_type_name,
                        registration_number: o.registration_number,
                        tax_id: o.tax_id,
                        website: o.website,
                        email: o.email,
                        phone: o.phone,
                        fax: o.fax,
                        address: o.address,
                        city: o.city,
                        state_province: o.state_province,
                        postal_code: o.postal_code,
                        country: o.country,
                        contact_person_name: o.contact_person_name,
                        contact_person_email: o.contact_person_email,
                        contact_person_phone: o.contact_person_phone,
                        notes: o.notes,
                        status: o.status,
                        metadata: o.metadata,
                        created_at: o.created_at.to_rfc3339(),
                        updated_at: o.updated_at.to_rfc3339(),
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
                None => Ok(HttpResponse::NotFound().json("Organization not found")),
            }
        }
        Ok(false) => Ok(HttpResponse::NotFound().json("Organization not found or no changes made")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn delete_organization(
    pool: web::Data<PgPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::delete_organization(&pool, *organization_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json("Organization deleted successfully")),
        Ok(false) => Ok(HttpResponse::NotFound().json("Organization not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Patient Employment handlers
pub async fn create_patient_employment(
    pool: web::Data<PgPool>,
    emp_data: web::Json<CreatePatientEmploymentRequest>,
) -> Result<HttpResponse> {
    match crate::db::create_patient_employment(
        &pool,
        emp_data.patient_id,
        emp_data.organization_id,
        emp_data.job_title.as_deref(),
        emp_data.department.as_deref(),
        emp_data.employee_id.as_deref(),
        emp_data.start_date,
        emp_data.notes.as_deref(),
    ).await {
        Ok(emp_id) => {
            let employments = crate::db::get_patient_employments(&pool, emp_data.patient_id).await.ok().unwrap_or_default();
            if let Some(emp) = employments.into_iter().find(|e| e.id == emp_id) {
                let response = PatientEmploymentResponse {
                    id: emp.id,
                    patient_id: emp.patient_id,
                    patient_name: emp.patient_name,
                    organization_id: emp.organization_id,
                    organization_name: emp.organization_name,
                    job_title: emp.job_title,
                    department: emp.department,
                    employee_id: emp.employee_id,
                    start_date: emp.start_date,
                    end_date: emp.end_date,
                    is_current: emp.is_current,
                    notes: emp.notes,
                    created_at: emp.created_at.to_rfc3339(),
                    updated_at: emp.updated_at.to_rfc3339(),
                };
                Ok(HttpResponse::Created().json(response))
            } else {
                Ok(HttpResponse::InternalServerError().json("Failed to fetch created employment"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_patient_employments(
    pool: web::Data<PgPool>,
    patient_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_patient_employments(&pool, *patient_id).await {
        Ok(employments) => {
            let responses: Vec<PatientEmploymentResponse> = employments.into_iter().map(|emp| {
                PatientEmploymentResponse {
                    id: emp.id,
                    patient_id: emp.patient_id,
                    patient_name: emp.patient_name,
                    organization_id: emp.organization_id,
                    organization_name: emp.organization_name,
                    job_title: emp.job_title,
                    department: emp.department,
                    employee_id: emp.employee_id,
                    start_date: emp.start_date,
                    end_date: emp.end_date,
                    is_current: emp.is_current,
                    notes: emp.notes,
                    created_at: emp.created_at.to_rfc3339(),
                    updated_at: emp.updated_at.to_rfc3339(),
                }
            }).collect();
            Ok(HttpResponse::Ok().json(responses))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Clinician Affiliation handlers
pub async fn create_clinician_affiliation(
    pool: web::Data<PgPool>,
    aff_data: web::Json<CreateClinicianAffiliationRequest>,
) -> Result<HttpResponse> {
    match crate::db::create_clinician_affiliation(
        &pool,
        aff_data.clinician_id,
        aff_data.organization_id,
        aff_data.affiliation_type.as_deref(),
        aff_data.department.as_deref(),
        aff_data.start_date,
        aff_data.is_primary,
        aff_data.notes.as_deref(),
    ).await {
        Ok(aff_id) => {
            let affiliations = crate::db::get_clinician_affiliations(&pool, aff_data.clinician_id).await.ok().unwrap_or_default();
            if let Some(aff) = affiliations.into_iter().find(|a| a.id == aff_id) {
                let response = ClinicianAffiliationResponse {
                    id: aff.id,
                    clinician_id: aff.clinician_id,
                    clinician_name: aff.clinician_name,
                    organization_id: aff.organization_id,
                    organization_name: aff.organization_name,
                    affiliation_type: aff.affiliation_type,
                    department: aff.department,
                    start_date: aff.start_date,
                    end_date: aff.end_date,
                    is_primary: aff.is_primary,
                    is_current: aff.is_current,
                    notes: aff.notes,
                    created_at: aff.created_at.to_rfc3339(),
                    updated_at: aff.updated_at.to_rfc3339(),
                };
                Ok(HttpResponse::Created().json(response))
            } else {
                Ok(HttpResponse::InternalServerError().json("Failed to fetch created affiliation"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_clinician_affiliations(
    pool: web::Data<PgPool>,
    clinician_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_clinician_affiliations(&pool, *clinician_id).await {
        Ok(affiliations) => {
            let responses: Vec<ClinicianAffiliationResponse> = affiliations.into_iter().map(|aff| {
                ClinicianAffiliationResponse {
                    id: aff.id,
                    clinician_id: aff.clinician_id,
                    clinician_name: aff.clinician_name,
                    organization_id: aff.organization_id,
                    organization_name: aff.organization_name,
                    affiliation_type: aff.affiliation_type,
                    department: aff.department,
                    start_date: aff.start_date,
                    end_date: aff.end_date,
                    is_primary: aff.is_primary,
                    is_current: aff.is_current,
                    notes: aff.notes,
                    created_at: aff.created_at.to_rfc3339(),
                    updated_at: aff.updated_at.to_rfc3339(),
                }
            }).collect();
            Ok(HttpResponse::Ok().json(responses))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Booking Insurance handlers
pub async fn create_booking_insurance(
    pool: web::Data<PgPool>,
    ins_data: web::Json<CreateBookingInsuranceRequest>,
) -> Result<HttpResponse> {
    match crate::db::create_booking_insurance(
        &pool,
        ins_data.booking_id,
        ins_data.organization_id,
        ins_data.policy_number.as_deref(),
        ins_data.group_number.as_deref(),
        ins_data.member_id.as_deref(),
        ins_data.coverage_type.as_deref(),
        ins_data.authorization_required,
        ins_data.notes.as_deref(),
    ).await {
        Ok(ins_id) => {
            let insurance = crate::db::get_booking_insurance(&pool, ins_data.booking_id).await.ok().flatten();
            match insurance {
                Some(i) => {
                    let response = BookingInsuranceResponse {
                        id: i.id,
                        booking_id: i.booking_id,
                        organization_id: i.organization_id,
                        organization_name: i.organization_name,
                        policy_number: i.policy_number,
                        group_number: i.group_number,
                        member_id: i.member_id,
                        coverage_type: i.coverage_type,
                        authorization_required: i.authorization_required,
                        authorization_number: i.authorization_number,
                        claim_status: i.claim_status,
                        claim_amount: i.claim_amount,
                        patient_responsibility: i.patient_responsibility,
                        notes: i.notes,
                        created_at: i.created_at.to_rfc3339(),
                        updated_at: i.updated_at.to_rfc3339(),
                    };
                    Ok(HttpResponse::Created().json(response))
                }
                None => Ok(HttpResponse::InternalServerError().json("Failed to fetch created insurance")),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_booking_insurance(
    pool: web::Data<PgPool>,
    booking_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_booking_insurance(&pool, *booking_id).await {
        Ok(Some(insurance)) => {
            let response = BookingInsuranceResponse {
                id: insurance.id,
                booking_id: insurance.booking_id,
                organization_id: insurance.organization_id,
                organization_name: insurance.organization_name,
                policy_number: insurance.policy_number,
                group_number: insurance.group_number,
                member_id: insurance.member_id,
                coverage_type: insurance.coverage_type,
                authorization_required: insurance.authorization_required,
                authorization_number: insurance.authorization_number,
                claim_status: insurance.claim_status,
                claim_amount: insurance.claim_amount,
                patient_responsibility: insurance.patient_responsibility,
                notes: insurance.notes,
                created_at: insurance.created_at.to_rfc3339(),
                updated_at: insurance.updated_at.to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Insurance info not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Document Issuer handlers
pub async fn create_document_issuer(
    pool: web::Data<PgPool>,
    issuer_data: web::Json<CreateDocumentIssuerRequest>,
) -> Result<HttpResponse> {
    match crate::db::create_document_issuer(
        &pool,
        issuer_data.document_id,
        issuer_data.organization_id,
        issuer_data.issuer_name.as_deref(),
        issuer_data.issue_date,
        issuer_data.reference_number.as_deref(),
        issuer_data.notes.as_deref(),
    ).await {
        Ok(issuer_id) => {
            let issuers = crate::db::get_document_issuers(&pool, issuer_data.document_id).await.ok().unwrap_or_default();
            if let Some(issuer) = issuers.into_iter().find(|i| i.id == issuer_id) {
                let response = DocumentIssuerResponse {
                    id: issuer.id,
                    document_id: issuer.document_id,
                    organization_id: issuer.organization_id,
                    organization_name: issuer.organization_name,
                    issuer_name: issuer.issuer_name,
                    issue_date: issuer.issue_date,
                    reference_number: issuer.reference_number,
                    notes: issuer.notes,
                    created_at: issuer.created_at.to_rfc3339(),
                };
                Ok(HttpResponse::Created().json(response))
            } else {
                Ok(HttpResponse::InternalServerError().json("Failed to fetch created issuer"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_document_issuers(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_document_issuers(&pool, *document_id).await {
        Ok(issuers) => {
            let responses: Vec<DocumentIssuerResponse> = issuers.into_iter().map(|issuer| {
                DocumentIssuerResponse {
                    id: issuer.id,
                    document_id: issuer.document_id,
                    organization_id: issuer.organization_id,
                    organization_name: issuer.organization_name,
                    issuer_name: issuer.issuer_name,
                    issue_date: issuer.issue_date,
                    reference_number: issuer.reference_number,
                    notes: issuer.notes,
                    created_at: issuer.created_at.to_rfc3339(),
                }
            }).collect();
            Ok(HttpResponse::Ok().json(responses))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

// Service Rule handlers
pub async fn create_service_rule(
    pool: web::Data<PgPool>,
    rule_data: web::Json<CreateServiceRuleRequest>,
) -> Result<HttpResponse> {
    match crate::db::create_service_rule(
        &pool,
        rule_data.service_id,
        &rule_data.rule_type,
        rule_data.rule_value.as_deref(),
        rule_data.rule_value_numeric,
        rule_data.description.as_deref(),
    ).await {
        Ok(rule_id) => {
            let rules = crate::db::get_service_rules(&pool, rule_data.service_id).await.ok().unwrap_or_default();
            if let Some(rule) = rules.into_iter().find(|r| r.id == rule_id) {
                let response = ServiceRuleResponse {
                    id: rule.id,
                    service_id: rule.service_id,
                    service_name: rule.service_name,
                    rule_type: rule.rule_type,
                    rule_value: rule.rule_value,
                    rule_value_numeric: rule.rule_value_numeric,
                    description: rule.description,
                    is_active: rule.is_active,
                    created_at: rule.created_at.to_rfc3339(),
                    updated_at: rule.updated_at.to_rfc3339(),
                };
                Ok(HttpResponse::Created().json(response))
            } else {
                Ok(HttpResponse::InternalServerError().json("Failed to fetch created rule"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_service_rules(
    pool: web::Data<PgPool>,
    service_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_service_rules(&pool, *service_id).await {
        Ok(rules) => {
            let responses: Vec<ServiceRuleResponse> = rules.into_iter().map(|rule| {
                ServiceRuleResponse {
                    id: rule.id,
                    service_id: rule.service_id,
                    service_name: rule.service_name,
                    rule_type: rule.rule_type,
                    rule_value: rule.rule_value,
                    rule_value_numeric: rule.rule_value_numeric,
                    description: rule.description,
                    is_active: rule.is_active,
                    created_at: rule.created_at.to_rfc3339(),
                    updated_at: rule.updated_at.to_rfc3339(),
                }
            }).collect();

            let response = ServiceRuleListResponse {
                rules: responses,
                total_count: responses.len(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn update_service_rule(
    pool: web::Data<PgPool>,
    rule_id: web::Path<Uuid>,
    update_data: web::Json<UpdateServiceRuleRequest>,
) -> Result<HttpResponse> {
    match crate::db::update_service_rule(
        &pool,
        *rule_id,
        update_data.rule_type.as_deref(),
        update_data.rule_value.as_deref(),
        update_data.rule_value_numeric,
        update_data.description.as_deref(),
        update_data.is_active,
    ).await {
        Ok(true) => {
            // Get the rule to return updated data
            let rules = crate::db::get_all_active_service_rules(&pool).await.ok().unwrap_or_default();
            if let Some(rule) = rules.into_iter().find(|r| r.id == *rule_id) {
                let response = ServiceRuleResponse {
                    id: rule.id,
                    service_id: rule.service_id,
                    service_name: rule.service_name,
                    rule_type: rule.rule_type,
                    rule_value: rule.rule_value,
                    rule_value_numeric: rule.rule_value_numeric,
                    description: rule.description,
                    is_active: rule.is_active,
                    created_at: rule.created_at.to_rfc3339(),
                    updated_at: rule.updated_at.to_rfc3339(),
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::NotFound().json("Rule not found"))
            }
        }
        Ok(false) => Ok(HttpResponse::NotFound().json("Rule not found or no changes made")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn delete_service_rule(
    pool: web::Data<PgPool>,
    rule_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::delete_service_rule(&pool, *rule_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json("Service rule deleted successfully")),
        Ok(false) => Ok(HttpResponse::NotFound().json("Service rule not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn check_service_eligibility(
    pool: web::Data<PgPool>,
    eligibility_data: web::Json<ServiceEligibilityRequest>,
) -> Result<HttpResponse> {
    match crate::db::check_service_eligibility(&pool, eligibility_data.patient_id, eligibility_data.service_id).await {
        Ok((eligible, reason, failed_rules)) => {
            // Get service name
            let service_name = match crate::db::get_service(&pool, eligibility_data.service_id).await {
                Ok(Some(service)) => service.name,
                _ => "Unknown".to_string(),
            };

            let response = ServiceEligibilityResponse {
                service_id: eligibility_data.service_id,
                service_name,
                patient_id: eligibility_data.patient_id,
                eligible,
                reason,
                failed_rules,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

pub async fn get_available_services(
    pool: web::Data<PgPool>,
    patient_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match crate::db::get_available_services_for_patient(&pool, *patient_id).await {
        Ok(services) => {
            let response = AvailableServicesListResponse {
                services,
                patient_id: *patient_id,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}