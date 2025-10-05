use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    #[serde(rename = "require2FA")]
    require_2fa: bool,
}

pub async fn signup(Json(_): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
