use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::app::config::DbPool;
use crate::app::errors::ServiceError;
use crate::app::models::{NewUser, SlimUser, User};
use crate::app::security::hash_password;

// RegistrationData represents a json request to register a new account
#[derive(Debug, Deserialize)]
pub struct RegistrationData {
    pub username: String,
    pub password: String,
}

/// POST /register 
pub async fn register_user(
    user_data: web::Json<RegistrationData>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_data = user_data.into_inner();
    let res = web::block(move || {
        query_create_user(
            user_data.username,
            user_data.password,
            pool,
        )
    })
    .await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}


/// Inserts new user row into the database
fn query_create_user(
    username: String,
    password: String,
    pool: web::Data<DbPool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::users;
    let conn: &PgConnection = &pool.get().unwrap();

    // try hashing the password, else return the error that will be converted to ServiceError
    let password: String = hash_password(&password)?;
    dbg!(&password);

    let user = NewUser::from_details(username, password);
    let inserted_user: User =
        diesel::insert_into(users).values(&user).get_result(conn)?;
    dbg!(&inserted_user);
    return Ok(inserted_user.into());
}
