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
    pub availability_slot_id: Uuid,
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
pub struct SymptomFilter {
    pub symptoms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceWithSymptoms {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration_minutes: i32,
    pub price: Option<f64>,
    pub category: Option<String>,
    pub symptoms: Vec<String>,
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