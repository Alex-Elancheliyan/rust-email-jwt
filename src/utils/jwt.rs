use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"4fG7h9KpR2vTqZxB8uYwL1sN6mE0aD3j";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,    
}

pub fn create_jwt(email: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

// encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))?;

    println!("Generated token: {}", token);
    println!("Token length: {}", token.len());

    Ok(token)

}
