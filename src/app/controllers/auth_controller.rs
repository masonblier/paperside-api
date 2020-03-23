use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize,Serialize};

use crate::app::database::DbPool;
use crate::app::errors::ServiceError;
use crate::app::middleware::session_user::SessionUser;
use crate::app::models::{Session, SlimUser, User};
use crate::app::security::verify_password;

/// struct for storing login request data
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestData {
    pub name: String,
    pub password: String,
}

/// DELETE /auth
pub async fn logout(
    id: Identity,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    use crate::schema::sessions::dsl::{token, sessions};
    let conn: &PgConnection = &pool.get().unwrap();
    
    if let Some(session_token) = id.identity() {
        // try deleting the session from the db, dont care if it fails
        let _ = diesel::delete(
            sessions
            .filter(token.eq(&session_token))
        ).execute(conn);
    }

    id.forget();
    HttpResponse::Ok().finish()
}

/// POST /auth
pub async fn login(
    auth_data: web::Json<AuthRequestData>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let auth_data_inner: AuthRequestData = auth_data.into_inner();

    let res = web::block(move || {
        let conn: &PgConnection = &pool.get().unwrap();
        let res = query_login(auth_data_inner, conn);

        match res {
            Ok(user) => {
                let new_session = Session::create(user.id);
                let created_session = query_create_session(new_session, conn).unwrap();
                Ok(created_session.token)
            }
            Err(err) => Err(err)
        }

    }).await;

    match res {
        Ok(token) => {
            id.remember(token);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

/// GET /auth
pub async fn get_me(session_user: SessionUser) -> HttpResponse {
    HttpResponse::Ok().json(session_user)
}


/// Queries User object by given user name and verifies if given password matches stored hash
fn query_login(auth_data: AuthRequestData, conn: &PgConnection) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{name, users};
    let mut items = users
        .filter(name.eq(&auth_data.name))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify_password(&user.passhash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

/// Inserts created Session object into the database
fn query_create_session(session: Session, conn: &PgConnection) -> Result<Session, ServiceError> {
    use crate::schema::sessions::dsl::{sessions};
    
    let inserted_session: Session =
        diesel::insert_into(sessions).values(&session).get_result(conn)?;
    return Ok(inserted_session.into());
}