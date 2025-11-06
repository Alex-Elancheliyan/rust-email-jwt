use sqlx::PgPool;
use crate::utils::jwt;

pub async fn login_user_service(pool: &PgPool, email: &str, password: &str) -> Option<String> {
    let result = sqlx::query!(
        "SELECT email, password, role FROM signup WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(record)) => {
            if record.password == password {
                println!("Login successful for user: {}", email);

                let token = jwt::create_jwt(email, &record.role).unwrap();

                let insert_res = sqlx::query!(
                    "INSERT INTO login_user (username, password, role, token) VALUES ($1, $2, $3, $4)",
                    record.email,
                    record.password,
                    record.role,
                    token
                ).execute(pool).await;

                if insert_res.is_ok() {
                    println!("User inserted into login_user table with Token.");
                    Some(token)
                } else {
                    println!("Failed to insert user into login_user table.");
                    None
                }
            } else {
                println!("Invalid password for {}", email);
                None
            }
        }
        Ok(None) => {
            println!("No user found with email: {}", email);
            None
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            None
        }
    }
}
