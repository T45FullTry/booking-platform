use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentRequest {
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
    pub content_base64: Option<String>,
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

fn get_pool(database_url: &str) -> Pool {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create database pool")
}

async fn create_document(
    pool: web::Data<Pool>,
    doc_data: web::Json<CreateDocumentRequest>,
) -> Result<HttpResponse> {
    let content = if let Some(base64_str) = &doc_data.content_base64 {
        match base64::decode(base64_str) {
            Ok(bytes) => Some(bytes),
            Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid base64 content")),
        }
    } else {
        None
    };

    let file_size = content.as_ref().map(|c| c.len() as i64);

    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let doc_id = Uuid::new_v4();
    let row = client
        .query_one(
            "INSERT INTO documents (patient_id, clinician_id, booking_id, consultation_id, 
                                    category, document_type, title, description, file_name, 
                                    mime_type, content, content_text, file_size_bytes, 
                                    is_patient_visible, metadata, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
             RETURNING id",
            &[
                &doc_data.patient_id,
                &doc_data.clinician_id,
                &doc_data.booking_id,
                &doc_data.consultation_id,
                &doc_data.category,
                &doc_data.document_type,
                &doc_data.title,
                &doc_data.description,
                &doc_data.file_name,
                &doc_data.mime_type,
                &content,
                &doc_data.content_text,
                &file_size,
                &doc_data.is_patient_visible.unwrap_or(true),
                &doc_data.metadata,
                &doc_data.clinician_id,
            ],
        )
        .await;

    match row {
        Ok(_) => {
            let response = DocumentResponse {
                id: doc_id,
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
                file_size_bytes: file_size,
                page_count: None,
                status: "active".to_string(),
                is_patient_visible: doc_data.is_patient_visible.unwrap_or(true),
                metadata: doc_data.metadata.clone(),
                created_by: doc_data.clinician_id,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

async fn get_document(
    pool: web::Data<Pool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let row = client
        .query_opt(
            "SELECT id, patient_id, clinician_id, booking_id, consultation_id, category,
                    document_type, title, description, file_name, mime_type, content,
                    content_text, file_size_bytes, page_count, status, is_patient_visible,
                    metadata, created_by, created_at, updated_at
             FROM documents WHERE id = $1",
            &[&document_id],
        )
        .await;

    match row {
        Ok(Some(row)) => {
            let response = DocumentResponse {
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
                file_size_bytes: row.get("file_size_bytes"),
                page_count: row.get("page_count"),
                status: row.get("status"),
                is_patient_visible: row.get("is_patient_visible"),
                metadata: row.get("metadata"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at").to_rfc3339(),
                updated_at: row.get("updated_at").to_rfc3339(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

async fn get_documents_by_category(
    pool: web::Data<Pool>,
    filter: web::Query<DocumentCategoryFilter>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let page = filter.page.unwrap_or(0);
    let limit = filter.limit.unwrap_or(20);
    let offset = page * limit;

    let mut where_clause = String::from("WHERE 1=1");
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut param_idx = 1;

    if let Some(patient_id) = filter.patient_id {
        where_clause.push_str(&format!(" AND patient_id = ${}", param_idx));
        params.push(&patient_id);
        param_idx += 1;
    }

    if let Some(cat) = &filter.category {
        where_clause.push_str(&format!(" AND category = ${}", param_idx));
        params.push(cat);
        param_idx += 1;
    }

    if let Some(s) = &filter.status {
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

    let rows = client.query(&query, &params).await;

    match rows {
        Ok(rows) => {
            let responses: Vec<DocumentResponse> = rows.into_iter().map(|row| {
                DocumentResponse {
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
                    file_size_bytes: row.get("file_size_bytes"),
                    page_count: row.get("page_count"),
                    status: row.get("status"),
                    is_patient_visible: row.get("is_patient_visible"),
                    metadata: row.get("metadata"),
                    created_by: row.get("created_by"),
                    created_at: row.get("created_at").to_rfc3339(),
                    updated_at: row.get("updated_at").to_rfc3339(),
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

async fn update_document(
    pool: web::Data<Pool>,
    document_id: web::Path<Uuid>,
    update_data: web::Json<UpdateDocumentRequest>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut param_idx = 1;

    if let Some(t) = &update_data.title {
        sets.push(format!("title = ${}", param_idx));
        params.push(t);
        param_idx += 1;
    }
    if let Some(d) = &update_data.description {
        sets.push(format!("description = ${}", param_idx));
        params.push(d);
        param_idx += 1;
    }
    if let Some(c) = &update_data.category {
        sets.push(format!("category = ${}", param_idx));
        params.push(c);
        param_idx += 1;
    }
    if let Some(s) = &update_data.status {
        sets.push(format!("status = ${}", param_idx));
        params.push(s);
        param_idx += 1;
    }
    if let Some(ipv) = update_data.is_patient_visible {
        sets.push(format!("is_patient_visible = ${}", param_idx));
        params.push(ipv);
        param_idx += 1;
    }
    if let Some(m) = &update_data.metadata {
        sets.push(format!("metadata = ${}", param_idx));
        params.push(m);
        param_idx += 1;
    }

    if sets.is_empty() {
        return Ok(HttpResponse::BadRequest().json("No fields to update"));
    }

    params.push(&document_id);
    let query = format!(
        "UPDATE documents SET {} WHERE id = ${}",
        sets.join(", "),
        param_idx
    );

    let result = client.execute(&query, &params).await;

    match result {
        Ok(count) if count > 0 => Ok(HttpResponse::Ok().json("Document updated successfully")),
        Ok(_) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

async fn delete_document(
    pool: web::Data<Pool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let result = client
        .execute(
            "UPDATE documents SET status = 'deleted' WHERE id = $1 AND status = 'active'",
            &[&document_id],
        )
        .await;

    match result {
        Ok(count) if count > 0 => Ok(HttpResponse::Ok().json("Document deleted successfully")),
        Ok(_) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

async fn stream_document(
    pool: web::Data<Pool>,
    document_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("DB error: {}", e))),
    };

    let row = client
        .query_opt(
            "SELECT content, mime_type FROM documents WHERE id = $1 AND status = 'active'",
            &[&document_id],
        )
        .await;

    match row {
        Ok(Some(row)) => {
            let content: Option<Vec<u8>> = row.get("content");
            let mime_type: Option<String> = row.get("mime_type");
            
            match content {
                Some(bytes) => {
                    let content_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
                    Ok(HttpResponse::Ok().content_type(content_type).body(bytes))
                }
                None => Ok(HttpResponse::NotFound().json("Document content not found")),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Document not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&database_url);

    println!("Starting document service...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/documents")
                            .route("", web::post().to(create_document))
                            .route("", web::get().to(get_documents_by_category))
                            .route("/{id}", web::get().to(get_document))
                            .route("/{id}", web::put().to(update_document))
                            .route("/{id}", web::delete().to(delete_document))
                            .route("/{id}/stream", web::get().to(stream_document))
                    )
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
