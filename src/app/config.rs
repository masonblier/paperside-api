use std::env;
use rocket::fairing::AdHoc;

/// Debug only secret for JWT encoding & decoding.
#[cfg(debug_assertions)]
const SECRET: &'static str = "g8h4wa+7egV/9dlfY-YLGQJfer/a4eEfreahDwm8+SQwDVXJg=";

/// app configuration
pub struct AppConfig {
    pub jwt_secret: Vec<u8>,
}

impl AppConfig {
    /// Returns rocket ad-hoc fairing to init AppConfig on app start
    pub fn manage() -> AdHoc {
        AdHoc::on_attach("Manage config", |rocket| {
            // Rocket doesn't expose it's own secret_key, so we use our own here.
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            Ok(rocket.manage(AppConfig {
                jwt_secret: secret.into_bytes()
            }))
        })
    }
}
