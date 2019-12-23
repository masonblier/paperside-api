use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
use jsonwebtoken as jwt;

use crate::schema::{sessions};
use crate::app::models::User;

// sessions model
#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(User)]
#[table_name="sessions"]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub hashed_access_token: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub accessed_by_client_ip: Option<String>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="sessions"]
pub struct NewSession {
    pub user_id: i32,
    pub hashed_access_token: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub accessed_by_client_ip: Option<String>,
}


// structs for holding session and user data associated with a request
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionAuth {
    pub exp: i64,
    pub session_id: i64,
    pub access_token: String,
}
#[derive(Clone)]
pub struct SessionUser {
    pub auth: Option<SessionAuth>,
    pub user: Option<User>,
}

impl SessionAuth {
    /// Encodes jwt token from auth struct
    pub fn to_jwt(&self, secret: &[u8]) -> String {
        jwt::encode(&jwt::Header::default(), self, secret).expect("JWT encode failed")
    }

    /// Decodes jwt token from encoded string
    pub fn from_jwt(encoded_jwt: &str, secret: &[u8]) -> Option<SessionAuth> {
        jwt::decode(encoded_jwt, secret, &jwt::Validation::new(jwt::Algorithm::HS256))
            .map_err(|err| {
                eprintln!("Auth decode error: {:?}", err);
            })
            .ok()
            .map(|token_data| token_data.claims)
    }
}
