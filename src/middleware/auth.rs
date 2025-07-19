use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

// api 授权白名单
pub const AUTH_WHITELIST: [&str; 1] = ["/api/login"];

pub async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let method_path_str = format!("[{}] - [{}]", req.method(), req.path());
    // 记录请求信息到日志
    info!("PRE: {method_path_str} - Headers: {:#?}", &req.headers());
    let res = next.call(req).await;
    let _ = match &res {
        Ok(response) => {
            // 记录响应信息到日志
            info!(
                "POST: {method_path_str}- Response status: {}",
                response.status()
            );
        }
        Err(err) => {
            error!("Error occurred: {}", err);
        }
    };
    res
}
