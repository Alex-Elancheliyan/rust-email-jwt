use axum::{
    extract::{State, ConnectInfo, Multipart},
    response::IntoResponse,
    Json,
};
use std::net::SocketAddr;
use tokio::fs;
use sqlx::PgPool;
use crate::services::student_service;

pub async fn get_students(State(pool): State<PgPool>) -> impl IntoResponse {
    match student_service::get_students_service(&pool).await {
        Ok(students) => Json(students).into_response(),
        Err(err) => {
            println!("DB Error: {:?}", err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

pub async fn add_student(
    State(pool): State<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut full_name = String::new();
    let mut email = String::new();
    let mut course = String::new();
    let mut age: i32 = 0;
    let mut created_by_id: i32 = 0;
    let mut pdf_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "full_name" => full_name = field.text().await.unwrap(),
            "email" => email = field.text().await.unwrap(),
            "course" => course = field.text().await.unwrap(),
            "age" => age = field.text().await.unwrap().parse().unwrap_or(0),
            "created_by" => created_by_id = field.text().await.unwrap().parse().unwrap_or(0),
            "pdf_file" => {
                let file_name = field.file_name().unwrap_or("upload.pdf").to_string();
                let file_bytes = field.bytes().await.unwrap();
                let save_dir = "uploads";
                fs::create_dir_all(save_dir).await.unwrap();
                let save_path = format!("{}/{}", save_dir, file_name);
                fs::write(&save_path, &file_bytes).await.unwrap();
                pdf_path = Some(save_path);
            }
            _ => {}
        }
    }

    let ip_address = Some(addr.ip().to_string());
    let created_by = match created_by_id {
        1 => "Admin".to_string(),
        2 => "SuperAdmin".to_string(),
        _ => "Unknown".to_string(),
    };

    match student_service::add_student_service(
        &pool,
        full_name,
        email,
        course,
        age,
        ip_address,
        created_by,
        pdf_path,
    )
    .await
    {
        Ok(_) => (axum::http::StatusCode::OK, "Student added").into_response(),
        Err(err) => {
            println!("Insert Error: {:?}", err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Insert failed").into_response()
        }
    }
}
