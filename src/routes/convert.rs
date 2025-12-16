use actix_web::{Responder, web, HttpResponse, HttpRequest, error::ErrorInternalServerError};
use actix_multipart::Multipart;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tempfile::NamedTempFile;
use base64::prelude::*;
use log::{info, debug};
use chrono::Utc;
use futures_util::stream::StreamExt;

use crate::services::cwebp::{self, ConversionOptions};

/// 转换请求参数
#[derive(Deserialize, Debug, Default)]
pub struct ConvertParams {
    #[serde(default = "default_lossless")]
    lossless: bool,
    #[serde(default = "default_quality")]
    quality: u8,
    #[serde(default = "default_near_lossless")]
    near_lossless: u8,
    #[serde(default = "default_compression_level")]
    compression_level: u8,
    preset: Option<String>,
    #[serde(default = "default_method")]
    method: u8,
    #[serde(default = "default_response_type")]
    response_type: String,
}

// 默认值函数
fn default_lossless() -> bool { false }
fn default_quality() -> u8 { 80 }
fn default_near_lossless() -> u8 { 100 }
fn default_compression_level() -> u8 { 6 }
fn default_method() -> u8 { 4 }
fn default_response_type() -> String { "webp".to_string() }

/// webp响应格式
#[derive(Serialize)]
pub struct WebpResponse {
    success: bool,
    response_type: String,
    file_id: String,
    filename: String,
    url: String,
    full_url: String,
    timestamp: String,
}

/// base64响应格式
#[derive(Serialize)]
pub struct Base64Response {
    success: bool,
    response_type: String,
    data: String,
    format: String,
    filename: String,
    timestamp: String,
}

/// 错误响应格式
#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
    message: String,
}

/// 转换图片API
pub async fn convert_image(
    req: HttpRequest,
    payload: Multipart,
    temp_dir: web::Data<std::path::PathBuf>,
    max_image_size: web::Data<u64>,
) -> Result<impl Responder, actix_web::Error> {
    let temp_dir = temp_dir.into_inner();
    
    // 解析表单数据
    let (file_content, filename, params) = parse_multipart(payload, **max_image_size).await?;
    
    info!("Converting image: {} with response_type: {}", filename, params.response_type);
    debug!("Conversion params: {:?}", params);
    
    // 创建临时输入文件
    let mut input_file = NamedTempFile::new_in(&*temp_dir)
        .map_err(|e| ErrorInternalServerError(format!("Failed to create temp file: {}", e)))?;
    
    // 写入文件内容
    input_file.write_all(&file_content)
        .map_err(|e| ErrorInternalServerError(format!("Failed to write temp file: {}", e)))?;
    
    // 生成输出文件路径
    let base_filename = filename.split(".").next().unwrap_or("image").to_string();
    let output_filename = format!("{}.webp", base_filename);
    let output_path = temp_dir.join(&output_filename);
    
    // 构建转换选项
    let conversion_opts = ConversionOptions {
        lossless: params.lossless,
        quality: params.quality,
        near_lossless: params.near_lossless,
        compression_level: params.compression_level,
        preset: params.preset,
        method: params.method,
    };
    
    // 执行转换
    cwebp::convert_to_webp(input_file.path(), &output_path, &conversion_opts)
        .map_err(|e| ErrorInternalServerError(format!("Conversion failed: {}", e)))?;
    
    // 获取基本文件名（不含扩展名）
    let base_filename = filename.split(".").next().unwrap_or("image").to_string();
    let output_filename = format!("{}.webp", base_filename);
    
    // 构建响应
    let response_type = params.response_type.to_lowercase();
    if response_type == "base64" {
        // 读取转换后的图片
        let webp_content = std::fs::read(&output_path)
            .map_err(|e| ErrorInternalServerError(format!("Failed to read output file: {}", e)))?;
        
        // 转换为base64
        let base64_data = BASE64_STANDARD.encode(&webp_content);
        
        // 保留文件，不删除
        
        // 返回base64响应
        let response = Base64Response {
            success: true,
            response_type: "base64".to_string(),
            data: base64_data,
            format: "webp".to_string(),
            filename: output_filename.clone(),
            timestamp: Utc::now().to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(response))
        
    } else {
        // 返回webp下载地址响应
        let url = format!("/api/images/{}", output_filename);
        let full_url = format!("http://{}{}", req.connection_info().host(), url);
        
        let response = WebpResponse {
            success: true,
            response_type: "webp".to_string(),
            file_id: output_filename.clone(),
            filename: output_filename.clone(),
            url,
            full_url,
            timestamp: Utc::now().to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(response))
    }
}

/// 获取转换后的webp图片
pub async fn get_image(
    path: web::Path<String>,
    temp_dir: web::Data<std::path::PathBuf>,
) -> Result<impl Responder, actix_web::Error> {
    let filename = path.into_inner();
    let temp_dir = temp_dir.into_inner();
    
    // 构建文件路径
    let file_path = temp_dir.join(&filename);
    
    debug!("Requesting image: {}, path: {:?}", filename, file_path);
    
    // 检查文件是否存在
    if !file_path.exists() {
        let response = ErrorResponse {
            success: false,
            error: "File not found".to_string(),
            message: "The requested image file does not exist".to_string(),
        };
        return Ok(HttpResponse::NotFound().json(response));
    }
    
    // 读取文件内容
    let file_content = std::fs::read(&file_path)?;
    
    // 返回图片数据
    Ok(HttpResponse::Ok()
        .content_type("image/webp")
        .append_header(("Content-Disposition", format!("inline; filename=\"{}\"", filename)))
        .body(file_content))
}

/// 解析multipart请求
async fn parse_multipart(
    mut payload: Multipart,
    max_image_size: u64
) -> Result<(Vec<u8>, String, ConvertParams), actix_web::Error> {
    let mut file_content: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut params: Option<ConvertParams> = None;
    
    while let Some(field_result) = payload.next().await {
        let mut field = field_result.map_err(ErrorInternalServerError)?;
        
        // 获取字段名称
        let field_name = field.name().to_string();
        
        if field_name == "image" {
            // 处理图片文件
            let mut content = Vec::new();
            while let Some(chunk_result) = field.next().await {
                let chunk = chunk_result.map_err(ErrorInternalServerError)?;
                
                // 检查图片大小是否超过限制
                if content.len() + chunk.len() > max_image_size as usize {
                    return Err(actix_web::error::ErrorBadRequest(format!("Image size exceeds maximum limit of {} bytes", max_image_size)));
                }
                
                content.extend_from_slice(&chunk);
            }
            
            file_content = Some(content);
            filename = field.content_disposition()
                .get_filename()
                .map(|f| f.to_string());
            
        } else if field_name == "params" {
            // 处理JSON参数
            let mut content = String::new();
            while let Some(chunk_result) = field.next().await {
                let chunk = chunk_result.map_err(ErrorInternalServerError)?;
                content.push_str(&String::from_utf8_lossy(&chunk));
            }
            
            params = Some(serde_json::from_str(&content).map_err(ErrorInternalServerError)?);
        }
    }
    
    // 确保所有必要数据都存在
    let file_content = file_content.ok_or_else(|| ErrorInternalServerError("No image file provided"))?;
    let filename = filename.ok_or_else(|| ErrorInternalServerError("No filename provided"))?;
    let params = params.unwrap_or_default();
    
    Ok((file_content, filename, params))
}
