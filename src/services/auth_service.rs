use sqlx::PgPool;
// use uuid::Uuid;
use rand::Rng;
use crate::utils::mailer;
// use crate::models::user::SignupUser;

pub async fn register_user_service(pool: &PgPool, name: &str, email: &str, mobile: &str,role: &str, ) -> bool {

    let password: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let res = sqlx::query!(
        "INSERT INTO signup ( name, email, mobile, password,role) VALUES ($1, $2, $3, $4,$5)",
        name,
        email,
        mobile,
        password,
        role,
    ).execute(pool).await;

 if res.is_ok() {
 
    if let Err(err) = mailer::send_password_email(email, &password,name,role) {
        println!("Failed to send email: {:?}", err);
       
    }
    true
} else {
    false
}

}
