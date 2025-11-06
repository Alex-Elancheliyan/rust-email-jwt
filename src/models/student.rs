use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub course: String,
    pub age: i32,
    pub reg_time: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub created_by: String,
    pub pdf_file: Option<String>,
}
