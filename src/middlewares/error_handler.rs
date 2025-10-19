use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
    body::Body
};
use crate::structs::api_response::ApiResponse;

pub async fn error_handler_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(req).await;

    if response.status().is_client_error() {
        let body = Json(ApiResponse::<()> {
            payload: (),
            message: format!("Client error: {}", response.status()),
        });
        return (StatusCode::BAD_REQUEST, body).into_response();
    }

    response
}
