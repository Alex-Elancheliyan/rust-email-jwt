use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;

pub fn send_password_email(to_email: &str, password: &str,name: &str, role: &str) -> Result<(), Box<dyn std::error::Error>> {

    let email_body = format!(
        "Hello {name},\n\n\
        Your account has been successfully created!\n\n\
        Username: {to_email}\n\
        Password: {password}\n\
        Role: {role}\n\n\
        Please keep these credentials safe.\n\n\
        Best regards,\n\
        Alex E"
    );
    let email = Message::builder().from("alexcheliyan1@gmail.com".parse()?).to(to_email.parse::<Mailbox>()?)
    .subject("Your Account Registration Successfull! ")
    .body(email_body)?;

    let creds = Credentials::new("alexcheliyan1@gmail.com".to_string(), "qzczsqhhujnllhsh".to_string(),              
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;
    println!("Email sent successfully to {}", to_email);

    Ok(())
}