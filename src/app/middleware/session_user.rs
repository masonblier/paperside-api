use std::pin::Pin;
use actix_identity::Identity;
use actix_web::{
    dev::Payload, web, Error, FromRequest, HttpRequest
};
use chrono::{Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::Future;

use crate::app::errors::ServiceError;
use crate::app::config::DbPool;
use crate::app::models::{Session, SlimUser, User};

// extend SlimUser type for use as middleware
pub type SessionUser = SlimUser;

/// middleware for getting SlimUser data from cookie identity session
impl FromRequest for SessionUser {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<SessionUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        // get identity session_token from request
        let fut = Identity::from_request(req, pl);
        let pool_fut = web::Data::<DbPool>::from_request(req, pl);

        // get remote ip address from request
        let remote_ip = req.connection_info().remote().map(|s| s.to_string());

        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                let pool = pool_fut.await?;
                let conn: &PgConnection = &pool.get().unwrap();

                let user_result: Result<SessionUser,_> = query_session_user(identity.clone(), conn);
                if user_result.is_ok() {
                    query_update_session(identity, remote_ip, conn);
                }

                return Ok(user_result?);
            };
            Err(ServiceError::Unauthorized.into())
        })
    }
}

/// Queries the database for session row by given token, and if successful, queries corresponding user data
fn query_session_user(session_token: String, conn: &PgConnection) -> Result<SessionUser, ServiceError> {
    use crate::schema::sessions::dsl::{token, sessions};
    use crate::schema::users::dsl::{id, users};
    
    let mut items = sessions
        .filter(token.eq(&session_token))
        .load::<Session>(conn)?;

    if let Some(session) = items.pop() {
        let mut items = users
            .filter(id.eq(&session.user_id))
            .load::<User>(conn)?;
        
        if let Some(user) = items.pop() {
            return Ok(SlimUser::from(user));
        }

    }
    Err(ServiceError::Unauthorized)
}


/// Updates the session record last_accessed_at and accessed_by_client_ip fields
fn query_update_session(session_token: String, accessed_by_ip: Option<String>, conn: &PgConnection) {
    use crate::schema::sessions::dsl::*;

    diesel::update(sessions.find(session_token.clone()))
        .set((
            last_accessed_at.eq(Utc::now()),
            accessed_by_client_ip.eq(accessed_by_ip)
        ))
        .execute(conn)
        .expect(&format!("Unable to update session {}", session_token));
}