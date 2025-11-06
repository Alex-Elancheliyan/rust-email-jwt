use axum::{routing::{get}, Router};
use sqlx::PgPool;
use crate::controllers::student_controller::{get_students, add_student};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/students", get(get_students).post(add_student))
        .with_state(pool)
}
