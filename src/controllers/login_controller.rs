use axum::{Json, extract::State, response::IntoResponse};
use sqlx::PgPool;
use crate::services::login_service;
use crate::models::user::LoginPayload;
use serde_json::json;

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
  
    let token_option = login_service::login_user_service(&pool, &payload.email, &payload.password).await;

    if let Some(token) = token_option {
  
        Json(json!({
            "status": "ok",
            "message": "Login successful",
            "token": token
        }))
        .into_response()
    } else {
   
        Json(json!({
            "status": "error",
            "message": "Invalid credentials"
        }))
        .into_response()
    }
}
