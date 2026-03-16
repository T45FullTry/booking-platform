use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};

// Patient models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePatientRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Booking models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBookingRequest {
    pub patient_id: Uuid,
    pub clinician_id: Uuid,
    pub service_id: Uuid,
    pub booking_date: NaiveDate,
    pub booking_time: NaiveTime,
    pub duration_minutes: i32,
    pub symptoms_reported: Option<Vec<String>>,
    pub consultation_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingResponse {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub clinician_id: Uuid,
    pub service_id: Uuid,
    pub booking_date: NaiveDate,
    pub booking_time: NaiveTime,
    pub duration_minutes: i32,
    pub status: String,
    pub symptoms_reported: Option<Vec<String>>,
    pub consultation_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Availability models
#[derive(Debug, Deserialize, Serialize)]
pub struct AvailabilityRequest {
    pub clinician_id: Uuid,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailabilityResponse {
    pub date: NaiveDate,
    pub slots: Vec<TimeSlot>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSlot {
    pub id: Uuid,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub available: bool,
}

// Clinician search models
#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicianSearchRequest {
    pub symptom: Option<String>,
    pub condition: Option<String>,
    pub specialty: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicianSearchResponse {
    pub results: Vec<ClinicianResult>,
    pub total_count: usize,
    pub page: usize,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicianResult {
    pub id: Uuid,
    pub name: String,
    pub specialty: String,
    pub rating: Option<f32>,
    pub available_times: Vec<String>,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicianDetailsResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub title: Option<String>,
    pub specialty: String,
    pub experience_years: Option<i32>,
    pub rating: Option<f32>,
    pub bio: Option<String>,
    pub education: Option<String>,
    pub languages: Vec<String>,
    pub available_times: Vec<String>,
    pub location: String,
}

// Cancel booking models
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelBookingRequest {
    pub booking_id: Uuid,
}

// Document models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    pub patient_id: Uuid,
    pub clinician_id: Uuid,
    pub booking_id: Option<Uuid>,
    pub consultation_id: Option<Uuid>,
    pub category: String, // sick_note, referral, prescription, lab_result, medical_report, discharge_summary, other
    pub document_type: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub content_base64: Option<String>, // Base64 encoded binary content
    pub content_text: Option<String>,
    pub is_patient_visible: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub is_patient_visible: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentResponse {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub clinician_id: Uuid,
    pub booking_id: Option<Uuid>,
    pub consultation_id: Option<Uuid>,
    pub category: String,
    pub document_type: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub page_count: Option<i32>,
    pub status: String,
    pub is_patient_visible: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentResponse>,
    pub total_count: usize,
    pub page: usize,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentCategoryFilter {
    pub category: Option<String>,
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}