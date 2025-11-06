use serde::{Serialize, Deserialize};
use sqlx::FromRow;
// use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SignupUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub role: String,
}
#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

