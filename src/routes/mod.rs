use actix_web::{web, HttpResponse, Error, body::BoxBody, dev::{Service, Transform, ServiceRequest, ServiceResponse}};
use futures_util::future::FutureExt;
use futures_util::future::LocalBoxFuture;
use std::task::{Context, Poll};

mod convert;
mod health;

/// API密钥验证中间件
pub struct ApiKeyAuth;

impl<S> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ApiKeyAuthMiddleware<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        async move {
            Ok(ApiKeyAuthMiddleware { service })
        }
        .boxed_local()
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 获取API密钥
        let api_key = req.app_data::<web::Data<String>>()
            .map(|data| data.as_ref().clone())
            .unwrap_or("dev".to_string());
        
        // 从请求头中获取API密钥
        let auth_header = req.headers().get("x-api-key");
        
        // 验证API密钥
        match auth_header {
            Some(header) => {
                let header_value = header.to_str().unwrap_or("");
                if header_value == api_key {
                    // API密钥验证通过，继续处理请求
                    let fut = self.service.call(req);
                    async move {
                        fut.await
                    }
                    .boxed_local()
                } else {
                        // API密钥验证失败，返回401错误
                        async move {
                            Ok(req.into_response(
                                HttpResponse::Unauthorized()
                                    .json(serde_json::json!({ "success": false, "error": "Unauthorized", "message": "Invalid API Key" }))
                            ))
                        }
                        .boxed_local()
                    }
            },
            None => {
                // 没有提供API密钥，返回401错误
                async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({ "success": false, "error": "Unauthorized", "message": "API Key is required" }))
                    ))
                }
                .boxed_local()
            }
        }
    }
}

/// 配置所有路由
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // 健康检查路由
        .service(web::resource("/health").route(web::get().to(health::health_check)))
        // API路由组，添加API密钥验证中间件
        .service(web::scope("/api")
            // 添加API密钥验证中间件
            .wrap(ApiKeyAuth)
            // 转换API
            .service(web::resource("/convert")
                .route(web::post().to(convert::convert_image)))
            // 获取图片API
            .service(web::resource("/images/{id}")
                .route(web::get().to(convert::get_image))))
    ;
}
