use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    body::{to_bytes, Body},
    Json,
};
use std::borrow::Cow;
use validator::{ValidationErrors};
use crate::structs::api_response::ApiResponse;

pub fn format_validation_errors(errors: ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let messages: Vec<String> = errs
                .iter()
                .map(|e| {
                    e.message
                        .clone()
                        .map(|m: Cow<'_, str>| m.into_owned()) 
                        .unwrap_or_else(|| "invalid value".to_string())
                })
                .collect();
            format!("{}: {}", field, messages.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

pub async fn error_handler_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(req).await;
    let status = response.status();

    if status.is_client_error() || status.is_server_error() {
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await;
        let message = match body_bytes {
            Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
            Err(_) => status.to_string(),
        };

        let body = Json(ApiResponse::<()>::error(message));
        return (status, body).into_response();
    }

    response
}