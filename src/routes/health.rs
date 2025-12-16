use actix_web::{Responder, HttpResponse};
use serde::Serialize;
use chrono::Utc;

/// 健康检查响应
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: String,
    service: String,
    version: String,
}

/// 健康检查端点
pub async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        service: "cwebp-api".to_string(),
        version: "1.0.0".to_string(),
    };
    
    HttpResponse::Ok().json(response)
}
