use actix_web::{App, HttpServer, web};
use log::info;
use std::path::Path;

// 导入routes模块
use crate::routes;

/// 启动Actix Web服务器
pub async fn run(output_dir: &Path, deltime: i32, max_image_size: u64) -> std::io::Result<()> {
    let output_dir = output_dir.to_path_buf();
    let addr = "0.0.0.0:3333";
    
    info!("Server starting on {}", addr);
    info!("Health check endpoint: http://{}/health", addr);
    info!("Conversion API endpoint: http://{}/api/convert", addr);
    info!("Image access endpoint: http://{}/api/images/:filename", addr);
    info!("File deletion time: {} hours (0 means no deletion)", deltime);
    info!("Maximum image size: {} bytes", max_image_size);
    
    // 如果deltime大于0，启动定时清理任务
    if deltime > 0 {
        let cleanup_dir = output_dir.clone();
        let cleanup_hours = deltime;
        
        // 启动后台清理线程
        std::thread::spawn(move || {
            info!("Starting file cleanup task with interval: {} hours", cleanup_hours);
            
            // 立即执行一次清理
            crate::utils::file::cleanup_files(&cleanup_dir, cleanup_hours);
            
            // 定时执行清理（每小时检查一次）
            loop {
                // 等待1小时
                std::thread::sleep(std::time::Duration::from_secs(3600));
                // 执行清理
                crate::utils::file::cleanup_files(&cleanup_dir, cleanup_hours);
            }
        });
    }
    
    // 绑定并运行服务器
    HttpServer::new(move || {
        App::new()
            // 注入输出目录路径
            .app_data(web::Data::new(output_dir.clone()))
            // 注入最大图片大小限制
            .app_data(web::Data::new(max_image_size))
            // 配置路由
            .configure(routes::configure)
    })
    .bind(addr)?
    .run()
    .await?;
    
    Ok(())
}
