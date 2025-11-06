use sqlx::PgPool;
use crate::repository::student_repo;
use crate::models::student::Student;
use chrono::Utc;

pub async fn get_students_service(pool: &PgPool) -> Result<Vec<Student>, sqlx::Error> {
    student_repo::get_all_students(pool).await
}

pub async fn add_student_service(
    pool: &PgPool,
    full_name: String,
    email: String,
    course: String,
    age: i32,
    ip_address: Option<String>,
    created_by: String,
    pdf_file: Option<String>,
) -> Result<(), sqlx::Error> {
    let reg_time = Utc::now();
    student_repo::insert_student(
        pool,
        full_name,
        email,
        course,
        age,
        reg_time,
        ip_address,
        created_by,
        pdf_file,
    )
    .await
}
