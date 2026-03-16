use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Patient {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clinician {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub title: Option<String>,
    pub specialty: String,
    pub license_number: Option<String>,
    pub experience_years: Option<i32>,
    pub rating: Option<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration_minutes: i32,
    pub price: Option<f64>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Symptom {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub body_system: Option<String>,
    pub severity_level: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Condition {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icd_code: Option<String>,
    pub chronic: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AvailabilitySlot {
    pub id: Uuid,
    pub clinician_id: Uuid,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub max_patients: i32,
    pub booked_patients: i32,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Booking {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn get_pool(database_url: &str) -> Pool {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());
    
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create database pool")
}

// Database initialization function
pub async fn init_db(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    // In a real implementation, this would initialize the database with the schema
    // For now, we'll just ensure the connection works
    let client = pool.get().await?;
    client.execute("SELECT 1", &[]).await?;
    Ok(())
}

// Patient operations
pub async fn create_patient(pool: &Pool, patient: &Patient) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO patients (first_name, last_name, date_of_birth, gender, phone, email, address)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id",
        &[&patient.first_name, &patient.last_name, &patient.date_of_birth, 
          &patient.gender, &patient.phone, &patient.email, &patient.address]
    ).await?;
    
    Ok(row.get("id"))
}

pub async fn get_patient(pool: &Pool, patient_id: Uuid) -> Result<Option<Patient>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, first_name, last_name, date_of_birth, gender, phone, email, address, created_at, updated_at
         FROM patients WHERE id = $1",
        &[&patient_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(Patient {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            date_of_birth: row.get("date_of_birth"),
            gender: row.get("gender"),
            phone: row.get("phone"),
            email: row.get("email"),
            address: row.get("address"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

// Clinician operations
pub async fn search_clinicians_by_symptom(
    pool: &Pool, 
    symptom: &str
) -> Result<Vec<Clinician>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT DISTINCT c.id, c.first_name, c.last_name, c.title, c.specialty, 
                c.license_number, c.experience_years, c.rating, c.created_at, c.updated_at
         FROM clinicians c
         JOIN symptom_conditions sc ON c.specialty = ANY(
             SELECT body_system FROM symptoms WHERE name ILIKE $1
         )
         WHERE c.specialty ILIKE '%' || $1 || '%'
         ORDER BY c.rating DESC NULLS LAST
         LIMIT 20",
        &[&symptom]
    ).await?;
    
    let clinicians = rows.into_iter().map(|row| Clinician {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        title: row.get("title"),
        specialty: row.get("specialty"),
        license_number: row.get("license_number"),
        experience_years: row.get("experience_years"),
        rating: row.get("rating"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect();
    
    Ok(clinicians)
}

pub async fn get_clinician(
    pool: &Pool, 
    clinician_id: Uuid
) -> Result<Option<Clinician>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, first_name, last_name, title, specialty, license_number, 
                experience_years, rating, created_at, updated_at
         FROM clinicians WHERE id = $1",
        &[&clinician_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(Clinician {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            title: row.get("title"),
            specialty: row.get("specialty"),
            license_number: row.get("license_number"),
            experience_years: row.get("experience_years"),
            rating: row.get("rating"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

// Service operations
pub async fn get_service(
    pool: &Pool, 
    service_id: Uuid
) -> Result<Option<Service>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, name, description, duration_minutes, price, category
         FROM services WHERE id = $1",
        &[&service_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(Service {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            duration_minutes: row.get("duration_minutes"),
            price: row.get("price"),
            category: row.get("category"),
        })),
        None => Ok(None),
    }
}

// Availability operations
pub async fn get_available_slots(
    pool: &Pool, 
    clinician_id: Uuid,
    date: NaiveDate
) -> Result<Vec<AvailabilitySlot>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT id, clinician_id, date, start_time, end_time, max_patients, 
                booked_patients, status, created_at, updated_at
         FROM availability_slots 
         WHERE clinician_id = $1 AND date = $2 AND status = 'available'
         ORDER BY start_time",
        &[&clinician_id, &date]
    ).await?;
    
    let slots = rows.into_iter().map(|row| AvailabilitySlot {
        id: row.get("id"),
        clinician_id: row.get("clinician_id"),
        date: row.get("date"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
        max_patients: row.get("max_patients"),
        booked_patients: row.get("booked_patients"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect();
    
    Ok(slots)
}

// Booking operations
pub async fn create_booking(
    pool: &Pool, 
    booking: &Booking
) -> Result<Booking, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO bookings (patient_id, clinician_id, service_id, booking_date, 
                               booking_time, duration_minutes, status, symptoms_reported, consultation_reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id, patient_id, clinician_id, service_id, booking_date, 
                   booking_time, duration_minutes, status, symptoms_reported, consultation_reason, 
                   created_at, updated_at",
        &[&booking.patient_id, &booking.clinician_id, &booking.service_id, 
          &booking.booking_date, &booking.booking_time, &booking.duration_minutes,
          &booking.status, &booking.symptoms_reported, &booking.consultation_reason]
    ).await?;
    
    Ok(Booking {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        clinician_id: row.get("clinician_id"),
        service_id: row.get("service_id"),
        booking_date: row.get("booking_date"),
        booking_time: row.get("booking_time"),
        duration_minutes: row.get("duration_minutes"),
        status: row.get("status"),
        symptoms_reported: row.get("symptoms_reported"),
        consultation_reason: row.get("consultation_reason"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_booking(
    pool: &Pool, 
    booking_id: Uuid
) -> Result<Option<Booking>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, patient_id, clinician_id, service_id, booking_date, 
                booking_time, duration_minutes, status, symptoms_reported, consultation_reason, 
                created_at, updated_at
         FROM bookings WHERE id = $1",
        &[&booking_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(Booking {
            id: row.get("id"),
            patient_id: row.get("patient_id"),
            clinician_id: row.get("clinician_id"),
            service_id: row.get("service_id"),
            booking_date: row.get("booking_date"),
            booking_time: row.get("booking_time"),
            duration_minutes: row.get("duration_minutes"),
            status: row.get("status"),
            symptoms_reported: row.get("symptoms_reported"),
            consultation_reason: row.get("consultation_reason"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

pub async fn cancel_booking(
    pool: &Pool, 
    booking_id: Uuid
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client.execute(
        "UPDATE bookings SET status = 'cancelled' WHERE id = $1 AND status = 'confirmed'",
        &[&booking_id]
    ).await?;
    
    Ok(result > 0)
}

// Document struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Document {
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
    pub content: Option<Vec<u8>>,
    pub content_text: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub page_count: Option<i32>,
    pub status: String,
    pub is_patient_visible: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Document operations
pub async fn create_document(
    pool: &Pool,
    doc: &Document,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO documents (patient_id, clinician_id, booking_id, consultation_id, 
                                category, document_type, title, description, file_name, 
                                mime_type, content, content_text, file_size_bytes, 
                                is_patient_visible, metadata, created_by)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
         RETURNING id",
        &[
            &doc.patient_id,
            &doc.clinician_id,
            &doc.booking_id,
            &doc.consultation_id,
            &doc.category,
            &doc.document_type,
            &doc.title,
            &doc.description,
            &doc.file_name,
            &doc.mime_type,
            &doc.content,
            &doc.content_text,
            &doc.file_size_bytes,
            &doc.is_patient_visible,
            &doc.metadata,
            &doc.created_by,
        ],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_document(
    pool: &Pool,
    document_id: Uuid,
) -> Result<Option<Document>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, patient_id, clinician_id, booking_id, consultation_id, category,
                document_type, title, description, file_name, mime_type, content,
                content_text, file_size_bytes, page_count, status, is_patient_visible,
                metadata, created_by, created_at, updated_at
         FROM documents WHERE id = $1",
        &[&document_id],
    ).await?;

    match row {
        Some(row) => Ok(Some(Document {
            id: row.get("id"),
            patient_id: row.get("patient_id"),
            clinician_id: row.get("clinician_id"),
            booking_id: row.get("booking_id"),
            consultation_id: row.get("consultation_id"),
            category: row.get("category"),
            document_type: row.get("document_type"),
            title: row.get("title"),
            description: row.get("description"),
            file_name: row.get("file_name"),
            mime_type: row.get("mime_type"),
            content: row.get("content"),
            content_text: row.get("content_text"),
            file_size_bytes: row.get("file_size_bytes"),
            page_count: row.get("page_count"),
            status: row.get("status"),
            is_patient_visible: row.get("is_patient_visible"),
            metadata: row.get("metadata"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

pub async fn get_documents_by_patient(
    pool: &Pool,
    patient_id: Uuid,
    category: Option<&str>,
    status: Option<&str>,
    page: usize,
    limit: usize,
) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let offset = page * limit;

    let mut where_clause = String::from("WHERE patient_id = $1");
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&patient_id];
    let mut param_idx = 2;

    if let Some(cat) = category {
        where_clause.push_str(&format!(" AND category = ${}", param_idx));
        params.push(cat);
        param_idx += 1;
    }

    if let Some(s) = status {
        where_clause.push_str(&format!(" AND status = ${}", param_idx));
        params.push(s);
        param_idx += 1;
    }

    let query = format!(
        "SELECT id, patient_id, clinician_id, booking_id, consultation_id, category,
                document_type, title, description, file_name, mime_type, content,
                content_text, file_size_bytes, page_count, status, is_patient_visible,
                metadata, created_by, created_at, updated_at
         FROM documents {} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );

    params.push(&limit);
    params.push(&offset);

    let rows = client.query(&query, &params).await?;

    Ok(rows.into_iter().map(|row| Document {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        clinician_id: row.get("clinician_id"),
        booking_id: row.get("booking_id"),
        consultation_id: row.get("consultation_id"),
        category: row.get("category"),
        document_type: row.get("document_type"),
        title: row.get("title"),
        description: row.get("description"),
        file_name: row.get("file_name"),
        mime_type: row.get("mime_type"),
        content: row.get("content"),
        content_text: row.get("content_text"),
        file_size_bytes: row.get("file_size_bytes"),
        page_count: row.get("page_count"),
        status: row.get("status"),
        is_patient_visible: row.get("is_patient_visible"),
        metadata: row.get("metadata"),
        created_by: row.get("created_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect())
}

pub async fn update_document(
    pool: &Pool,
    document_id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
    category: Option<&str>,
    status: Option<&str>,
    is_patient_visible: Option<bool>,
    metadata: Option<&serde_json::Value>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut param_idx = 1;

    if let Some(t) = title {
        sets.push(format!("title = ${}", param_idx));
        params.push(t);
        param_idx += 1;
    }
    if let Some(d) = description {
        sets.push(format!("description = ${}", param_idx));
        params.push(d);
        param_idx += 1;
    }
    if let Some(c) = category {
        sets.push(format!("category = ${}", param_idx));
        params.push(c);
        param_idx += 1;
    }
    if let Some(s) = status {
        sets.push(format!("status = ${}", param_idx));
        params.push(s);
        param_idx += 1;
    }
    if let Some(ipv) = is_patient_visible {
        sets.push(format!("is_patient_visible = ${}", param_idx));
        params.push(ipv);
        param_idx += 1;
    }
    if let Some(m) = metadata {
        sets.push(format!("metadata = ${}", param_idx));
        params.push(m);
        param_idx += 1;
    }

    if sets.is_empty() {
        return Ok(false);
    }

    params.push(&document_id);
    let query = format!(
        "UPDATE documents SET {} WHERE id = ${}",
        sets.join(", "),
        param_idx
    );

    let result = client.execute(&query, &params).await?;
    Ok(result > 0)
}

pub async fn delete_document(
    pool: &Pool,
    document_id: Uuid,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client.execute(
        "UPDATE documents SET status = 'deleted' WHERE id = $1 AND status = 'active'",
        &[&document_id],
    ).await?;

    Ok(result > 0)
}

pub async fn stream_document_content(
    pool: &Pool,
    document_id: Uuid,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT content FROM documents WHERE id = $1 AND status = 'active'",
        &[&document_id],
    ).await?;

    match row {
        Some(row) => Ok(row.get("content")),
        None => Ok(None),
    }
}