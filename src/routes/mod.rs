use actix_web::web;

mod convert;
mod health;

/// 配置所有路由
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // 健康检查路由
        .service(web::resource("/health").route(web::get().to(health::health_check)))
        // API路由组
        .service(web::scope("/api")
            // 转换API
            .service(web::resource("/convert")
                .route(web::post().to(convert::convert_image)))
            // 获取图片API
            .service(web::resource("/images/{id}")
                .route(web::get().to(convert::get_image))))
    ;
}
