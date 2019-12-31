// src/auth/routes.rs
// ..
use crate::email::{Email, Contact};
use crate::email_verification_token::{EmailVerificationToken, EmailVerificationTokenMessage};
use chrono::Utc;
use hex;
use serde::Deserialize;

#[post("/invite")]
async fn invite(body: web::Json<EmailVerificationTokenMessage>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    let token = EmailVerificationToken::create(body.clone())?;
    let token_string = hex::encode(token.id);

    Email::new(Contact::new("tore@cloudmaker.dev", "Cloudmaker"))
        .add_recipient(body.email)
        .set_subject("Confirm your email")
        .set_html(format!("Your confirmation code is: {}", &token_string))
        .send()?;

    Ok(HttpResponse::Ok().json(json!({"message": "Verification email sent"})))
}

#[derive(Deserialize)]
struct RegistrationMessage {
    token: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn register(body: web::Json<RegistrationMessage>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    let token_id = hex::decode(body.token)
        .map_err(|e| ApiError::new(403, "Invalid token"))?;
    
    let token = EmailVerificationToken::find(&token_id)
        .map_err(|e| {
            match e.status_code {
                404 => ApiError::new(403, "Invalid token"),
                _ => e,
            }
        })?;

    if token.email != body.email {
        return Err(ApiError::new(403, "Invalid token"));
    }

    if token.expires_at < Utc::now().naive_utc() {
        return Err(ApiError::new(403, "Token expired"));
    }
 
    let user = User::create(UserMessage { email: body.email, password: body.password })?;

    Ok(HttpResponse::Ok().json(json!({"message": "Successfully registered", "user": user})))
}

// ..

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(invite);
    cfg.service(register);
    // ..
}
