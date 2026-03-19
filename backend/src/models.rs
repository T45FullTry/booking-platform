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
    pub patient_id: Uuid,      // The person the appointment is for (appointee)
    pub booker_id: Uuid,       // The person making the booking
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
    pub patient_id: Uuid,      // The person the appointment is for (appointee)
    pub booker_id: Uuid,       // The person making the booking
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

// Organization models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrganizationTypeRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub organization_type_id: Uuid,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub organization_type_id: Option<Uuid>,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub organization_type_id: Uuid,
    pub organization_type_name: String,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,
    pub notes: Option<String>,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationResponse>,
    pub total_count: usize,
    pub page: usize,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationFilter {
    pub organization_type_id: Option<Uuid>,
    pub organization_type_name: Option<String>,
    pub status: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Patient Employment models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePatientEmploymentRequest {
    pub patient_id: Uuid,
    pub organization_id: Uuid,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub employee_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientEmploymentResponse {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub patient_name: String,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub employee_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Clinician Affiliation models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateClinicianAffiliationRequest {
    pub clinician_id: Uuid,
    pub organization_id: Uuid,
    pub affiliation_type: Option<String>,
    pub department: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicianAffiliationResponse {
    pub id: Uuid,
    pub clinician_id: Uuid,
    pub clinician_name: String,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub affiliation_type: Option<String>,
    pub department: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_primary: bool,
    pub is_current: bool,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Booking Insurance models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBookingInsuranceRequest {
    pub booking_id: Uuid,
    pub organization_id: Uuid,
    pub policy_number: Option<String>,
    pub group_number: Option<String>,
    pub member_id: Option<String>,
    pub coverage_type: Option<String>,
    pub authorization_required: Option<bool>,
    pub authorization_number: Option<String>,
    pub claim_status: Option<String>,
    pub claim_amount: Option<rust_decimal::Decimal>,
    pub patient_responsibility: Option<rust_decimal::Decimal>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingInsuranceResponse {
    pub id: Uuid,
    pub booking_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub policy_number: Option<String>,
    pub group_number: Option<String>,
    pub member_id: Option<String>,
    pub coverage_type: Option<String>,
    pub authorization_required: bool,
    pub authorization_number: Option<String>,
    pub claim_status: Option<String>,
    pub claim_amount: Option<rust_decimal::Decimal>,
    pub patient_responsibility: Option<rust_decimal::Decimal>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Document Issuer models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentIssuerRequest {
    pub document_id: Uuid,
    pub organization_id: Uuid,
    pub issuer_name: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentIssuerResponse {
    pub id: Uuid,
    pub document_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub issuer_name: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}