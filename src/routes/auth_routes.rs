use axum::{routing::post, Router};
use sqlx::PgPool;
use crate::controllers::auth_controller::register_user;
use crate::controllers::login_controller::login_user;

pub fn create_auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(pool)
}
