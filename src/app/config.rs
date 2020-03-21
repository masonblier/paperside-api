/// length of secure session access tokens
pub const TOKEN_LENGTH: usize = 32;

lazy_static::lazy_static! {
    /// loads SECRET_KEY environment variable for use in password hash validation
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY")
        .expect("\n\n  Cowardly refusing to run without SECRET_KEY=(32-character string) environment variable\n\n");

    // server bind address, default to `127.0.0.1:8080`
    pub static ref BIND_ADDRESS: String = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // app domain, default to `localhost`
    pub static ref APP_DOMAIN: String = std::env::var("DOMAIN")
        .unwrap_or_else(|_| "localhost".to_string());

    // postgres database uri
    pub static ref DATABASE_URI: String = std::env::var("DATABASE_URL")
        .expect("\n\n  DATABASE_URI environment variable required for postgres connection\n\n");
}
