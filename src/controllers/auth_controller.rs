use axum::{Json, extract::State, response::IntoResponse};
use sqlx::PgPool;
use crate::services::auth_service;
use crate::models::user::RegisterPayload;
use serde_json::json;

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let success = auth_service::register_user_service(&pool, &payload.name, &payload.email, &payload.mobile,&payload.role,).await;

    if success {
        Json(json!({
            "status": "ok",
            "message": "User registered successfully. Password sent via email."
        }))
        .into_response()
    } else {
        Json(json!({
            "status": "error",
            "message": "Registration failed"
        }))
        .into_response()
    }
}
