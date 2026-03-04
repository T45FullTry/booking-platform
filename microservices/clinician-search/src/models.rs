use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub symptom: Option<String>,
    pub condition: Option<String>,
    pub specialty: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub name: String,
    pub specialty: String,
    pub rating: f32,
    pub available_times: Vec<String>,
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
    pub page: usize,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct ClinicianDetails {
    pub id: Uuid,
    pub name: String,
    pub specialty: String,
    pub rating: f32,
    pub bio: String,
    pub education: String,
    pub experience_years: i32,
    pub available_times: Vec<String>,
    pub location: String,
    pub languages: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ClinicianResponse {
    pub clinician: ClinicianDetails,
}