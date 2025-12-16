use log::{info};
use env_logger::Env;
use std::path::Path;
use std::env;
use actix_web::rt::System;

mod server;
mod routes;
mod services;
mod utils;

fn main() {
    // 初始化日志
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    info!("Starting cwebp API server...");
    
    // 设置输出目录
    let output_dir = Path::new("/app/img_webp");
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
        info!("Created output directory: {:?}", output_dir);
    }
    
    // 从环境变量读取deltime参数，默认为72小时
    let deltime_str = env::var("DELTIME").unwrap_or("72".to_string());
    let deltime = deltime_str.parse::<i32>().unwrap_or(72);
    info!("File deletion time set to: {} hours (0 means no deletion)", deltime);
    
    // 从环境变量读取图片大小限制参数，默认为100MB
    let imgsize_str = env::var("IMGSIZE").unwrap_or("100".to_string());
    let imgsize = imgsize_str.parse::<u64>().unwrap_or(100) * 1024 * 1024; // 转换为字节
    info!("Image size limit set to: {} bytes", imgsize);
    
    // 从环境变量读取API密钥，默认为dev
    let x_api_key = env::var("X_API_KEY").unwrap_or("dev".to_string());
    info!("API Key set to: {}", x_api_key);
    
    // 启动异步运行时并运行服务器
    System::new().block_on(async move {
        server::run(output_dir, deltime, imgsize, x_api_key).await.expect("Failed to start server");
    });
}
