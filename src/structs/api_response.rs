use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub payload: T,
    pub message: String,
}