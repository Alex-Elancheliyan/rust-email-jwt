use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::models::student::Student;

pub async fn get_all_students(pool: &PgPool) -> Result<Vec<Student>, sqlx::Error> {
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(pool)
        .await?;
    Ok(students)
}

pub async fn insert_student(
    pool: &PgPool,
    full_name: String,
    email: String,
    course: String,
    age: i32,
    reg_time: DateTime<Utc>,
    ip_address: Option<String>,
    created_by: String,
    pdf_file: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO students (full_name, email, course, age, reg_time, ip_address, created_by, pdf_file)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        full_name,
        email,
        course,
        age,
        reg_time,
        ip_address,
        created_by,
        pdf_file
    )
    .execute(pool)
    .await?;
    Ok(())
}
