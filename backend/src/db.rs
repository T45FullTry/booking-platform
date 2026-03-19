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
        "INSERT INTO bookings (patient_id, booker_id, clinician_id, service_id, booking_date, 
                               booking_time, duration_minutes, status, symptoms_reported, consultation_reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id, patient_id, booker_id, clinician_id, service_id, booking_date, 
                   booking_time, duration_minutes, status, symptoms_reported, consultation_reason, 
                   created_at, updated_at",
        &[&booking.patient_id, &booking.booker_id, &booking.clinician_id, &booking.service_id, 
          &booking.booking_date, &booking.booking_time, &booking.duration_minutes,
          &booking.status, &booking.symptoms_reported, &booking.consultation_reason]
    ).await?;
    
    Ok(Booking {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        booker_id: row.get("booker_id"),
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
        "SELECT id, patient_id, booker_id, clinician_id, service_id, booking_date, 
                booking_time, duration_minutes, status, symptoms_reported, consultation_reason, 
                created_at, updated_at
         FROM bookings WHERE id = $1",
        &[&booking_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(Booking {
            id: row.get("id"),
            patient_id: row.get("patient_id"),
            booker_id: row.get("booker_id"),
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

// Organization Type struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrganizationType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Organization struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Organization {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Organization Type operations
pub async fn get_organization_types(pool: &Pool) -> Result<Vec<OrganizationType>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT id, name, description, created_at FROM organization_types ORDER BY name",
        &[]
    ).await?;
    
    Ok(rows.into_iter().map(|row| OrganizationType {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        created_at: row.get("created_at"),
    }).collect())
}

pub async fn get_organization_type(pool: &Pool, type_id: Uuid) -> Result<Option<OrganizationType>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT id, name, description, created_at FROM organization_types WHERE id = $1",
        &[&type_id]
    ).await?;
    
    match row {
        Some(row) => Ok(Some(OrganizationType {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        })),
        None => Ok(None),
    }
}

// Organization operations
pub async fn create_organization(
    pool: &Pool,
    org: &Organization,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO organizations (name, organization_type_id, registration_number, tax_id, 
                                    website, email, phone, fax, address, city, state_province, 
                                    postal_code, country, contact_person_name, contact_person_email, 
                                    contact_person_phone, notes, status, metadata)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
         RETURNING id",
        &[
            &org.name,
            &org.organization_type_id,
            &org.registration_number,
            &org.tax_id,
            &org.website,
            &org.email,
            &org.phone,
            &org.fax,
            &org.address,
            &org.city,
            &org.state_province,
            &org.postal_code,
            &org.country,
            &org.contact_person_name,
            &org.contact_person_email,
            &org.contact_person_phone,
            &org.notes,
            &org.status,
            &org.metadata,
        ],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_organization(
    pool: &Pool,
    organization_id: Uuid,
) -> Result<Option<Organization>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT o.id, o.name, o.organization_type_id, ot.name as organization_type_name,
                o.registration_number, o.tax_id, o.website, o.email, o.phone, o.fax,
                o.address, o.city, o.state_province, o.postal_code, o.country,
                o.contact_person_name, o.contact_person_email, o.contact_person_phone,
                o.notes, o.status, o.metadata, o.created_at, o.updated_at
         FROM organizations o
         JOIN organization_types ot ON o.organization_type_id = ot.id
         WHERE o.id = $1",
        &[&organization_id],
    ).await?;

    match row {
        Some(row) => Ok(Some(Organization {
            id: row.get("id"),
            name: row.get("name"),
            organization_type_id: row.get("organization_type_id"),
            organization_type_name: row.get("organization_type_name"),
            registration_number: row.get("registration_number"),
            tax_id: row.get("tax_id"),
            website: row.get("website"),
            email: row.get("email"),
            phone: row.get("phone"),
            fax: row.get("fax"),
            address: row.get("address"),
            city: row.get("city"),
            state_province: row.get("state_province"),
            postal_code: row.get("postal_code"),
            country: row.get("country"),
            contact_person_name: row.get("contact_person_name"),
            contact_person_email: row.get("contact_person_email"),
            contact_person_phone: row.get("contact_person_phone"),
            notes: row.get("notes"),
            status: row.get("status"),
            metadata: row.get("metadata"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

pub async fn get_organizations(
    pool: &Pool,
    filter: Option<&str>,
    status: Option<&str>,
    page: usize,
    limit: usize,
) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let offset = page * limit;

    let mut where_clause = String::from("WHERE 1=1");
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
    let mut param_idx = 1;

    if let Some(f) = filter {
        where_clause.push_str(&format!(" AND (o.name ILIKE ${} OR ot.name ILIKE ${})", param_idx, param_idx));
        params.push(&format!("%{}%", f));
        param_idx += 1;
    }

    if let Some(s) = status {
        where_clause.push_str(&format!(" AND o.status = ${}", param_idx));
        params.push(s);
        param_idx += 1;
    }

    let query = format!(
        "SELECT o.id, o.name, o.organization_type_id, ot.name as organization_type_name,
                o.registration_number, o.tax_id, o.website, o.email, o.phone, o.fax,
                o.address, o.city, o.state_province, o.postal_code, o.country,
                o.contact_person_name, o.contact_person_email, o.contact_person_phone,
                o.notes, o.status, o.metadata, o.created_at, o.updated_at
         FROM organizations o
         JOIN organization_types ot ON o.organization_type_id = ot.id
         {} ORDER BY o.name LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );

    params.push(&limit);
    params.push(&offset);

    let rows = client.query(&query, &params).await?;

    Ok(rows.into_iter().map(|row| Organization {
        id: row.get("id"),
        name: row.get("name"),
        organization_type_id: row.get("organization_type_id"),
        organization_type_name: row.get("organization_type_name"),
        registration_number: row.get("registration_number"),
        tax_id: row.get("tax_id"),
        website: row.get("website"),
        email: row.get("email"),
        phone: row.get("phone"),
        fax: row.get("fax"),
        address: row.get("address"),
        city: row.get("city"),
        state_province: row.get("state_province"),
        postal_code: row.get("postal_code"),
        country: row.get("country"),
        contact_person_name: row.get("contact_person_name"),
        contact_person_email: row.get("contact_person_email"),
        contact_person_phone: row.get("contact_person_phone"),
        notes: row.get("notes"),
        status: row.get("status"),
        metadata: row.get("metadata"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect())
}

pub async fn update_organization(
    pool: &Pool,
    organization_id: Uuid,
    name: Option<&str>,
    organization_type_id: Option<Uuid>,
    registration_number: Option<&str>,
    tax_id: Option<&str>,
    website: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    address: Option<&str>,
    city: Option<&str>,
    state_province: Option<&str>,
    postal_code: Option<&str>,
    country: Option<&str>,
    contact_person_name: Option<&str>,
    contact_person_email: Option<&str>,
    contact_person_phone: Option<&str>,
    notes: Option<&str>,
    status: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut param_idx = 1;

    if let Some(n) = name {
        sets.push(format!("name = ${}", param_idx));
        params.push(n);
        param_idx += 1;
    }
    if let Some(otid) = organization_type_id {
        sets.push(format!("organization_type_id = ${}", param_idx));
        params.push(&otid);
        param_idx += 1;
    }
    if let Some(rn) = registration_number {
        sets.push(format!("registration_number = ${}", param_idx));
        params.push(rn);
        param_idx += 1;
    }
    if let Some(tid) = tax_id {
        sets.push(format!("tax_id = ${}", param_idx));
        params.push(tid);
        param_idx += 1;
    }
    if let Some(w) = website {
        sets.push(format!("website = ${}", param_idx));
        params.push(w);
        param_idx += 1;
    }
    if let Some(e) = email {
        sets.push(format!("email = ${}", param_idx));
        params.push(e);
        param_idx += 1;
    }
    if let Some(p) = phone {
        sets.push(format!("phone = ${}", param_idx));
        params.push(p);
        param_idx += 1;
    }
    if let Some(a) = address {
        sets.push(format!("address = ${}", param_idx));
        params.push(a);
        param_idx += 1;
    }
    if let Some(c) = city {
        sets.push(format!("city = ${}", param_idx));
        params.push(c);
        param_idx += 1;
    }
    if let Some(sp) = state_province {
        sets.push(format!("state_province = ${}", param_idx));
        params.push(sp);
        param_idx += 1;
    }
    if let Some(pc) = postal_code {
        sets.push(format!("postal_code = ${}", param_idx));
        params.push(pc);
        param_idx += 1;
    }
    if let Some(co) = country {
        sets.push(format!("country = ${}", param_idx));
        params.push(co);
        param_idx += 1;
    }
    if let Some(cpn) = contact_person_name {
        sets.push(format!("contact_person_name = ${}", param_idx));
        params.push(cpn);
        param_idx += 1;
    }
    if let Some(cpe) = contact_person_email {
        sets.push(format!("contact_person_email = ${}", param_idx));
        params.push(cpe);
        param_idx += 1;
    }
    if let Some(cpp) = contact_person_phone {
        sets.push(format!("contact_person_phone = ${}", param_idx));
        params.push(cpp);
        param_idx += 1;
    }
    if let Some(n) = notes {
        sets.push(format!("notes = ${}", param_idx));
        params.push(n);
        param_idx += 1;
    }
    if let Some(s) = status {
        sets.push(format!("status = ${}", param_idx));
        params.push(s);
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

    params.push(&organization_id);
    let query = format!(
        "UPDATE organizations SET {} WHERE id = ${}",
        sets.join(", "),
        param_idx
    );

    let result = client.execute(&query, &params).await?;
    Ok(result > 0)
}

pub async fn delete_organization(
    pool: &Pool,
    organization_id: Uuid,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = client.execute(
        "UPDATE organizations SET status = 'inactive' WHERE id = $1 AND status = 'active'",
        &[&organization_id],
    ).await?;

    Ok(result > 0)
}

// Patient Employment struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PatientEmployment {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_patient_employment(
    pool: &Pool,
    patient_id: Uuid,
    organization_id: Uuid,
    job_title: Option<&str>,
    department: Option<&str>,
    employee_id: Option<&str>,
    start_date: Option<NaiveDate>,
    notes: Option<&str>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO patient_employments (patient_id, organization_id, job_title, department, 
                                          employee_id, start_date, is_current, notes)
         VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7)
         RETURNING id",
        &[&patient_id, &organization_id, &job_title, &department, &employee_id, &start_date, &notes],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_patient_employments(
    pool: &Pool,
    patient_id: Uuid,
) -> Result<Vec<PatientEmployment>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT pe.id, pe.patient_id, p.first_name || ' ' || p.last_name as patient_name,
                pe.organization_id, o.name as organization_name,
                pe.job_title, pe.department, pe.employee_id, pe.start_date, pe.end_date,
                pe.is_current, pe.notes, pe.created_at, pe.updated_at
         FROM patient_employments pe
         JOIN patients p ON pe.patient_id = p.id
         JOIN organizations o ON pe.organization_id = o.id
         WHERE pe.patient_id = $1
         ORDER BY pe.is_current DESC, pe.start_date DESC",
        &[&patient_id],
    ).await?;

    Ok(rows.into_iter().map(|row| PatientEmployment {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        patient_name: row.get("patient_name"),
        organization_id: row.get("organization_id"),
        organization_name: row.get("organization_name"),
        job_title: row.get("job_title"),
        department: row.get("department"),
        employee_id: row.get("employee_id"),
        start_date: row.get("start_date"),
        end_date: row.get("end_date"),
        is_current: row.get("is_current"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect())
}

// Clinician Affiliation struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClinicianAffiliation {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_clinician_affiliation(
    pool: &Pool,
    clinician_id: Uuid,
    organization_id: Uuid,
    affiliation_type: Option<&str>,
    department: Option<&str>,
    start_date: Option<NaiveDate>,
    is_primary: Option<bool>,
    notes: Option<&str>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO clinician_affiliations (clinician_id, organization_id, affiliation_type, 
                                              department, start_date, is_primary, is_current, notes)
         VALUES ($1, $2, $3, $4, $5, COALESCE($6, FALSE), TRUE, $7)
         RETURNING id",
        &[&clinician_id, &organization_id, &affiliation_type, &department, &start_date, &is_primary, &notes],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_clinician_affiliations(
    pool: &Pool,
    clinician_id: Uuid,
) -> Result<Vec<ClinicianAffiliation>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT ca.id, ca.clinician_id, c.first_name || ' ' || c.last_name as clinician_name,
                ca.organization_id, o.name as organization_name,
                ca.affiliation_type, ca.department, ca.start_date, ca.end_date,
                ca.is_primary, ca.is_current, ca.notes, ca.created_at, ca.updated_at
         FROM clinician_affiliations ca
         JOIN clinicians c ON ca.clinician_id = c.id
         JOIN organizations o ON ca.organization_id = o.id
         WHERE ca.clinician_id = $1
         ORDER BY ca.is_primary DESC, ca.is_current DESC, ca.start_date DESC",
        &[&clinician_id],
    ).await?;

    Ok(rows.into_iter().map(|row| ClinicianAffiliation {
        id: row.get("id"),
        clinician_id: row.get("clinician_id"),
        clinician_name: row.get("clinician_name"),
        organization_id: row.get("organization_id"),
        organization_name: row.get("organization_name"),
        affiliation_type: row.get("affiliation_type"),
        department: row.get("department"),
        start_date: row.get("start_date"),
        end_date: row.get("end_date"),
        is_primary: row.get("is_primary"),
        is_current: row.get("is_current"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect())
}

// Booking Insurance struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookingInsurance {
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_booking_insurance(
    pool: &Pool,
    booking_id: Uuid,
    organization_id: Uuid,
    policy_number: Option<&str>,
    group_number: Option<&str>,
    member_id: Option<&str>,
    coverage_type: Option<&str>,
    authorization_required: Option<bool>,
    notes: Option<&str>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO booking_insurance (booking_id, organization_id, policy_number, group_number, 
                                        member_id, coverage_type, authorization_required, notes)
         VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, FALSE), $8)
         RETURNING id",
        &[&booking_id, &organization_id, &policy_number, &group_number, &member_id, &coverage_type, &authorization_required, &notes],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_booking_insurance(
    pool: &Pool,
    booking_id: Uuid,
) -> Result<Option<BookingInsurance>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_opt(
        "SELECT bi.id, bi.booking_id, bi.organization_id, o.name as organization_name,
                bi.policy_number, bi.group_number, bi.member_id, bi.coverage_type,
                bi.authorization_required, bi.authorization_number, bi.claim_status,
                bi.claim_amount, bi.patient_responsibility, bi.notes,
                bi.created_at, bi.updated_at
         FROM booking_insurance bi
         JOIN organizations o ON bi.organization_id = o.id
         WHERE bi.booking_id = $1",
        &[&booking_id],
    ).await?;

    match row {
        Some(row) => Ok(Some(BookingInsurance {
            id: row.get("id"),
            booking_id: row.get("booking_id"),
            organization_id: row.get("organization_id"),
            organization_name: row.get("organization_name"),
            policy_number: row.get("policy_number"),
            group_number: row.get("group_number"),
            member_id: row.get("member_id"),
            coverage_type: row.get("coverage_type"),
            authorization_required: row.get("authorization_required"),
            authorization_number: row.get("authorization_number"),
            claim_status: row.get("claim_status"),
            claim_amount: row.get("claim_amount"),
            patient_responsibility: row.get("patient_responsibility"),
            notes: row.get("notes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })),
        None => Ok(None),
    }
}

// Document Issuer struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentIssuer {
    pub id: Uuid,
    pub document_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub issuer_name: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_document_issuer(
    pool: &Pool,
    document_id: Uuid,
    organization_id: Uuid,
    issuer_name: Option<&str>,
    issue_date: Option<NaiveDate>,
    reference_number: Option<&str>,
    notes: Option<&str>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let row = client.query_one(
        "INSERT INTO document_issuers (document_id, organization_id, issuer_name, issue_date, 
                                       reference_number, notes)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        &[&document_id, &organization_id, &issuer_name, &issue_date, &reference_number, &notes],
    ).await?;

    Ok(row.get("id"))
}

pub async fn get_document_issuers(
    pool: &Pool,
    document_id: Uuid,
) -> Result<Vec<DocumentIssuer>, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let rows = client.query(
        "SELECT di.id, di.document_id, di.organization_id, o.name as organization_name,
                di.issuer_name, di.issue_date, di.reference_number, di.notes, di.created_at
         FROM document_issuers di
         JOIN organizations o ON di.organization_id = o.id
         WHERE di.document_id = $1",
        &[&document_id],
    ).await?;

    Ok(rows.into_iter().map(|row| DocumentIssuer {
        id: row.get("id"),
        document_id: row.get("document_id"),
        organization_id: row.get("organization_id"),
        organization_name: row.get("organization_name"),
        issuer_name: row.get("issuer_name"),
        issue_date: row.get("issue_date"),
        reference_number: row.get("reference_number"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
    }).collect())
}